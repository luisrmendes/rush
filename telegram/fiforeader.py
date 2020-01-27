import os
import atexit

FIFO_PATH = "fifo.tmp"

@atexit.register
def cleanup():
    try:
        os.unlink(FIFO)
    except:
        pass

def fifoReader():
    filename = "fifo.tmp"

    # Block until writer finishes...
    with open(filename, 'r') as f:
        data = f.read()
        print(data)

    
