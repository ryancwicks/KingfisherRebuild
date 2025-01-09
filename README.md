# Kingfisher Boat Rebuild

The goal of this project is to rebuild a Kingfisher boat from Clearpath Robotics

## Hardware

The main system CPU is a Fit2 PC with an intel Atom processor. To cross compile for this device, you want to build using the i686-unknown-linux-gnu toolchain.

```
sudo apt-get --no-install-recommends install gcc-multilib
rustup target add i686-unknown-linux-gnu
```

You can build the assorted nodes for the target using

```
cargo build --target=i686-unknown-linux-gnu --release
```

## Project Layout

 - ControlBoard: reverse engineered board design (very incomplete, but most of the signals were traced out.)
 - [kingfisher-uc](./kingfisher_uc/README.md): Rust microcontroller code for controlling the motors and lights and recieving radio input.
 - [kf_data_types](./kingfisher_nodes/kingfisher_data_types/README.md): Rust data types library used to allow different parts of the project to communicate.
 - [kingfisher_nodes](./kingfisher_nodes/README.md): The programs that run on the main computer to co-ordinate the system.
 - [tools](./tools/README.md): A set of tools, scripts and config files used by the system.
 - [uc_backup](./uc_backup/README.md): A backup of the original Clearpath Robotics microcontroller code.
 - [yocto_image](./yocto_image/README.md)base image for the main pc.

## Links:

[AntMicro Nano Board](https://github.com/antmicro/jetson-nano-baseboard) -  Carrier board for the Jetson Nano that replaces the old HardKernel PC.
[Kingfisher Github Repo](https://github.com/clearpathrobotics/kingfisher) - Original Kingfisher code. 
[Kingfisher Apps Github Repo](https://github.com/clearpathrobotics/kingfisher_apps) - Apps for the Kingfisher?
[Kingfisher Bringup Github Repo](https://github.com/clearpathrobotics/clearpath_kingfisher) - Bringup?

There used to be a ros wiki page on the Kingfisher, but it seems to have been removed.

