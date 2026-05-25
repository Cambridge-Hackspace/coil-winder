run:
	cargo run

devbuild:
	cargo build
	avr-objcopy -O ihex -R .eeprom target/avr-none/debug/coil_winder.elf target/avr-none/debug/coil_winder.hex

sim:
	cargo build
	simavr -m atmega328p -f 16000000 target/avr-none/debug/coil_winder.elf
