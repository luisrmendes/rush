from telegram.ext import Updater
import logging
from telegram.ext import CommandHandler
from telegram.ext import MessageHandler, Filters
from api_key import telegram_api_key
from system import bash_call


updater = Updater(token=telegram_api_key, use_context=True)

dispatcher = updater.dispatcher
logging.basicConfig(format='%(asctime)s - %(name)s - %(levelname)s - %(message)s',
                     level=logging.INFO)


# /sv_brightness command
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