# Dfinity Self Describing Standard

## Overview
   
   The ERC1820 registry contract provides a common contract interface registration/query service, and Dfinity also needs such a service.
   Dfinity currently does not have such a service.

## Problems to be solved
   No one has the motivation to establish such a service, because the call to the contract does not require the caller to pay gas (Cycles)  fees, but the contract publisher provides gas (Cycles) in the contract.
  
## Solution
  
   Canister should implement the interface self-describing.

   Dfinity can solve the problem solved by ERC1820 through the interface self-description, and achieve the self-description of the canister interface by implementing supportedInterface(text) -> (bool).

## Example 
   You can test it with the following command :
   
   ```
      dfx deploy
      
      dfx canister call standard supportedInterface '("test:(text)->(text)query")'
   ```

## About us

   We are from ICP Deland team. 

   We are building a decentralized exchange based on Dfinity with   open order book protocol.

   Offcial Website : [https://deland.one](https://deland.one)

## Reference

To learn more before you start working with DfinitySelfDescribingStandard, see the following documentation available online:

- [Quick Start](https://sdk.dfinity.org/docs/quickstart/quickstart-intro.html)
- [SDK Developer Tools](https://sdk.dfinity.org/docs/developers-guide/sdk-guide.html)
- [Motoko Programming Language Guide](https://sdk.dfinity.org/docs/language-guide/motoko.html)
- [Motoko Language Quick Reference](https://sdk.dfinity.org/docs/language-guide/language-manual.html)
- [Introduction to working with Rust](https://sdk.dfinity.org/docs/rust-guide/rust-intro.html)


 
