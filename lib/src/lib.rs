use uint::construct_uint;

construct_uint!{

    pub struct U256(4);
}

pub mod crypto;
pub mod util;
pub mod sha256;
pub mod types;