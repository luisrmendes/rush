# Design Notes

## Why a Priority Queue

To manage multiple commands that have control over a certain feature. For example, brightness control of the system 1 monitors - when a user sets a command to disable automatic brightness control, this decision has top priority. Since the decision to change the brightness is performed by the command with most priority, no other command made by the program will change the brightness control.
