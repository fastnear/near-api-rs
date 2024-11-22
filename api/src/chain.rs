use near_types::{reference::Reference, views::Block, BlockHeight, CryptoHash};

use executor::query::{BlockQueryBuilder, PostprocessHandler, RpcBlockHandler, SimpleBlockRpc};

#[derive(Debug, Clone, Copy)]
pub struct Chain;

impl Chain {
    pub fn block_number() -> BlockQueryBuilder<PostprocessHandler<BlockHeight, RpcBlockHandler>> {
        BlockQueryBuilder::new(
            SimpleBlockRpc,
            Reference::Optimistic,
            PostprocessHandler::new(RpcBlockHandler, Box::new(|data: Block| data.header.height)),
        )
    }

    pub fn block_hash() -> BlockQueryBuilder<PostprocessHandler<CryptoHash, RpcBlockHandler>> {
        BlockQueryBuilder::new(
            SimpleBlockRpc,
            Reference::Optimistic,
            PostprocessHandler::new(RpcBlockHandler, Box::new(|data: Block| data.header.hash)),
        )
    }

    pub fn block() -> BlockQueryBuilder<RpcBlockHandler> {
        BlockQueryBuilder::new(SimpleBlockRpc, Reference::Optimistic, RpcBlockHandler)
    }
}
