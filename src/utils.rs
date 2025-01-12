use ethereum_types::H256;
use keccak_hash::keccak;

pub(crate) fn hash(bytes: &[u8]) -> H256 {
    H256::from(keccak(bytes).0)
}

pub(crate) fn update_val_if_some<T>(target: &mut T, opt: Option<T>) {
    if let Some(new_val) = opt {
        *target = new_val;
    }
}
