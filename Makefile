.PHONY: all
all:
	docker-compose -f docker-compose.yaml up

.PHONY: build
build:
	cargo build --manifest-path=/app/pokecord-backend/Cargo.toml
	. scripts/start_pokecord_discord.sh

.PHONY: run
run:
	python3.9 -m pokecord
