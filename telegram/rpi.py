from system import bash_call


class Rpi:

    def speakersOn(self):
        bash_call("ssh pi@192.168.1.106 'python3 ~/rush/energienie.py 1=on && python3 ~/rush/energienie.py 1=on'")
        
    def speakersOff(self):
        bash_call("ssh pi@192.168.1.106 'python3 ~/rush/energienie.py 1=off && python3 ~/rush/energienie.py 1=off'")
        
    def lightsOn(self):
        bash_call("ssh pi@192.168.1.106 'python3 ~/rush/energienie.py 2=on && python3 ~/rush/energienie.py 2=on'")
        
    def lightsOff(self):
        bash_call("ssh pi@192.168.1.106 'python3 ~/rush/energienie.py 2=off && python3 ~/rush/energienie.py 2=off'")
        
    def youtubePlay(self):
        bash_call("ssh pi@192.168.1.106 'python3 ~/rush/energienie.py 1=on && python3 ~/rush/energienie.py 1=on'")
        command = "ssh pi@192.168.1.106 'rm -f play.mp3 && youtube-dl --extract-audio --audio-format mp3 -o 'play.mp3' "
        command += context.args[0]
        command += " && omxplayer play.mp3 &'"
        bash_call(command)
          
        
    