# New Rust Windows Driver
This is a simple tool that simplifies the creation for Windows Driver crates in Rust.

## Introduction
To be honest, the [Microsoft's Guide](https://github.com/microsoft/windows-drivers-rs/blob/main/README.md#adding-windows-drivers-rs-to-your-driver-package) to create a Windows driver crate costs way too much manual labor, especially the part for configuring the files. Why not we just simplify it?

This project will create a crate which has initialized all stuff, including all WDK dependencies, cargo configurations, and any other chores.

## Build
Just follow regular way to build a Rust program:
```
cargo build
```

## Run
Place the tool somewhere you keep all of your projects. Double click it to run the tool. Then follow the prompt to create your crate.

## License
This repository is licensed under the [MIT License](./license.txt).