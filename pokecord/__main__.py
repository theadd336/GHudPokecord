import asyncio
from asyncio.events import AbstractEventLoop

from asyncio.runners import _cancel_all_tasks
from pokecord import pokecord_backend


async def main():
    """Main entry point into the pokecord application."""
    a = pokecord_backend.registration.get_starter_pokemon_list()
    print(a)
    b = await pokecord_backend.registration.register_player("HI")
    print(b)

    pokemon = await pokecord_backend.list_pokemon()
    print("Some pokemon:")
    for name in pokemon[:20]:
        print(f"- {name}")


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
