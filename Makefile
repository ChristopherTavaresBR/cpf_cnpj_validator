.PHONY: dev prod build test

dev:
	docker-compose up dev

prod:
	docker-compose up prod

build:
	docker-compose build prod

test:
	docker run --rm -v $(pwd):/app -w /app rust cargo test