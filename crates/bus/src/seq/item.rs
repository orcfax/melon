use std::ops::Deref;

use minicbor::{Decode, Encode};

use crate::statement::{Rational, Statement};

/// We simplify here. We consider only Statements of Rational body.
#[derive(Debug, Clone, Encode, Decode)]
#[repr(transparent)]
#[cbor(transparent)]
pub struct Item(#[n(0)] pub Statement<Rational>);

impl Deref for Item {
    type Target = Statement<Rational>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
