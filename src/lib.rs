use std::time::{SystemTime, UNIX_EPOCH};
use ring::digest::{Context, SHA256};

pub mod block;
pub mod blockchain;
pub mod proof_of_work;
pub mod transaction;

pub fn current_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as i64
}

pub fn sha256_digest(data: &[u8]) -> Vec<u8> {
    let mut context = Context::new(&SHA256);
    context.update(data);
    let digest = context.finish();
    digest.as_ref().to_vec()
}
