# Rush - Canivete Suíço

## Telegram Bot dependencies
python3  
python-pip3  
pip3 install python-telegram-bot  

## server side dependencies
dig (bind?)  
wakeonlan  
Bot requires target desktop i2c monitor bus number  

## desktop side dependencies
ddcutil  
i2c-tools  
User monitorControl in i2c group  

## Ambrosio commands and description  
desktop_wakeup - Wake up desktop WoL  
ipv4 - Get ipv4 home address  
ipv6 - Get ipv6 home address  
desktop_status - Check if desktop is online, what OS is running  
lights_on - Turns on sockets 1 and 2  
lights_off - Turns off sockets 1 and 2  
