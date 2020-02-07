from bot_handler import updater
from threading import Thread

# Start bot
updater.start_polling()

# thread = Thread(target = fifoReader, args = ())
# thread.start()