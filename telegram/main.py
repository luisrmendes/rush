from bot_handler import *
from threading import Thread
from fifoHandler import * 

# /sv_brightness command
def sv_brightness(update, context):
    # fifoWriter("sudo sh /home/luis/github/dotFiles/brightness.sh " + context.args[0])
    os.system("sudo sh /home/luis/github/dotFiles/brightness.sh " + context.args[0])
    
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

# thread = Thread(target = fifoReader, args = ())
# thread.start()


