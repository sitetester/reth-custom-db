use reth_ethereum::provider::db::{
    DatabaseError,
    table::{Decode, Encode, IntoVec},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct EntityKey(pub Vec<u8>);

impl AsRef<[u8]> for EntityKey {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl IntoVec for EntityKey {
    fn into_vec(self) -> Vec<u8> {
        self.0
    }
}

impl Encode for EntityKey {
    type Encoded = Vec<u8>;

    fn encode(self) -> Self::Encoded {
        self.0
    }
}

impl Decode for EntityKey {
    fn decode(value: &[u8]) -> Result<Self, DatabaseError> {
        Ok(Self(value.to_vec()))
    }
}
