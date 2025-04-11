# Assignment 2

This is example "hello world" project from Developing Programs in Rust https://solana.com/ru/docs/programs/rust . 

## Usage
First, setup your solana cli.<br>

    solana-keygen new
    solana config set -ud
    solana airdrop 2

Then clone this repo to your machine and add dev dependencies.<br>

    git clone https://github.com/a5bbbbb/BC2_assignment_2.git
    cd BC2_assignment_2
    cargo add solana-program-test@1.18.26 --dev
    cargo add solana-sdk@1.18.26 --dev
    cargo add tokio --dev
    cargo add solana-client@1.18.26 --dev

Then build the proram and deploy it.<br>

    cargo build-sbf
    solana program deploy ./target/deploy/hello_world.so
    
Then you can test this program using the example.<br>

    cargo run --example client

## Example
Solana network: [devnet](https://api.devnet.solana.com )

Program address: 8LVMqMuEpNq4Sng8SirxdYvDgTyxd5CkgmqsmEth2trY

Transaction Signature: 3JckvkQDfSErPbHgzHwjpJ5iBmZFBvfAh7AhiKKoUV8cADiJRQ6CEQ9k7QKyrf6fxhLZeggkGfWK5Bsn75gTJWru
