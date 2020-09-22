from aux import *

class Server:
    
    def getIpv4(self):
        result = bashCall("dig @resolver1.opendns.com A myip.opendns.com +short -4")
        return result
    
    def getIpv6(self):
        result = bashCall("dig @resolver1.opendns.com AAAA myip.opendns.com +short -6")
        return result
    
    def serverBrightness(self, context):
        result = bashCall("sudo sh /home/luis/github/dotFiles/brightness.sh " + context.args[0])
        return result