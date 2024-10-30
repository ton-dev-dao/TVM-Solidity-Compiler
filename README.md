<meta name="title" content="TVM-Solidity-Compiler">
<meta name="description" content="Solidity compiler for TVM">
<meta name='keywords' content='compiler, smart-contracts, blockchain, solidity, tvm, ton'>

# The TVM Solidity compiler

[![GitHub](https://img.shields.io/github/license/ton-dev-dao/TVM-Solidity-Compiler?style=for-the-badge)](./LICENSE)


Port of the Solidity smart-contract [compiler](https://github.com/ethereum/solidity) generating TVM bytecode for TVM compatible blockchains.

## TVM Solidity API reference

[API documentation is here](https://github.com/ton-dev-dao/TVM-Solidity-Compiler/blob/main/API.md)

## Code samples

See [samples](https://github.com/ton-dev-dao/samples). It's an example of using TVM Solidity compiler and [blueprint](https://github.com/ton-org/blueprint). 

## Build and Install

### Sold driver

We recommend using `sold` to compile smart-contracts. Documentation is available at [README.md](https://github.com/ton-dev-dao/TVM-Solidity-Compiler/blob/main/sold/README.md).

### Building compiler

Original Instructions about how to build and install the Solidity compiler can be found in the [Solidity documentation](https://solidity.readthedocs.io/en/latest/installing-solidity.html#building-from-source).

#### Ubuntu Linux

```shell
git clone https://github.com/ton-dev-dao/TVM-Solidity-Compiler
cd TVM-Solidity-Compiler
sh ./compiler/scripts/install_deps.sh
mkdir build
cd build
cmake ../compiler/ -DCMAKE_BUILD_TYPE=Release
cmake --build . -- -j8
```

#### Windows 10

Install Visual Studio Build Tools 2019, Git bash, cmake.
Run Developer PowerShell for VS 2019

```shell
git clone https://github.com/ton-dev-dao/TVM-Solidity-Compiler
cd TVM-Solidity-Compiler
compiler\scripts\install_deps.ps1
mkdir build
cd build
cmake -DBOOST_ROOT="..\compiler\deps\boost\" -DCMAKE_MSVC_RUNTIME_LIBRARY=MultiThreaded ..\compiler
cmake --build . --config Release -- /m
```

## Links

 * [Assembler and disassembler](https://github.com/ton-dev-dao/ton-dev-assembler)
 * [Code samples](https://github.com/ton-dev-dao/samples/tree/main) in TVM Solidity

## License
[GNU GENERAL PUBLIC LICENSE Version 3](./LICENSE)
