use futures::StreamExt;
use libp2p::{
    gossipsub::{self, MessageId},
    kad::{self},
    swarm::SwarmEvent,
};
use melon_bus::{Bus, Event};
use melon_core::{
    Bytes, Endorsement, Id, Manifest, Members,
    cbor::ToCbor,
    node,
    time::{self, Trio},
};
use melon_crypto::ed25519;
use std::{
    collections::BTreeMap,
    sync::Arc,
    time::{Duration, Instant},
};

use tracing::{info, warn};

use crate::{Addressbook, Config, Message, Topic};

use crate::{BehaviourEvent, Error, Swarm, addr::extract_peer_id};

pub struct Service {
    own_id: node::Id,
    bus: Arc<Bus>,
    swarm: Swarm,
    /// Time of the validator set
    time: Trio,
    proposers: Addressbook,
    pending_queries: BTreeMap<Id, Instant>,
}

impl Service {
    pub async fn new(config: Config, bus: Arc<Bus>) -> Result<Self, Error> {
        let keypair = crate::keypair::from_signing_key(&config.key);
        let own_id = node::Id::from(ed25519::VerifyingKey::from(config.key.clone()));
        let mut swarm = Swarm::new(keypair)?;

        if let Some(listen_on) = config.listen_on {
            swarm
                .0
                .listen_on(listen_on.clone())
                .map_err(|err| Error::Init(format!("Listen :: {}", err)))?;
        }
        // Bootstrap
        if !config.bootstrap.is_empty() {
            for addr in config.bootstrap.iter() {
                let peer_id =
                    extract_peer_id(addr).unwrap_or_else(|_| panic!("No peer id in {}", addr));
                swarm
                    .0
                    .behaviour_mut()
                    .kademlia
                    .add_address(&peer_id, addr.clone());
            }
            swarm
                .0
                .behaviour_mut()
                .kademlia
                .bootstrap()
                .map_err(|err| Error::Init(format!("Bootstrap :: {}", err)))?;
        }
        // Subscribe to topics
        for topic in Topic::all() {
            if let Err(e) = swarm.0.behaviour_mut().gossip.subscribe(&topic.into()) {
                warn!("Subscription failed: {:?}", e);
            }
        }
        let time = config.time;
        let addressbook = Addressbook::from(config.members);

        Ok(Self {
            swarm,
            bus,
            own_id,
            proposers: addressbook,
            time,
            pending_queries: BTreeMap::new(),
        })
    }

    pub async fn run(mut self) -> Result<(), Error> {
        let mut rx = self.bus.subscribe();
        loop {
            tokio::select! {
                event = rx.recv() => {
                    match event {
                        Ok(event) => self.on_bus(event)?,
                        Err(err) => {
                            eprintln!("{:?}", err);
                        }
                    }
                },
                event = self.swarm.0.select_next_some() => self.on_swarm(event).await?,
            }
        }
    }

    pub fn set_proposers(&mut self, addressbook: Addressbook) {
        self.proposers = addressbook;
    }

    pub fn epoch(&mut self, epoch: Trio) {
        self.time = epoch;
    }

    // Handlers

    fn on_bus(&mut self, event: Event) -> Result<(), Error> {
        // FIXME :: Replumb this
        match event {
            Event::Proposed { endorsement, data } => self.on_proposed(endorsement, data)?,
            Event::Endorsed(endorsement) => self.on_endorsed(endorsement)?,
            Event::Missed { id } => self.on_missed(id)?,
            Event::Epoch { time, members } => self.on_epoch(time, members)?,
            Event::Period { time, manifest } => self.on_period(time, manifest)?,
            Event::BadMessage {
                endorsement,
                severity,
            } => self.on_bad_message(endorsement, severity)?,
            _ => {}
        };
        Ok(())
    }

    async fn on_swarm(&mut self, event: SwarmEvent<BehaviourEvent>) -> Result<(), Error> {
        match event {
            SwarmEvent::NewListenAddr { address, .. } => {
                tracing::info!(
                    "Node is ready. Reach me at: {}/p2p/{}",
                    address,
                    self.swarm.0.local_peer_id()
                );
            }
            SwarmEvent::Behaviour(BehaviourEvent::Gossip(gossipsub::Event::Message {
                propagation_source: _,
                message_id: _,
                message,
            })) => {
                let Some(source) = message.source else {
                    warn!("Message without source");
                    return Ok(());
                };
                let Ok(node_id) = node::Id::try_from(source) else {
                    warn!("Invalid node id");
                    return Ok(());
                };
                let Ok(msg) = minicbor::decode::<Message>(&message.data) else {
                    warn!("Failed to decode message");
                    return Ok(());
                };
                let Some(bls_key) = self.proposers.get_bls(&node_id) else {
                    warn!("Unknown node: {node_id:?}");
                    return Ok(());
                };
                if msg.epoch != self.time {
                    // FIXME :: Allow buffer for clockdrift
                    warn!("Stale epoch from {node_id:?}");
                    return Ok(());
                }
                let event = msg.into_event(bls_key.clone());
                self.bus.publish(event);
            }
            SwarmEvent::Behaviour(BehaviourEvent::Kademlia(
                kad::Event::OutboundQueryProgressed { result, .. },
            )) => {
                if let kad::QueryResult::GetRecord(result) = result {
                    match result {
                        Ok(kad::GetRecordOk::FoundRecord(record)) => {
                            let key = record.record.key.as_ref();
                            // FIXME :: shouldn't unwrap_or.
                            let Ok(id) = key.try_into() else {
                                warn!("Unknown kademlia key!");
                                return Ok(());
                            };
                            let Some(_query) = self.pending_queries.remove(&id) else {
                                warn!("No known query for the id");
                                return Ok(());
                            };
                            // FIXME !!
                            // let _ =
                            //     self.bus
                            //         .publish(bus::Event::Seq(seq::event::Found {
                            //             id,
                            //             data: record.record.value,
                            //         }));
                        }
                        Ok(_) => warn!("The other ok case"),
                        Err(_err) => warn!("Kademlia failed to find"),
                    }
                }
            }
            SwarmEvent::ConnectionEstablished {
                peer_id, endpoint, ..
            } => {
                info!("Peer {} connected", peer_id);
                self.swarm
                    .0
                    .behaviour_mut()
                    .kademlia
                    .add_address(&peer_id, endpoint.get_remote_address().clone());
                // if let Ok(node_id) = node::Id::try_from(peer_id) {
                //     let mut s = self.state.write().await;
                //     if !s.peers.contains(&node_id) {
                //         s.peers.push(node_id);
                //     }
                // }
            }
            SwarmEvent::ConnectionClosed { peer_id, .. } => {
                info!("Peer {} disconnected", peer_id);
                // if let Ok(node_id) = node::Id::try_from(peer_id) {
                //     let mut s = self.state.write().await;
                //     s.peers.retain(|id| id != &node_id);
                // }
            }
            _ => {}
        };
        Ok(())
    }

    // Actions

    /// Publish to Network
    pub fn to_net(&mut self, message: Message) -> Result<MessageId, Error> {
        let id = self
            .swarm
            .0
            .behaviour_mut()
            .gossip
            .publish(Topic::Melon, message.to_cbor())
            .map_err(|err| Error::Gossip(err.to_string()))?;
        Ok(id)
    }

    /// Get record from Kademlia
    pub fn get(&mut self, id: Id) -> Result<(), Error> {
        let now = Instant::now();
        let last_req = self.pending_queries.get(&id).cloned();
        if last_req.is_none_or(|t| now.duration_since(t) > Duration::from_millis(500)) {
            let _ = self
                .swarm
                .0
                .behaviour_mut()
                .kademlia
                .get_record(kad::RecordKey::new(id.as_ref()));
            self.pending_queries.insert(id, now);
        };
        Ok(())
    }

    fn on_proposed(&mut self, endorsement: Endorsement, data: Bytes) -> Result<(), Error> {
        tracing::info!("Proposed : {}", endorsement.id.to_string());
        let Some(id) = self.proposers.get_node(&endorsement.key) else {
            tracing::warn!("Rouge proposer {:?}", &endorsement.key);
            return Ok(());
        };
        if id != &self.own_id {
            return Ok(());
        }
        self.to_net(Message::from_proposal(self.time.clone(), endorsement, data))?;
        Ok(())
    }

    fn on_endorsed(&mut self, endorsement: Endorsement) -> Result<(), Error> {
        tracing::info!("Endorsed : {}", endorsement.id.to_string());
        let Some(id) = self.proposers.get_node(&endorsement.key) else {
            tracing::warn!("Rouge proposer {:?}", &endorsement.key);
            return Ok(());
        };
        if id != &self.own_id {
            return Ok(());
        }
        self.to_net(Message::from_endorsement(self.time.clone(), endorsement))?;
        Ok(())
    }

    fn on_missed(&self, _id: Id) -> Result<(), Error> {
        todo!("missed");
    }

    fn on_period(&self, _time: time::Duo, _manifest: Manifest) -> Result<(), Error> {
        todo!("period");
    }

    fn on_epoch(&self, _time: Trio, _members: Members) -> Result<(), Error> {
        todo!("epoch")
    }

    fn on_bad_message(&self, endorsement: Endorsement, severity: u8) -> Result<(), Error> {
        tracing::info!(?endorsement, "received");
        tracing::info!(?severity, "severity");
        todo!("message")
    }
}
