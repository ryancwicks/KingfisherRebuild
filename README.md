# Kingfisher Boat Rebuild

The goal of this project is to rebuild a Kingfisher boat from Clearpath Robotics

## Project Layout

 - ControlBoard: reverse engineered board design (very incomplete, but most of the signals were traced out.)
 - kingfisher-uc: Rust microcontroller code for controlling the motors and lights and recieving radio input.
 - kf_data_types: Rust data types library used to allow different parts of the project to communicate.


## Links:

[AntMicro Nano Board](https://github.com/antmicro/jetson-nano-baseboard) -  Carrier board for the Jetson Nano that replaces the old HardKernel PC.
[Kingfisher Github Repo](https://github.com/clearpathrobotics/kingfisher) - Original Kingfisher code. 
[Kingfisher Apps Github Repo](https://github.com/clearpathrobotics/kingfisher_apps) - Apps for the Kingfisher?
[Kingfisher Bringup Github Repo](https://github.com/clearpathrobotics/clearpath_kingfisher) - Bringup?

There used to be a ros wiki page on the Kingfisher, but it seems to have been removed.

