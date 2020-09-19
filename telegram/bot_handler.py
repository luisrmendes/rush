# coding=utf-8

from telegram.ext import Updater, CommandHandler, MessageHandler, Filters
import logging
from api_key import telegram_api_key
from psw import psw
from system import bash_call


updater = Updater(token=telegram_api_key, use_context=True)

dispatcher = updater.dispatcher
logging.basicConfig(format='%(asctime)s - %(name)s - %(levelname)s - %(message)s',
                     level=logging.INFO)


# /speakersOn command
def speakers_on(update, context):
    result = bash_call("ssh pi@192.168.1.106 'python3 ~/rush/energienie.py 1=on && python3 ~/rush/energienie.py 1=on'")
    updater.bot.send_message(chat_id=update.effective_chat.id, text=result)
    
speakers_on_handler = CommandHandler('speakers_on', speakers_on)
dispatcher.add_handler(speakers_on_handler)

# /speakers_off command
def speakers_off(update, context):
    result = bash_call("ssh pi@192.168.1.106 'python3 ~/rush/energienie.py 1=off && python3 ~/rush/energienie.py 1=off'")
    updater.bot.send_message(chat_id=update.effective_chat.id, text=result)
    
speakers_off_handler = CommandHandler('speakers_off', speakers_off)
dispatcher.add_handler(speakers_off_handler)

# /lights_off command
def lights_off(update, context):
    result = bash_call("ssh pi@192.168.1.106 'python3 ~/rush/energienie.py 2=off && python3 ~/rush/energienie.py 2=off'")
    updater.bot.send_message(chat_id=update.effective_chat.id, text=result)
    
lights_off_handler = CommandHandler('lights_off', lights_off)
dispatcher.add_handler(lights_off_handler)

# /lights_on command
def lights_on(update, context):
    result = bash_call("ssh pi@192.168.1.106 'python3 ~/rush/energienie.py 2=on && python3 ~/rush/energienie.py 2=on'")
    updater.bot.send_message(chat_id=update.effective_chat.id, text=result)
    
lights_on_handler = CommandHandler('lights_on', lights_on)
dispatcher.add_handler(lights_on_handler)

# /rpi_play command
# youtube-dl --extract-audio --audio-format mp3 <link> https:/&& omxplayer Rush\ -\ Clockwork\ Angels\ Tour\ -\ The\ Garden-EsBNzf5JlZA.mkv
def youtube_play(update, context):
    result = bash_call("ssh pi@192.168.1.106 'omxplayer ~/rush/wakeup_songs/03.\ Lakeside\ Park.mp3'")
    updater.bot.send_message(chat_id=update.effective_chat.id, text=result)
    
youtube_play_handler = CommandHandler('youtube_play', youtube_play)
dispatcher.add_handler(youtube_play_handler)

# /shutdown command
def shutdown_desktop(update, context):
    result = bash_call("net rpc shutdown -f -t 0 -C 'Bye Bye, says server' -U luis%" + psw + " -I 192.168.1.71")
    updater.bot.send_message(chat_id=update.effective_chat.id, text=result)
    
shutdown_handler = CommandHandler('shutdown_desktop', shutdown_desktop)
dispatcher.add_handler(shutdown_handler)

# /status command
def status(update, context):
    result = bash_call("ping -c 1 192.168.1.71 2>&1 >/dev/null ; echo $?")
    result = result.replace("\n", '')
    if result == "0":   
        send = "Desktop is online"
    else:
        send = "Desktop is offline"
    updater.bot.send_message(chat_id=update.effective_chat.id, text=send)
    
status_handler = CommandHandler('status', status)
dispatcher.add_handler(status_handler)

# /ipv6 command
def ipv6(update, context):
    result = bash_call("dig @resolver1.opendns.com AAAA myip.opendns.com +short -6")
    updater.bot.send_message(chat_id=update.effective_chat.id, text=result)
    
ipv6_handler = CommandHandler('ipv6', ipv6)
dispatcher.add_handler(ipv6_handler)

# /ipv4 command
def ipv4(update, context):
    result = bash_call("dig @resolver1.opendns.com A myip.opendns.com +short -4")
    updater.bot.send_message(chat_id=update.effective_chat.id, text=result)
    
ipv4_handler = CommandHandler('ipv4', ipv4)
dispatcher.add_handler(ipv4_handler)

# /desktop_wakeup command
def desktop_wakeup(update, context):
    result = bash_call("wakeonlan 00:D8:61:a1:CE:00")
    updater.bot.send_message(chat_id=update.effective_chat.id, text=result)
    
desktop_wakeup_handler = CommandHandler('desktop_wakeup', desktop_wakeup)
dispatcher.add_handler(desktop_wakeup_handler)

# /sv_brightness command
def sv_brightness(update, context):
    result = bash_call("sudo sh /home/luis/github/dotFiles/brightness.sh " + context.args[0])
    updater.bot.send_message(chat_id=update.effective_chat.id, text=result)
    
sv_brightness_handler = CommandHandler('sv_brightness', sv_brightness)
dispatcher.add_handler(sv_brightness_handler)


# Response to text 
# def echo(update, context):
#     context.bot.send_message(chat_id=update.effective_chat.id, text=update.message.text)

# echo_handler = MessageHandler(Filters.text, echo)
# dispatcher.add_handler(echo_handler)


def ambrosio(update, context):
    if update.message.text == "Ambrosio" or update.message.text == "Ambrósio" :
        context.bot.send_message(chat_id=update.effective_chat.id, text="Diga senhora")
 
ambrosio_handler = MessageHandler(Filters.text, ambrosio)
dispatcher.add_handler(ambrosio_handler)