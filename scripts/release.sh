#!/bin/bash

cargo build --release && cp ./target/release/ace /usr/local/bin/ace && setcap CAP_SYS_ADMIN=eip /usr/local/bin/ace
