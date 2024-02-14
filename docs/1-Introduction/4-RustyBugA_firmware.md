# RustyBugA firmware
The RustyBugA firmware is a work in progress and it is still in it's early stages. Anyway, the team writing it has some experience coding the firmware of a line follower (in C) so we can assume a number of things that will contain this code.

This page will be updated with the features as we develop them (hopefully :D).

## Robot states
The robot can be in a number of different stages during it's running time, for example:
- Calibrating sensors
- Running
- Waiting for user input
- ...

The robot can go from one state to another given a series of conditions, like:
- It has finished the work to be done in a state.
- The user has interrupted.
- There has been detected a failure.
- There is an error/exception in the code.

There is a concept called _state machine_ that allows us to code these states and transitions. We will code a state machine to do this.

## Control loop
As we will start by a synchronous design, we need a loop that performs in series the needed tasks the robot must perform during a state.

## Control tasks during the running state
During the running state, we would like to perform a number of tasks in a constrain period of time. Here are some of them:

### Line sensing
The objective of this task is to give a measurement of how close to the center of the line is the robot. In this task, the robot has to:
- Read line sensor's measurements (with different light conditions).
- Detect differences in the light detected by the individual sensors.
- Estimate the position of the line.

There are a number of items to consider here:
- Should we use past measurements?
- Do all individual sensors have the same sensitivity?
- Does ambient light influence in the measurements?
- ...

### Speed/movement planning

### motors control

### More advanced algorithms
#### Map recording
