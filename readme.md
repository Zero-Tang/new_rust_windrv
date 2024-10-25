# New Rust Windows Driver
This is a simple tool that simplifies the creation for Windows Driver crates in Rust.

## Introduction
To be honest, the [Microsoft's Guide](https://github.com/microsoft/windows-drivers-rs/blob/main/README.md#adding-windows-drivers-rs-to-your-driver-package) to create a Windows driver crate costs way too much manual labor, especially the part for configuring the files. Why not we just simplify it?

This project will create a crate which has initialized all stuff, including all WDK dependencies, cargo configurations, and any other chores. \
If you do things manually, then these chores include:

- Use `cargo` to add WDK-related dependencies.
- Modify `Cargo.toml`.
- Create `Makefile.toml`.
- Create `.cargo/config.toml`.
- Create `<crate_name>.inx`.

Can you memorize all steps? It's somewhat unlikely to do these chores accurately.

## Build
Just follow regular way to build a Rust program:
```
cargo build
```

## Run
This tool expects you have installed [rust-lang](https://www.rust-lang.org/learn/get-started).

Place the tool somewhere you keep all of your projects. Double click it to run the tool. Then follow the prompt to create your crate.

After the crate is created, you should be able to build your driver:
```
cargo make
```

Please note that you need to satisfy the [Build Requirements](https://github.com/microsoft/windows-drivers-rs/tree/main?tab=readme-ov-file#build-requirements) on your own.

## License
This repository is licensed under the [MIT License](./license.txt).