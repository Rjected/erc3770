//! Implements [ERC-3770](https://github.com/ethereum/ERCs/blob/e83c0862bce4ae2b53db5ea4ce26799b1e3cfe20/ERCS/erc-3770.md).
use alloy_chains::NamedChain;
use alloy_primitives::Address;

use core::fmt::{Display, Formatter};
use core::format_args;

/// Create an ERC-3770 chain-specific address.
///
/// This will construct an address format based on the following semantics:
/// * `shortName` is mandatory and MUST be a valid chain short name from <https://github.com/ethereum-lists/chains>
/// * `address` is mandatory and MUST be a
///   [ERC-55](https://github.com/ethereum/ERCs/blob/e83c0862bce4ae2b53db5ea4ce26799b1e3cfe20/ERCS/erc-55.md)
///   compatible hexadecimal address
pub fn create_address(chain: NamedChain, address: Address) -> Result<String, ChainListError> {
    Ok(format!("{}:{address}", chain.chain_short_name()?))
}

/// An error indicating that the [`NamedChain`] does not have a valid chain list name from
/// <https://github.com/ethereum-lists/chains>
#[derive(Debug)]
pub struct ChainListError(NamedChain);

impl Display for ChainListError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("chain {} does not have a valid chain list name from https://github.com/ethereum-lists/chains", self.0))
    }
}

// TODO: when <https://github.com/rust-lang/rust/issues/103765>
// impl core::error::Error for ChainListError {}

// TODO: remove trait if NamedChain ends up supporting method for short names
/// A trait for short chain names
pub trait ChainListName {
    /// The error type if the type fails to convert to a valid short name.
    type Error;

    /// Returns a valid chain short name from <https://github.com/ethereum-lists/chains>
    fn chain_short_name(&self) -> Result<&str, Self::Error>;
}

impl ChainListName for NamedChain {
    type Error = ChainListError;

    /// Returns a valid chain short name from <https://github.com/ethereum-lists/chains>
    #[inline]
    fn chain_short_name(&self) -> Result<&str, Self::Error> {
        // TODO: add more chains, maybe macro for enum type like strum
        let name = match self {
            Self::Mainnet => "eth",
            Self::Ropsten => "rop",
            Self::Rinkeby => "rin",
            Self::Goerli => "gor",
            Self::Holesky => "holesky",
            Self::Sepolia => "sep",
            Self::Base => "base",
            Self::Arbitrum => "arb1",
            Self::ArbitrumNova => "arb-nova",
            Self::Optimism => "oeth",
            Self::BinanceSmartChain => "bsc",
            Self::Polygon => "matic",
            Self::PolygonMumbai => "maticmum",
            // for any other type return the error
            chain => return Err(ChainListError(*chain)),
        };

        Ok(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_eth_poly_base_fmt() {
        let address = Address::repeat_byte(0x42);
        assert_eq!(
            &create_address(NamedChain::Mainnet, address).unwrap(),
            "eth:0x4242424242424242424242424242424242424242"
        );
        assert_eq!(
            &create_address(NamedChain::Polygon, address).unwrap(),
            "matic:0x4242424242424242424242424242424242424242"
        );
        assert_eq!(
            &create_address(NamedChain::Base, address).unwrap(),
            "base:0x4242424242424242424242424242424242424242"
        );
    }
}
