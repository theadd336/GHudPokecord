import discord
from pokecord.utils.paged_message import PagedMessage
from pokecord.constants.types import TYPE_TO_COLOR

def _create_starter_pokemon_embed(pokemon):
    # TODO: Use actual pokemon object
    color = TYPE_TO_COLOR[pokemon[1]]
    embed = discord.Embed(
        title=pokemon[0],
        color=discord.Color.from_rgb(color[0], color[1], color[2]),
    )
    embed.set_image(url=pokemon[2])
    return embed

async def get_starter_pokemon_embeds(ctx, bot):
    # STEP 1 - get starter pokemon for all generations 
    # TODO: get data from rust API
    starter_list = [
        ("Charmander", "fire", "https://assets.pokemon.com/assets/cms2/img/pokedex/full/004.png"),
        ("Squirtle", "water", "https://assets.pokemon.com/assets/cms2/img/pokedex/full/004.png"),
        ("Totodile", "water", "https://assets.pokemon.com/assets/cms2/img/pokedex/full/004.png"),
        ("Cyndaquil", "fire", "https://assets.pokemon.com/assets/cms2/img/pokedex/full/155.png"),
        ("Charmander", "fire", "https://assets.pokemon.com/assets/cms2/img/pokedex/full/004.png"),
        ("Squirtle", "water", "https://assets.pokemon.com/assets/cms2/img/pokedex/full/004.png"),
        ("Totodile", "water", "https://assets.pokemon.com/assets/cms2/img/pokedex/full/004.png"),
        ("Cyndaquil", "fire", "https://assets.pokemon.com/assets/cms2/img/pokedex/full/155.png"),
    ]
    # STEP 2 - create embed for showing each pokemon
    pokemon_embeds = [_create_starter_pokemon_embed(pokemon) for pokemon in starter_list]

    # STEP 3 - send a paged message for showing the starters
    starter_message = PagedMessage(ctx, bot, pokemon_embeds, 3)
    await starter_message.send_message()