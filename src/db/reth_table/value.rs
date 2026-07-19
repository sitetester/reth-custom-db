use reth_ethereum::codec::{Compress, Decompress, DecompressError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct EntityValue(pub Vec<u8>);

impl AsRef<[u8]> for EntityValue {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Compress for EntityValue {
    type Compressed = Vec<u8>;

    fn uncompressable_ref(&self) -> Option<&[u8]> {
        Some(&self.0)
    }

    fn compress(self) -> Self::Compressed {
        self.0
    }

    fn compress_to_buf<B: bytes::BufMut + AsMut<[u8]>>(&self, buf: &mut B) {
        buf.put_slice(&self.0);
    }
}

impl Decompress for EntityValue {
    fn decompress(value: &[u8]) -> Result<Self, DecompressError> {
        Ok(Self(value.to_vec()))
    }

    fn decompress_owned(value: Vec<u8>) -> Result<Self, DecompressError> {
        Ok(Self(value))
    }
}
