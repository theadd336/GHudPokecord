#!/bin/bash

cp /app/pokecord-backend/target/debug/libpokecord_backend.so /app/pokecord-discord/pokecord_backend.so
python3.7 -m pokecord-discord
