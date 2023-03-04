# Tools

This part of the repository contains sets of tools and config files for the system that aren't specific to any given bit of software.

## UDEV rules

This directory contains a set of udev rules to ensure that the microcontroller and other system devices always enumerate the same way.

To get the information needed to fill out a USB connected devices udev rules, run the following command and then plug in the device:

```
udevadm info --attribute-walk --path=$(udevadm info --query=path --name=/dev/ttyACM0)
```

Use the existing rules files under [udev-rules](./udev-rules) and fill in the appropriate fields. 