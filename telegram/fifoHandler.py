import os
import time
from threading import Thread

FIFO_PATH = "/tmp/brainsfifo"

def handleFIFO(data):
    print(data)

def fifoReader():
    while 1:
        if watch_file(FIFO_PATH):
            # Block until writer finishes...
            with open(FIFO_PATH, 'r') as f:
                data = f.read()
                handleFIFO(data)
        else:
            watch_file(FIFO_PATH)  

    
def watch_file(filename, time_limit=0, check_interval=0.1):
    
    now = time.time()
    last_time = now + time_limit

    while time.time() <= last_time:
        if os.path.exists( filename ):
             return True
        else:
            # Wait for check interval seconds, then check again.
            time.sleep( check_interval )

    return False
