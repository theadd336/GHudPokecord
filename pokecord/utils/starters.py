import discord
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

def get_starter_pokemon_embeds():
    # STEP 1 - get starter pokemon for all generations 
    starter_list = [("Charmander", "fire", "https://assets.pokemon.com/assets/cms2/img/pokedex/full/004.png"), ("Cyndaquil", "fire", "https://assets.pokemon.com/assets/cms2/img/pokedex/full/155.png")]
    # STEP 2 - create embed for showing each pokemon
    pokemon_embeds = [_create_starter_pokemon_embed(pokemon) for pokemon in starter_list]
    return pokemon_embeds
    # STEP 3 - create pages of pokemon, one page for each generation. This assume that the get starter pokemon returns exactly 24 pokemon
    # For starters, each page will have 3 pokemon and there will be 8 pages total