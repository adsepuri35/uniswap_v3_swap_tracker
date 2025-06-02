use crate::PoolInfo;
use crate::TokenInfo;

//send over mspc channel to update UI
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