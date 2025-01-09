# 13DOF Click Micro Serial Converter

This application takes the Mikro Click 13DOF board and sends the data over USB serial topside for resending through DDS. This board contains 3 different sensors:

- [BMM150](https://download.mikroe.com/documents/datasheets/BMM150_Datasheet.pdf) Geomagnetic sensor (3DOF)
- [BMI088](https://download.mikroe.com/documents/datasheets/BMI088_Datasheet.pdf) IMU (6DOF)
- [BME680](https://download.mikroe.com/documents/datasheets/BME680_Datasheet.pdf) Environmental Sensor (gas, temperature, humidity, pressure) (4DOF)

The board is connected to a STM32F411CE on a Black Pill board. Communication is through I2C.

The driver structure was inspired by [this](https://gitlab.com/alaarmann/bmx055-rs) project.

## Installing the toolchain

```
rustup install thumbv7em-none-eabi
```

## Debug Compile

To access the defmt logs without a lot of USB noise:

```
DEFMT_LOG=mikro_click_13dof=trace,embassy_usb_synopsis_otg=off cargo run
```
