#!/bin/bash

cp /app/pokecord-backend/target/debug/libpokecord_backend.so /app/pokecord/pokecord_backend.so
python3.9 -m pokecord
