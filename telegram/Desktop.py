from system import bash_call
from psw import psw

class Desktop:
    
    def getDesktopStatus(self):
        result = bash_call("ping -c 1 192.168.1.71 2>&1 >/dev/null ; echo $?")
        result = result.replace("\n", '')
        if result == "0":   
            send = "Desktop is online"
        else:
            send = "Desktop is offline"
            
        return send
    
    

    def shutdownDesktop(self):
        # TODO: Needs some work, maybe change to a ssh call 
        result = bash_call("net rpc shutdown -f -t 0 -C 'Bye Bye, says server' -U luis%" + psw + " -I 192.168.1.71")
        return result
        
    def wakeupDesktop(self):
        result = bash_call("wakeonlan 00:D8:61:a1:CE:00")
        return result