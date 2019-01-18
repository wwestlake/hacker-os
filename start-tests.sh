#!/bin/bash
bootimage run --bin test-basic-boot -- \
    -serial mon:stdio -display none \
    -device isa-debug-exit,iobase=0xf4,iosize=0x04

bootimage run --bin test-panic -- \
    -serial mon:stdio -display none \
    -device isa-debug-exit,iobase=0xf4,iosize=0x04
