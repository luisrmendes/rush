from system import bash_call

class Server:
    
    def getIpv4(self):
        result = bash_call("dig @resolver1.opendns.com A myip.opendns.com +short -4")
        return result
    
    def getIpv6(self):
        result = bash_call("dig @resolver1.opendns.com AAAA myip.opendns.com +short -6")
        return result
    
    def serverBrightness(self, context):
        result = bash_call("sudo sh /home/luis/github/dotFiles/brightness.sh " + context.args[0])
        return result