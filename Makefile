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


.PHONY: build-macos
build-macos:
	cargo build --manifest-path=./pokecord-backend/Cargo.toml

.PHONY: run-macos
run-macos: build-macos
	cp ./pokecord-backend/target/debug/libpokecord_backend.dylib pokecord/pokecord_backend.so
	PYTHONPATH=. python3.9 -m pokecord