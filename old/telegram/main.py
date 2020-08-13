from bot_handler import updater
from threading import Thread
from system import *
import subprocess

# Start telegram bot
updater.start_polling()

# thread = Thread(target = fifoReader, args = ())
# thread.start()

import datetime
now = datetime.datetime.now()
while (1):
    time.sleep(60)
    now = datetime.datetime.now()
    if now.hour == 6 and now.minute == 50:
        bash_call("ssh pi@192.168.1.106 'python3 ~/rush/energienie.py 1=on && python3 ~/rush/energienie.py 1=on'")
        bash_call("ssh pi@192.168.1.106 'python3 ~/rush/energienie.py 2=on && python3 ~/rush/energienie.py 2=on'")
        time.sleep(5)
        bash_call("ssh pi@192.168.1.106 'omxplayer ~/rush/wakeup_songs/03.\ Lakeside\ Park.mp3'")
