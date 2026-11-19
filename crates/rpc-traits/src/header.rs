use alloy_consensus::Sealable;
use alloy_primitives::U256;
use reth_primitives_traits::SealedHeader;

/// Conversion trait for obtaining RPC header from a consensus header.
pub trait FromConsensusHeader<T> {
    /// Takes a consensus header and converts it into `self`.
    ///
    /// `block_size` is present for full block responses and absent for header-only responses.
    fn from_consensus_header(header: SealedHeader<T>, block_size: Option<usize>) -> Self;

    /// Same as [`from_consensus_header`](Self::from_consensus_header) but also attaches the
    /// block's total difficulty to the RPC header.
    ///
    /// Defaults to ignoring `total_difficulty` (falling back to
    /// [`from_consensus_header`](Self::from_consensus_header)) so existing implementors are
    /// unaffected; implementors that carry total difficulty override this.
    fn from_consensus_header_with_td(
        header: SealedHeader<T>,
        block_size: Option<usize>,
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
    fn from_consensus_header(header: SealedHeader<T>, block_size: Option<usize>) -> Self {
        Self::from_consensus(header.into(), None, block_size.map(U256::from))
    }

    fn from_consensus_header_with_td(
        header: SealedHeader<T>,
        block_size: Option<usize>,
        total_difficulty: Option<U256>,
    ) -> Self {
        Self::from_consensus(header.into(), total_difficulty, block_size.map(U256::from))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_consensus::Header;

    #[test]
    fn block_size_is_only_set_when_provided() {
        let header = || SealedHeader::new_unhashed(Header::default());

        let rpc_header = alloy_rpc_types_eth::Header::from_consensus_header(header(), None);
        assert_eq!(rpc_header.size, None);

        let rpc_header = alloy_rpc_types_eth::Header::from_consensus_header(header(), Some(42));
        assert_eq!(rpc_header.size, Some(U256::from(42)));
    }

    #[test]
    fn total_difficulty_is_preserved_with_optional_block_size() {
        for block_size in [None, Some(42)] {
            for total_difficulty in [None, Some(U256::ZERO), Some(U256::from(123))] {
                let header = SealedHeader::new_unhashed(Header::default());
                let expected_hash = header.hash();
                let rpc_header = alloy_rpc_types_eth::Header::from_consensus_header_with_td(
                    header,
                    block_size,
                    total_difficulty,
                );
                assert_eq!(rpc_header.hash, expected_hash);
                assert_eq!(rpc_header.size, block_size.map(U256::from));
                assert_eq!(rpc_header.total_difficulty, total_difficulty);
            }
        }
    }
}
