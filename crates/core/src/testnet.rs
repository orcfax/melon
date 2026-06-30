use crate::{Config, Manifest, Member, Members, Register, config, node};

pub struct Testnet(Vec<(node::Secrets, Register)>);

impl Testnet {
    pub fn generate(n: usize, base_port: u16) -> Self {
        let inner: Vec<(node::Secrets, Register)> = (0..n)
            .map(|i| {
                let secrets = node::Secrets::from_seed(i);
                let registration = Register::make(
                    &secrets.member.as_ref().unwrap(),
                    &secrets.net,
                    format!("/ip4/127.0.0.1/tcp/{}", base_port + i as u16),
                );
                (secrets, registration)
            })
            .collect();

        Self(inner)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn bootstrap_addr(&self) -> (String, node::Id) {
        let rendezvous = &self.0[0].1;
        let addr = rendezvous.addr().to_string();
        let id = rendezvous.id().clone();
        (addr, id)
    }

    pub fn members(&self) -> Members {
        self.0
            .iter()
            .map(|(_, r)| Member::new(r.id().clone(), r.key().clone()))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }

    pub fn manifest(&self) -> Manifest {
        Manifest::genesis(self.members(), [0u8; 32])
    }

    pub fn config(&self, i: usize) -> Option<Config> {
        let (secrets, reg) = self.0.get(i)?;
        let c = Config {
            secrets: secrets.clone(),
            manifest: self.manifest(),
            addr: reg.addr().to_string(),
            bootstrap: if i == 0 {
                vec![]
            } else {
                vec![self.bootstrap_addr()]
            },
        };
        Some(c)
    }

    pub fn configs(&self) -> impl Iterator<Item = Config> + '_ {
        (0..self.len()).map(|i| self.config(i).unwrap())
    }

    pub fn write(&self, output_dir: &str) -> anyhow::Result<()> {
        std::fs::create_dir_all(output_dir)?;
        for (i, config) in self.configs().enumerate() {
            config::write(&config, config::default_path(output_dir, i))?;
        }
        Ok(())
    }
}
