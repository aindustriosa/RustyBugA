# Generic hardware structure of the line follower

## Motion
The vast majority of the line followers you'll see will have just two wheels in the back of the robot and will use differential traction:

Example from https://www.jsumo.com/storm-pid-controlled-fast-line-follower :
![Example from https://www.jsumo.com/storm-pid-controlled-fast-line-follower](https://www.jsumo.com/storm-pid-controlled-fast-line-follower-1990-72-B.jpg)

Some robots have 4 wheels also in the back axis:
Example from https://forum.pololu.com/t/fast-line-follower-build/11956

![Example of 4 wheels configuration](https://forum.pololu.com/uploads/default/optimized/2X/f/fbeed1eedc243b8668a6e72bcc6d5b7794c36734_2_666x500.jpg)

In order to prevent the line sensors from touching the floor, some kind of support is put under the front of the robot.

## Position sensors
Position sensors are used to calculate the robot location in the world. In simple robots, they give just the information to don't miss the line. In advanced robots, they can provide enough information to create a map of the path and make strategies to complete the loops faster.

### Line sensors
The objective of the line sensors is to read the position of the robot with respect to the black line that represents the path to follow.

[![watch this](https://img.youtube.com/vi/ijKpDYibkUs/default.jpg)](https://www.youtube.com/watch?v=ijKpDYibkUs) Robot following a line in slow motion.

Usually, an array of independent light sensors is used for this task. Here are some examples:

[![watch this](https://img.youtube.com/vi/Q0GXCIZL1N4/default.jpg)](https://www.youtube.com/watch?v=Q0GXCIZL1N4) Simple array of two sensors

https://www.instructables.com/DIY-Line-Follower-Sensor-Array/
![DIY board](https://content.instructables.com/F2U/VGGD/HX1ISIRF/F2UVGGDHX1ISIRF.jpg)

There are alternatives, like:
- Cameras: https://ozrobotics.com/shop/modmi-modular-re-configurable-robot-for-research-learning-and-experimentation/

![Image of a line follower with visible light camera](https://ozrobotics.com/wp-content/uploads/2023/07/ShenzhenCorebingoCoLtd-Mobile-Platform-Visual-Line-Following-Experiment.jpg)

Another example from https://www.instructables.com/Line-Follower-With-ESP32-CAM-L0Cost-Robot-Controll/ 

![another example of line follower with a cheap camera](https://content.instructables.com/F5K/KZJ5/LJWVIXMC/F5KKZJ5LJWVIXMC.jpg)

These sensors can also be used to detect special marks in the path (like the start/ end of a lap).

### Measuring speed: Encoders
Encoders attached to the motor shafts are the usual way to measure the speed of each wheel.

[![watch this](https://img.youtube.com/vi/zzHcsJDV3_o/default.jpg)](https://www.youtube.com/watch?v=zzHcsJDV3_o) How an encoder works.

### IMU: Inertial Measurement Unit
An IMU can detect accelerations of the line follower and, from these, speed and location. They are used only in very advanced robots.

### Gyroscope
A gyro is a sensor that detects the angular position of the robot. They are used only in very advanced robots.

## User interface
The robot provides some facilities to so the user can tell the robot what to do in each moment (calibrate, start/stop, go faster...) and know the robot state.

### Outputs
#### LEDs
LEDs are the most straightforward way to tell something to the user at a distance: if the battery is low, if it is running at max speed, if it has detected a curve... LEDs are cheap, but you need the pins and extra resistors to drive them.

#### Music
The robot also can express it's state with sound. The most economical way to do this is by using buzzers, that can perform just beeps or even melodies.

#### More advanced output: screens
Image from https://hackaday.io/project/166435-small-race-competition-line-follower

![screen in a line follower](https://cdn.hackaday.io/images/5891131562541613307.JPG)

### Inputs
#### Buttons
The most used inputs are the buttons. Note that a number of combinations can be made with a single button: short pulse, long pulse...

### Advanced input/output
#### Command line interface (CLI) through serial port
Some advanced robots expose a serial port, which is a text based input/output interface with a computer. You can use the serial port to make the robot print periodically it's status (like logs) or to write down some orders to it, like configuration options. 

#### Wireless communication
If the robot has the hardware, you could leverage a Bluetooth interface to expose a CLI. You could also use WiFi to host a configuration web page in the robot.

Note that in some competition rules, wireless connection with robots is not allowed.
