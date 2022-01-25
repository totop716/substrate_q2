use jsonrpc_core::Result;
use jsonrpc_derive::rpc;
use tokio::clock::now;



#[rpc]
pub trait Gettimetrait {
	#[rpc(name = "get_blocktime")]
	fn current_block_time(&self) -> Result<u64>;

}

/// A struct that implements the `Blocktime`
pub struct Blocktime;

impl Gettimetrait for Blocktime{
	fn current_block_time(&self) -> Result<u64> {
        let current_time = now();
		Ok(7)
	}
}

