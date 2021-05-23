import discord
from discord.ext import commands
from pokecord.utils.starters import get_starter_pokemon_embeds

CLIENT_PREFIX = "p!"
bot = commands.Bot(CLIENT_PREFIX)

@bot.command(
    help="Prints info on how to get started with PokeBot",
    brief="Print info for new users",
)
async def info(ctx):
    print(f"info command requested by {ctx.author}")
    info_message = "Welcome to the world of Pokemon! To get started, you must first choose a starter pokemon. Type `p!starters` to see the starters, and `p!choose <starter>` to choose a starter pokemon. Type `p!help` to see all bot commands."
    await ctx.send(info_message)

@bot.command(
    help="Prints the list of starter pokemon",
    brief="Print starting pokemon"
)
async def starters(ctx):
    print(f"starters command requested by {ctx.author}")
    starters_message = "starters view"
    embeds = get_starter_pokemon_embeds()
    await ctx.send(embed=embeds[0])
    await ctx.send(embed=embeds[1])

@bot.command()
async def embedpages(ctx):
    print(f"embed pages requested")
    page1 = discord.Embed(
        title="Page 1/2",
        description="description",
        color=discord.Color.from_rgb(255, 0, 255),
    )
    page2 = discord.Embed(
        title="Page 2/2",
        description="description",
        color=discord.Color.orange(),
    )
    page3 = discord.Embed(
        title="Page 3/2",
        description="description",
        color=discord.Color.red(),
    )

    pages = [page1, page2, page3]

    message = await ctx.send(embed=page1)
    await message.add_reaction('◀')
    await message.add_reaction('▶')

    def check(reaction, user):
        print("checking")
        return user == ctx.author

    i = 0
    reaction = None
    print("HERE!")
    while True:
        print("IN LOOP!")
        if str(reaction) == '◀':
            if i > 0:
                i -= 1
                await message.edit(embed = pages[i])
        elif str(reaction) == '▶':
            if i < 2:
                i += 1
                await message.edit(embed = pages[i])
        
        try:
            print("WAITING!")
            reaction, user = await bot.wait_for('reaction_add', timeout = 30.0, check = check)
            print(f"Reaction: {reaction}, User: {user}")
            await message.remove_reaction(reaction, user)
            print("REMOVED!")
        except Exception as e:
            print(e)
            break
