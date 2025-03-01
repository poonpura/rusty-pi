#!/bin/bash
cargo build
if [ $? -eq 0 ]; then  
    arm-none-eabi-objcopy -O binary target/armv6zk-none-eabihf/debug/kernel kernel.img
else
    echo "Build failed. Skipping objcopy."
fi