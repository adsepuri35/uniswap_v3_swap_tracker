use crate::PoolInfo;
use crate::TokenInfo;

#[derive(Debug, Clone)]
pub enum BackendUpdate {
    PoolUpdated(PoolInfo),
    TokenUpdated(TokenInfo),
    ChainStats {
        eth_swaps: usize,
        base_swaps: usize,
        arb_swaps: usize,
    },
}