#! /usr/bin/bash
set -eu
FILE="$(pwd)/target/avr-atmega328p/release/rust-arduino-blink.elf"

cargo +nightly build --release || exit 1
#avr-strip "$FILE"

[ -f "$FILE" ] || { echo "ERROR: File \"$FILE\" is not found. Run \"cargo +nightly build --release\"." ; exit 1; }

avrdude -q -patmega328p -carduino -P/dev/ttyUSB0 -D -U "flash:w:$FILE:e"
