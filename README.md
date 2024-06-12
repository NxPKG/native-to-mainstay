# Native to Mainstay

**N**ative **T**o **A**nchor(nta) is a CLI tool that helps you generate [Mainstay](https://github.com/nxpkg/mainstay) IDLs and clients for native [Solana](https://github.com/solana-labs/solana) programs.

## Installation

- Install with cargo:

```sh
cargo install native-to-mainstay
```

or

- Build from source:

```sh
git clone https://github.com/nxpkg/native-to-mainstay.git
cd native-to-mainstay/generator
cargo build --release
```

See the available commands with:

```
native-to-mainstay
```

**NOTE:** You can see the optional arguments for any command with:

```sh
native-to-mainstay <COMMAND_NAME> --help
```

Examples will assume you've installed with **cargo**. If you've built from source, you can follow the examples by running the binary or adding an alias to the binary accordingly.

## Generation

There is an example tutorial in `example` directory. You can also check out `client` directory to see the generated packages and it's tests.

#### Generate IDL

You can easily create an Mainstay IDL from a Solana program with:

```sh
native-to-mainstay idl <PATH_TO_SOLANA_PROGRAM>
```

#### Generate package

Having the IDL is nice but IDL alone will not allow program interactions. Luckily this tool will create a package that you can import to your project and interact with native Solana programs like you would with Mainstay programs.

```sh
native-to-mainstay package <PATH_TO_SOLANA_PROGRAM>
```

**NOTE:** In the case of an unsupported or an unknown type(like in the case of type being defined outside of the program crate), serialization/deserialization differences, or custom account trait implementations the generated code might not be correct. Some of the [SPL](https://github.com/solana-labs/solana-program-library) programs have these problems and you can check out `client/packages` to see how it was handled. There are a lot of things that could go wrong in this process, it wouldn't be wise trying to list all of them.

#### Supported de/serializers

- [Borsh](https://borsh.io)

## License

Licensed under [MIT](https://github.com/nxpkg/native-to-mainstay/blob/master/generator/LICENSE).
