#!/bin/bash 

cd ./frontend
yarn install
yarn build 

cd ../backend
rustup override set nightly
cargo build 
cargo run
