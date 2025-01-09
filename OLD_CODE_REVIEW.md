# Reviewing old code

## Bringup:
- Sensor.launch indicate there is a nmea based gps on port /dev/ttyACM0
- also indicates a cmps09 on port /dev/ttyUSB0 running at 50 Hz. This appears to be a [tilt compensated compas module](https://www.robot-electronics.co.uk/htm/cmps09doc.htm)
- There appear to be 3 topics published under clearpath_base: data/system_status, data/differential_output, data/power_status
- There is also a teleop.launch file talking about drive and turn speed.

## Kingfisher Apps:
- Seems to be bringup code for a ROS joystick driver (TeleOp driver)
- includes some python code for running the joystick, I think. I don't know ROS that well.

## Kingfisher:
- looks like ros code to install the appropriate nodes on the device. There is a reference to the firmware, but it's an internal Clearpath link.
    - git: {local-name: kingfisher_firmware, uri: 'ssh://internal.clearpathrobotics.com:1222/git/CPR/kingfisher_firmware.git'}
- There are also references to a clearpath bitbucket account that doesn't exist anymore.
- This repo has lots of interesting stuff in it:    
    - vserial - some sort of port forwarder
    - bridge forwards udp multicast to a port
    - netserial, seems to forward tcp to serial with socat
    - installs android teleop for control
    - camera specific ffmv settings in ffmv.yaml
    - old fashioned ublox protocol setting
    - imu_um6 is the imu
    - nmea gps was at 115200
    - camera1394 driver for camera
    - gps and sonar are sent over netserial/socat publishing
    - vserial was used for broadcasting sonar data.
    - Looks like the uC is arduino based, uses the same udev rules.
    - point grey firefly or chameleon cameras
    - Sonar was a UC232R - NVM adapter to sonar was UC232R
    - user was administrator
    - The messages section has drive and system state info (like battery and RC states)
    - some interesting python scripts under nodes for testing the boat and handling twists
    - twist subscriber has some info about channel mixing.
    