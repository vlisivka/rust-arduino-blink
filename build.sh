#!/bin/sh

#cargo +nightly build -Z build-std=core --target ./avr-atmega328p.json

cargo +nightly build --release
