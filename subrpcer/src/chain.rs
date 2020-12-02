// --- crates.io ---
use serde::Serialize;
use serde_json::{json, Value};
// --- subrpcer ---
use crate::rpc;

pub fn get_block_hash(block_number: impl Serialize) -> Value {
	rpc("chain_getBlockHash", json!([block_number]), 1)
}