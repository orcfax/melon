mod feed_id;
pub use feed_id::FeedId;

mod rational;
use melon_core::domain;
pub use rational::Rational;

use minicbor::{Decode, Decoder, Encode, Encoder, decode, encode};
use std::time::Duration;

impl domain::Domain for Statement {
    const DST: &'static [u8] = "MELON_APP_V1".as_bytes();

    const KIND: domain::Kind = domain::Kind::App;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Statement<T = Rational> {
    feed_id: FeedId,
    created_at: Duration,
    body: T,
}

impl<T> Statement<T> {
    pub fn feed_id(&self) -> &FeedId {
        &self.feed_id
    }
    pub fn created_at(&self) -> Duration {
        self.created_at
    }
    pub fn body(&self) -> &T {
        &self.body
    }
}

impl<T> Statement<T>
where
    T: Clone + for<'b> Decode<'b, ()> + Encode<()>,
{
    pub fn new(feed_id: FeedId, created_at: Duration, body: T) -> Self {
        Self {
            feed_id,
            created_at,
            body,
        }
    }
}

impl<C, T: Encode<C>> Encode<C> for Statement<T> {
    fn encode<W: encode::Write>(
        &self,
        e: &mut Encoder<W>,
        ctx: &mut C,
    ) -> Result<(), encode::Error<W::Error>> {
        e.tag(minicbor::data::Tag::new(121))?
            .begin_array()?
            .bytes(self.feed_id.as_ref())?
            .encode(self.created_at.as_millis() as u64)?
            .encode_with(&self.body, ctx)?
            .end()?;
        Ok(())
    }
}

impl<'b, C, T: Decode<'b, C>> Decode<'b, C> for Statement<T> {
    fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, decode::Error> {
        if d.tag()? != minicbor::data::Tag::new(121) {
            return Err(decode::Error::message("Expected tag 121 for Statement"));
        }
        d.array()?;
        let feed_id = FeedId::from(d.bytes()?);
        let created_at = Duration::from_millis(d.decode()?);
        let body: T = d.decode_with(ctx)?;
        if d.datatype()? == minicbor::data::Type::Break {
            d.skip()?;
        }

        Ok(Statement {
            created_at,
            feed_id,
            body,
        })
    }
}
