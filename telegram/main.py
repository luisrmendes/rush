from commandHandler import updater
from aux import *
import subprocess

# Start telegram bot
updater.start_polling()

# thread = Thread(target = fifoReader, args = ())
# thread.start()

import datetime
now = datetime.datetime.now()

    # if now.hour == 6 and now.minute == 50:
    #     bashCall("ssh pi@192.168.1.106 'python3 ~/rush/energienie.py 1=on && python3 ~/rush/energienie.py 1=on'")
    #     bashCall("ssh pi@192.168.1.106 'python3 ~/rush/energienie.py 2=on && python3 ~/rush/energienie.py 2=on'")
    #     time.sleep(5)
    #     bashCall("ssh pi@192.168.1.106 'omxplayer ~/rush/wakeup_songs/03.\ Lakeside\ Park.mp3'")
    
