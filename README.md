# Rustbridge workshop
This repo is for the Rustbridge workshop on the 23th of February 2019 in Sofia

## Preparations

### Installing Rust
For installation it's advised to use rustup as it will (among other things) allow you to switch between versions of Rust without having to download anything additional.

#### Linux

Execute: curl https://sh.rustup.rs -sSf | sh

#### Mac

$ brew install rustup

Use rustup to install the Rust compiler (rustc) and the Rust package manager (cargo).

$ rustup-init

To verify you can run:

$ rustc –version

#### Windows

Download the right installer for your architecture:

https://win.rustup.rs/x86_64
https://win.rustup.rs/i686

#### More information

https://rustup.rs/

Having problems or questions? [Email me](mailto:arjan@rustbg.org)

### IDE
An IDE isn't mandatory, but recommended. There are several IDEs supporting Rust. 

For the workshop we recommend using VS Code with the [Rust (rls) extension](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust).