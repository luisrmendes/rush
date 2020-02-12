from bot_handler import updater
from threading import Thread
from system import *

# Start telegram bot
updater.start_polling()

# thread = Thread(target = fifoReader, args = ())
# thread.start()