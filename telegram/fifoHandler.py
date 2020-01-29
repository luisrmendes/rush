import os, sys
import time
from threading import Thread

FIFO1_PATH = "/tmp/brainsfifo1"
FIFO2_PATH = "/tmp/brainsfifo2"

def handleFIFO(data):
    print(data)

def fifoReader():
    while 1:
        if watchFile(FIFO1_PATH):
            # Block until writer finishes...
            with open(FIFO1_PATH, 'r') as f:
                data = f.read()
                handleFIFO(data)
        else:
            watchFile(FIFO1_PATH)  

    
def watchFile(filename, time_limit=0, check_interval=0.1):    
    now = time.time()
    last_time = now + time_limit

    while time.time() <= last_time:
        if os.path.exists( filename ):
             return True
        else:
            # Wait for check interval seconds, then check again.
            time.sleep( check_interval )

    return False


def fifoWriter(content):
    os.mkfifo(FIFO2_PATH)
    fd = os.open(FIFO2_PATH, os.O_WRONLY| os.O_CREAT)
    os.write(fd, str.encode(content) + str.encode('\0'))
    os.close(fd)
    os.unlink(FIFO2_PATH)
    