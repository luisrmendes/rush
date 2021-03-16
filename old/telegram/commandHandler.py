# coding=utf-8

from telegram.ext import Updater, CommandHandler, MessageHandler, Filters
import logging
from aux import *
from Rpi import Rpi
from Server import Server
from Desktop import Desktop

import os
from os.path import join, dirname
from dotenv import load_dotenv

dotenv_path = join(dirname(__file__), '.env')
load_dotenv(dotenv_path)


updater = Updater(token=os.getenv('TELEGRAM_API'), use_context=True)

dispatcher = updater.dispatcher
logging.basicConfig(format='%(asctime)s - %(name)s - %(levelname)s - %(message)s',
                     level=logging.INFO)


rpi = Rpi()
desktop = Desktop()
server = Server()

# /speakersOn command
def speakers_on(update, context):
    result = rpi.speakersOn()
    updater.bot.send_message(chat_id=update.effective_chat.id, text=result)
    
dispatcher.add_handler(CommandHandler('speakers_on', speakers_on))

# /speakers_off command
def speakers_off(update, context):
    result = rpi.speakersOff()
    updater.bot.send_message(chat_id=update.effective_chat.id, text=result)
    
dispatcher.add_handler(CommandHandler('speakers_off', speakers_off))

# /lights_off command
def lights_off(update, context):
    result = rpi.lightsOff()
    updater.bot.send_message(chat_id=update.effective_chat.id, text=result)
     
dispatcher.add_handler(CommandHandler('lights_off', lights_off))

# /lights_on command
def lights_on(update, context):
    result = rpi.lightsOn()
    updater.bot.send_message(chat_id=update.effective_chat.id, text=result)

dispatcher.add_handler(CommandHandler('lights_on', lights_on))

# /rpi_play command
def youtube_play(update, context):
    result = rpi.youtubePlay(context)
    updater.bot.send_message(chat_id=update.effective_chat.id, text=result)
    
dispatcher.add_handler(CommandHandler('youtube_play', youtube_play))

# /shutdown command
def shutdown_desktop(update, context):
    result = desktop.shutdownDesktop()
    updater.bot.send_message(chat_id=update.effective_chat.id, text=result)
    
dispatcher.add_handler(CommandHandler('shutdown_desktop', shutdown_desktop))

# /status command
def status(update, context):
    result = desktop.getDesktopStatus()    
    updater.bot.send_message(chat_id=update.effective_chat.id, text=result)
    
dispatcher.add_handler(CommandHandler('status', status))

# /desktop_wakeup command
def desktop_wakeup(update, context):
    result = desktop.wakeupDesktop()
    updater.bot.send_message(chat_id=update.effective_chat.id, text=result)
    
dispatcher.add_handler(CommandHandler('desktop_wakeup', desktop_wakeup))

# /ipv6 command
def ipv6(update, context):
    result = server.getIpv6()
    updater.bot.send_message(chat_id=update.effective_chat.id, text=result)
    
dispatcher.add_handler(CommandHandler('ipv6', ipv6))

# /ipv4 command
def ipv4(update, context):
    result = server.getIpv4()
    updater.bot.send_message(chat_id=update.effective_chat.id, text=result)
    
dispatcher.add_handler(CommandHandler('ipv4', ipv4))

# /sv_brightness command
def sv_brightness(update, context):
    result = server.serverBrightness(context)
    updater.bot.send_message(chat_id=update.effective_chat.id, text=result)
    
dispatcher.add_handler(CommandHandler('sv_brightness', sv_brightness))


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