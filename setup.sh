curl https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env
rustup component add rustfmt --toolchain stable-x86_64-apple-darwin # This might need to be edited to accommodate different OSes
cargo build
cargo run
