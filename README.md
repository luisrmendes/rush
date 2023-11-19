# Rush

## How To Run
```sh
cd src/main/  
go run main.go 
```

## Host Machine Dependencies
- bind (dig...) 
- wol  
- Requires  desktop i2c monitor bus numbers  
- gnu-netcat

## Client Side Brightness Control Dependencies  
- ddcutil  
- i2c-tools  
- User monitorControl in i2c group  
- User monitor in video group
- Add udev rule to give video group brightness control permissions
- Add monitorcontrol write permissions to kbd_backlight (eg on .bashrc, "sudo chmod 757 /sys/class/leds/tpacpi\:\:kbd_backlight/brightness")

## Ambrosio commands and description  
desktop_wakeup - Wake up desktop WoL  
ipv4 - Get ipv4 home address  
ipv6 - Get ipv6 home address  
desktop_status - Check if desktop is online, what OS is running  
lights_on - Turns on sockets 1 and 2  
lights_off - Turns off sockets 1 and 2  
disable_brightness_auto_control - Disable automatic brightness control  
enable_brightness_auto_control - Enable automatic brightness control  
get_system_status - Get system status

## ESP8266 PIN layout
```c
#define LED_1_PIN 2
#define LED_2_PIN 0
#define LED_3_PIN 4
#define LED_4_PIN 5
```
![Alt text](image.png)
