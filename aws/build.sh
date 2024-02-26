#!/bin/bash

export PATH="/home/ec2-user/.cargo/bin:$PATH"

cd /home/ec2-user/app/source
cargo build --release

mv ./target/release/ /home/ec2-user/app/bin
rm -rf /home/ec2-user/app/source

