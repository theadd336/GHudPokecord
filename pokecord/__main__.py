import os
from pokecord.PokecordClient import bot

TOKEN = os.getenv("DISCORD_TOKEN")

bot.run(TOKEN)