.PHONY: help setup relay-build relay-run mobile-setup mobile-run docker-up docker-down test clean

help:
	@echo "Commands:"
	@echo "  make setup          - Install all dependencies"
	@echo "  make relay-build    - Build Rust relay server"
	@echo "  make relay-run      - Run relay server locally"
	@echo "  make mobile-setup   - Setup Flutter mobile"
	@echo "  make mobile-run     - Run Flutter app"
	@echo "  make docker-up      - Start Docker services"
	@echo "  make docker-down    - Stop Docker services"
	@echo "  make test           - Run all tests"

setup:
	cd relay-server && cargo build --release
	cd mobile && flutter pub get

relay-build:
	cd relay-server && cargo build --release

relay-run:
	cd relay-server && cargo run

mobile-setup:
	cd mobile && flutter pub get

mobile-run:
	cd mobile && flutter run -d chrome

docker-up:
	docker-compose up -d

docker-down:
	docker-compose down

test:
	cd relay-server && cargo test
	cd mobile && flutter test

clean:
	cd relay-server && cargo clean
	cd mobile && flutter clean
	docker-compose down
