import os, sys
import time
import subprocess
from threading import Thread

def bashCall(content):
    result = subprocess.run(content, stdout=subprocess.PIPE, shell=True)
    return result.stdout.decode('utf-8')

# TODO: How to get method output from a thread
def bashCallThread(content):
    thread = Thread(target = bashCall, args = (content,))
    thread.start()
