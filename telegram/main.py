from telegram.ext import Updater
import logging
from telegram.ext import CommandHandler
from telegram.ext import MessageHandler, Filters
from api_key import telegram_api_key
from fifoHandler import *
from threading import Thread

updater = Updater(token=telegram_api_key, use_context=True)

dispatcher = updater.dispatcher
logging.basicConfig(format='%(asctime)s - %(name)s - %(levelname)s - %(message)s',
                     level=logging.INFO)

updater.start_polling()


# /sv_brightness command
def sv_brightness(update, context):
    fifoWriter("sudo sh /home/luis/work/sideProjs/dotFiles/brightness.sh " + context.args[0])
    

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

thread = Thread(target = fifoReader, args = ())
thread.start()
# thread.join()

