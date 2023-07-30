# <h1 align="center"> soliris </h1>

<p align="center">
    <img src="./.github/assets/COVER.PNG" style="border-radius:1%" alt="">
</p>

<p align="center">
    Solidity code scanner and optimizer.
</p>

Cover by [DALL-E](https://openai.com/dall-e-2/).

## Motivations

We want to help Solidity developers write better smart contracts.
Various tools have been developed with this goal in mind: [Foundry](https://github.com/foundry-rs/foundry), [Slither](https://github.com/crytic/slither), [Heimdall](https://github.com/Jon-Becker/heimdall-rs) and so many more.

The release of the [alloy-rs core libraries](https://github.com/alloy-rs/core) gave us the opportunity to experiment with the Solidity AST in Rust, and we immediately knew that we could build something upon it.

We opted for a project able to evolve and scale. A software able to perform various scans on Solidity code - using the AST - and report valuable information to the developer.

Soliris does not aim to be specialized in one single topic and should preferably hold scanners with different purposes: improving code syntax, giving more context about the contract's state, propose memory improvements and more.
But we also acknowledge the limitations of the solution: it will never replace a compiler, nor a dedicated security tool such as Slither.
It's more of a serious test demonstrating the possibilities of [syn-solidity](https://github.com/alloy-rs/core/tree/main/crates/syn-solidity).

## Development Status

Today, `soliris` **is not production-ready** and might report false information.

**Specifically, we have a few optimisation scans in mind which might help you refactor pieces of your code. Don't use them in production. They might break a lot of things in your smart contracts.**

Below is a list of the scanners implementation statuses:

|    Name       	     | Goal                                                                                                         	 | Status 	 |
|:-------------------:|----------------------------------------------------------------------------------------------------------------|:--------:|
| Missing Comments 	  | Reports missing comments in your code.                                                                       	 |  ✅   	   |
| Mutable Functions 	 | Reports functions able to mutate your contract's state.                                                      	 |  ✅   	   |
| Mutable Variables 	 | Reports variables likely to mutate.                                                                          	 |  ✅   	   |
|  Unused Imports  	  | Reports unused `import` declarations in your contracts.                                                      	 |  ❌   	   |
| Mutation Grapher 	  | Creates a graph showing the variables likely to mutate connected to the places where they undergo mutations. 	 |  ❌   	   |
| Struct Repacker  	  | Suggests an alternative way to define a struct such that it takes less storage slots.                        	 |  ❌   	   |

## Getting Started

### Local Build

There is one prerequisite to build `soliris` locally:
- Have Rust installed. [Instructions available here](https://www.rust-lang.org/tools/install).

We also recommend to [install Task](https://taskfile.dev/installation/).

Then, run the following command to build the project:
```shell
task build-release
```

### Run Soliris

You can scan a test contract available in this repository:
```shell
./target/release/soliris scan -f tests/contracts/SimpleContract.sol
```

## Contributing

If you would like to contribute to this project, please refer to the instructions in the
dedicated document [here](./CONTRIBUTING.md).

## Authors

This project is a pure open-source contribution to the Solidity ecosystem.
It is currently maintained by the 🤖 at [Quartz Technology](https://github.com/quartz-technology).
