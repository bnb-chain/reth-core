use alloy_consensus::Sealable;
use alloy_primitives::U256;
use reth_primitives_traits::SealedHeader;

/// Conversion trait for obtaining RPC header from a consensus header.
pub trait FromConsensusHeader<T> {
    /// Takes a consensus header and converts it into `self`.
    fn from_consensus_header(header: SealedHeader<T>, block_size: usize) -> Self;

    /// Same as [`from_consensus_header`](Self::from_consensus_header) but also attaches the
    /// block's total difficulty to the RPC header.
    ///
    /// Defaults to ignoring `total_difficulty` (falling back to
    /// [`from_consensus_header`](Self::from_consensus_header)) so existing implementors are
    /// unaffected; implementors that carry total difficulty override this.
    fn from_consensus_header_with_td(
        header: SealedHeader<T>,
        block_size: usize,
        total_difficulty: Option<U256>,
    ) -> Self
    where
        Self: Sized,
    {
        let _ = total_difficulty;
        Self::from_consensus_header(header, block_size)
    }
}

impl<T: Sealable> FromConsensusHeader<T> for alloy_rpc_types_eth::Header<T> {
    fn from_consensus_header(header: SealedHeader<T>, block_size: usize) -> Self {
        Self::from_consensus(header.into(), None, Some(U256::from(block_size)))
    }

    fn from_consensus_header_with_td(
        header: SealedHeader<T>,
        block_size: usize,
        total_difficulty: Option<U256>,
    ) -> Self {
        Self::from_consensus(header.into(), total_difficulty, Some(U256::from(block_size)))
    }
}
