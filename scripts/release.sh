#!/bin/bash

cargo build --release

cp ./target/release/ace /usr/local/bin/ace
