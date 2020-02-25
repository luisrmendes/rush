import os, sys
import time
import subprocess

FIFO1_PATH = "/tmp/brains_to_telegram"
FIFO2_PATH = "/tmp/telegram_to_brains"

# Executes bash commands, handles bad ones
def bash_call(content):

    # This is all fucked up
    # if content.find(';') != -1 or content.find('&&') != -1:
    #     # for x in content.split(';'):
    #     #     bash_call(x)
        
    #     # slight security hazard, look to replace!! (shell=True)
    #     result = subprocess.run(content, stdout=subprocess.PIPE, shell=True)
    #     return result.stdout.decode('utf-8')

    # try:
    #     result = subprocess.run(content.split(),  stdout=subprocess.PIPE)

    #     # returns 127 in ssh commands, correct this
    #     # if result.returncode == 127:
    #     #     return "Error: No file was found"

    #     return result.stdout.decode('utf-8')       
    # except subprocess.CalledProcessError:
    #     pass # handle errors in the called executable
    # except OSError:
    #     print("Command " + content + " not found")
    #     return "Command " + content + " not found"

    result = subprocess.run(content, stdout=subprocess.PIPE, shell=True)
    return result.stdout.decode('utf-8')

# Just a handler for fifoReader        
def handleFIFO(data):
    print(data) 
    return data

# Polling method for checking if a fifo exists, then reads it
def fifoWatcher_and_Reader():
    while 1:
        if watchFile(FIFO1_PATH):
            # Block until writer finishes...
            with open(FIFO1_PATH, 'r') as f:
                data = f.read()
                handleFIFO(data)
        else:
            watchFile(FIFO1_PATH)  

# Returns true if file with filename exists, if none waits check_interval seconds to check again               
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

# Writes to FIFO, given constant FIFO path
def fifoWriter(content):
    #os.unlink(FIFO2_PATH)
    os.mkfifo(FIFO2_PATH)
    fd = os.open(FIFO2_PATH, os.O_WRONLY| os.O_CREAT)
    os.write(fd, str.encode(content) + str.encode('\0'))
    os.close(fd)
    os.unlink(FIFO2_PATH)
