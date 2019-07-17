curl https://sh.rustup.rs -sSf | sh
rustup component add rustfmt --toolchain stable-x86_64-apple-darwin
cargo build
cargo run
