# Dfinity Self Describing Standard

<strong>Please do not use this standard</strong>

Motoko canister support default method `__get_candid_interface_tmp_hack` to get did, if rust canister implement this method ,Developer can check canister's interface through `__get_candid_interface_tmp_hack`.

```RUST
candid::export_service!();

#[query(name = "__get_candid_interface_tmp_hack")]
#[candid_method(query, rename = "__get_candid_interface_tmp_hack")]
fn __export_did_tmp_() -> String {
    __export_service()
} 
```



## Overview

   The ERC1820 registry contract provides a common contract interface registration/query service, and Dfinity also needs such a service.
   Dfinity currently does not have such a service.

## Problems to be solved
   No one has the motivation to establish such a service, because the call to the contract does not require the caller to pay gas (Cycles)  fees, but the contract publisher provides gas (Cycles) in the contract.

## Solution

   Canister should implement the interface self-describing.

   Dfinity can solve the problem solved by ERC1820 through the interface self-description, and achieve the self-description of the canister interface by implementing supportedInterface(text) -> (bool).

## How to test 
   You can test it with the following command :

   ```
      dfx deploy
      
      dfx canister call standard supportedInterface '("test:(text)->(text)query")'
   ```

## About us

   We are from Deland-Labs team. 

   We are building a decentralized exchange based on Dfinity with Open Order Protocol.

   Offcial Website : [https://deland.one](https://deland.one)

## References

To learn more before you start working with Dfinity Self Describing Standard, see the following documentation available online:

- [Quick Start](https://sdk.dfinity.org/docs/quickstart/quickstart-intro.html)
- [SDK Developer Tools](https://sdk.dfinity.org/docs/developers-guide/sdk-guide.html)
- [Motoko Programming Language Guide](https://sdk.dfinity.org/docs/language-guide/motoko.html)
- [Motoko Language Quick Reference](https://sdk.dfinity.org/docs/language-guide/language-manual.html)
- [Introduction to working with Rust](https://sdk.dfinity.org/docs/rust-guide/rust-intro.html)



