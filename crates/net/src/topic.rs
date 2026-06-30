use libp2p::gossipsub::{IdentTopic, TopicHash};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIter, EnumString, IntoEnumIterator};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TopicError {
    #[error("unrecognized topic: {0}")]
    UnrecognizedTopic(String),
}

/// TODO :: We no longer uses topics. Clean this up
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    AsRefStr,
    Display,
    EnumIter,
    EnumString,
)]
#[strum(serialize_all = "lowercase")]
pub enum Topic {
    Melon,
}

impl Topic {
    pub fn all() -> Vec<Topic> {
        Self::iter().collect()
    }
}

impl From<Topic> for IdentTopic {
    fn from(topic: Topic) -> Self {
        IdentTopic::new(topic.as_ref())
    }
}

impl TryFrom<IdentTopic> for Topic {
    type Error = TopicError;

    fn try_from(value: IdentTopic) -> Result<Self, Self::Error> {
        Self::try_from(value.hash())
    }
}

impl From<Topic> for TopicHash {
    fn from(topic: Topic) -> Self {
        TopicHash::from_raw(topic.as_ref())
    }
}

impl TryFrom<TopicHash> for Topic {
    type Error = TopicError;

    fn try_from(value: TopicHash) -> Result<Self, Self::Error> {
        use std::str::FromStr;
        Self::from_str(value.as_str()).map_err(|_| TopicError::UnrecognizedTopic(value.to_string()))
    }
}
