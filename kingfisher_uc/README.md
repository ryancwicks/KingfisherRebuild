# Kingfisher Microcontroller

This project contains the code for controlling the Kingfisher board for it's onboard ATMEGA32U4 microcontroller. It cannot be programmed without a avr programmer, as the chip does not appear to use the Leonardo bootloader.

## Programming

Build the code first: 

```
cargo build --release
```

Then change the elf file into a hex file and strip any uneccessary symbols.

```
avr-objcopy -O ihex target/avr-atmega32u4/release/kingfisher_uc.elf firmware.hex
```

Then upload it to the board:

```
avrdude -c atmelice_isp -p m32u4 -P usb -U flash:w:firmware.hex:i 
```

If you have the Leonardo bootloader code, you can program through USB with raverdude by pressing the reboot button and then running:

```
cargo run
```

## USB Serial Strange Behaviour

It seems the serial write blocks if nothing is listening to the USB bus on the reciever side. 