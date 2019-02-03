#!/bin/bash

cp ./target/release/cromwell /usr/local/bin/cromwell && setcap CAP_SYS_ADMIN=eip /usr/local/bin/cromwell
getcap /usr/local/bin/cromwell
