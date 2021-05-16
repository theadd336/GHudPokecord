import asyncio
from pokecord import pokecord_backend


async def main():
    """Main entry point into the pokecord application."""
    a = pokecord_backend.registration.get_starter_pokemon_list()
    print(a)
    b = await pokecord_backend.registration.register_player("HI")
    print(b)


if __name__ == "__main__":
    # NB: Due to PyO3's demands, the python event loop for asyncio is created
    # during the import of pokecord_backend. We then get the running loop
    # and use that as the main loop. Do not change these lines.
    loop = asyncio.get_event_loop()
    loop.run_until_complete(main())
