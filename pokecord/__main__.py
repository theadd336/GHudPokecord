import asyncio
import logging
import os
import sys
from asyncio.events import AbstractEventLoop
from asyncio.runners import _cancel_all_tasks
from pokecord import pokecord_backend
from pokecord.pokecord_client import bot

TOKEN = os.getenv("DISCORD_TOKEN")

async def main():
    root = logging.getLogger()
    root.setLevel(logging.DEBUG)

    handler = logging.StreamHandler(sys.stdout)
    handler.setLevel(logging.INFO)
    formatter = logging.Formatter('%(asctime)s - %(name)s - %(levelname)s - %(message)s')
    handler.setFormatter(formatter)
    root.addHandler(handler)

    pokecord_backend.test_logging()

    """Main entry point into the pokecord application."""
    a = pokecord_backend.registration.get_starter_pokemon_list()
    logging.info(a)
    b = await pokecord_backend.registration.register_player("HI")
    logging.info(b)
    await bot.start(TOKEN)

def shutdown_loop(loop: AbstractEventLoop):
    """Shuts down the running event loop. This code is duplicated from
    https://github.com/python/cpython/blob/3.9/Lib/asyncio/runners.py

    Parameters
    ----------
    loop : AbstractEventLoop
        A running asyncio event loop
    """
    try:
        _cancel_all_tasks(loop)
        loop.run_until_complete(loop.shutdown_asyncgens())
        loop.run_until_complete(loop.shutdown_default_executor())
    finally:
        asyncio.events.set_event_loop(None)
        loop.close()


if __name__ == "__main__":
    # NB: Due to PyO3's demands, the python event loop for asyncio is created
    # during the import of pokecord_backend. We then get the running loop
    # and use that as the main loop. Do not change these lines.
    loop = asyncio.get_event_loop()
    try:
        loop.run_until_complete(main())
    finally:
        shutdown_loop(loop)
