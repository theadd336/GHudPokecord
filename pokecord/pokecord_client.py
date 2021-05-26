import discord
import logging
from discord.ext import commands
from pokecord.utils.starters import get_starter_pokemon_embeds

CLIENT_PREFIX = "p!"
bot = commands.Bot(CLIENT_PREFIX)

@bot.command(
    help="Prints info on how to get started with PokeBot",
    brief="Print info for new users",
)
async def info(ctx):
    logging.info(f"info command requested by {ctx.author}")
    info_message = "Welcome to the world of Pokemon! To get started, you must first choose a starter pokemon. Type `p!starters` to see the starters, and `p!choose <starter>` to choose a starter pokemon. Type `p!help` to see all bot commands."
    await ctx.send(info_message)

@bot.command(
    help="Prints the list of starter pokemon",
    brief="Print starting pokemon"
)
async def starters(ctx):
    logging.info(f"starters command requested by {ctx.author}")
    await get_starter_pokemon_embeds(ctx, bot)
