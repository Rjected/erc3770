# erc3770

Implements a helper method for [ERC-3770](https://github.com/ethereum/ERCs/blob/e83c0862bce4ae2b53db5ea4ce26799b1e3cfe20/ERCS/erc-3770.md)

## Example
```rust
use erc3770::create_address;
use alloy_chains::NamedChain;
use alloy_primitives::Address;

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
```

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in these crates by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
</sub>
