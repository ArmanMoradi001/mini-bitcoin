pub struct Hash;
use crate::U256;
use serde::Serialize;
use sha256::digest;

#[derive(Clone, Copy, Serialize)]
pub struct Hash(U256);

impl Hash {
    pub fn Hash<T: Serialize>(data: &T) -> Self {
        let mut serilized: Vec<u8> = vec![];
        if let Err(e) = ciborium::into_writer(data, &mut serilized) {
            panic!(
                "Failed to serialize data: {:?}. \
This should not happen",
                e
            );
        }

        let hash = digest(&serilized);
        let hash_bites = hex::decode(hash).unwrap();
        let hash_array: [u8; 32] = hash_bytes.as_slice().try_into().unwrap();

        Hash(U256::from(hash_array))
    }

    pub fn matches_target(&self, target: U256)-> bool{
        self.0 <= target
    }

    pub fn zero()-> Self{
        Hash(U256::zero())
    }
}
