from aux import *
from psw import psw

class Desktop:
    
    def getDesktopStatus(self):
        result = bashCall("ping -c 1 192.168.1.71 2>&1 >/dev/null ; echo $?")
        result = result.replace("\n", '')
        if result == "0":   
            send = "Desktop is online"
        else:
            send = "Desktop is offline"
            
        return send        

    def shutdownDesktop(self):
        goodbyeMsg = "Server says goodbye!"
        bashCall("ssh luis@192.168.1.71 'shutdown'")
        return result
        
    def wakeupDesktop(self):
        result = bashCall("wakeonlan 00:D8:61:a1:CE:00")
        return result