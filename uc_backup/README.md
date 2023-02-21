# Backing up the existing uC code

Since I don't have any access (at least any obvious public access) to the original uC code, it's prudent to back up what's been flashed onto the device currently. The following is an explanation of how I did that using avrdude and my Atmel ICE programmer.

To download the program hex, eeprom contents and fuse settings, do the folllowing:

Read and store the fuse values
```
avrdude -c atmelice_isp -p m32u4 -P usb -U lfuse:r:lfusefile.hex:i 
avrdude -c atmelice_isp -p m32u4 -P usb -U hfuse:r:hfusefile.hex:i
avrdude -c atmelice_isp -p m32u4 -P usb -U efuse:r:efusefile.hex:i
```

Read and store the main code and eeprom values
```
avrdude -c atmelice_isp -p m32u4 -P usb -U flash:r:firmware.hex:i 
avrdude -c atmelice_isp -p m32u4 -P usb -U eeprom:r:eeprom.eep:i
```
