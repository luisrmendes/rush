# bot.py
import os
import random

from discord.ext import commands
from dotenv import load_dotenv

import subprocess
import time

load_dotenv()
token = os.getenv('DISCORD_TOKEN')

bot = commands.Bot(command_prefix='!')

@bot.command(name='99')
async def nine_nine(ctx):
    brooklyn_99_quotes = [
        'I\'m the human form of the 💯 emoji.',
        'Bingpot!',
        (
            'Cool. Cool cool cool cool cool cool cool, '
            'no doubt no doubt no doubt no doubt.'
        ),
    ]

    response = random.choice(brooklyn_99_quotes)
    await ctx.send(response)

@bot.command(name='roll_dice')
async def roll(ctx, number_of_dice, number_of_sides):
    dice = [
        str(random.choice(range(1, number_of_sides + 1)))
        for _ in range(number_of_dice)
    ]
    await ctx.send(', '.join(dice))

@bot.command(name='start', help='Starts server')
async def start(ctx):
    # bashCommand = "minecraft"
    # process = subprocess.Popen(bashCommand.split(), stdout=subprocess.PIPE)
    # output, error = process.communicate()
    # os.system("ssh -t luis@192.168.1.64 'cd minecraft; tmux new  -d -s minecraft-session 'sudo sh minecraft_run.sh''")
    os.system("cd ~/RAD/Serverpack/1.36; tmux new -d -s minecraft 'sh LaunchServer.sh'")
    #time.sleep(70)
    print("Its probabily on!")

bot.run(token)