#!/usr/bin/bash
set -eu
FILE="${1:-$(pwd)/target/avr-atmega328p/release/rust-arduino-blink.elf}"
UART_DEVICE="/dev/ttyUSB0"

[ -f "$FILE" ] || { cargo +nightly-2021-01-07 build --release || exit 1 ; }

[ -f "$FILE" ] || { echo "ERROR: File \"$FILE\" is not found. Run \"cargo +nightly build --release\"." >&2 ; exit 1; }

exec avrdude -q -patmega328p -carduino -P"$UART_DEVICE" -D -U "flash:w:$FILE:e"
