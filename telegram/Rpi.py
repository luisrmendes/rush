from system import bash_call


class Rpi:

    def speakersOn(self):
        result = bash_call("ssh pi@192.168.1.106 'python3 ~/rush/energienie.py 1=on && python3 ~/rush/energienie.py 1=on'")
        if (result == ''):
            return "Ok"
        else:
            return "Some kind of error occured"        
        
    def speakersOff(self):
        result = bash_call("ssh pi@192.168.1.106 'python3 ~/rush/energienie.py 1=off && python3 ~/rush/energienie.py 1=off'")
        if (result == ''):
            return "Ok"
        else:
            return "Some kind of error occured"
        
    def lightsOn(self):
        result = bash_call("ssh pi@192.168.1.106 'python3 ~/rush/energienie.py 2=on && python3 ~/rush/energienie.py 2=on'")
        if (result == ''):
            return "Ok"
        else:
            return "Some kind of error occured"
        
    def lightsOff(self):
        result = bash_call("ssh pi@192.168.1.106 'python3 ~/rush/energienie.py 2=off && python3 ~/rush/energienie.py 2=off'")
        if (result == ''):
            return "Ok"
        else:
            return "Some kind of error occured"
        
    def youtubePlay(self, context):
        bash_call("ssh pi@192.168.1.106 'python3 ~/rush/energienie.py 1=on && python3 ~/rush/energienie.py 1=on'")
        command = "ssh pi@192.168.1.106 'rm -f play.mp3 && youtube-dl --extract-audio --audio-format mp3 -o 'play.mp3' "
        command += context.args[0]
        command += " && omxplayer play.mp3 &'"
        result = bash_call(command)
        
        return "Song has ended"
          
        
    