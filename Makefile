.PHONY: run test build docker-build certs

# Avvia il proxy in modalità sviluppo
run:
	RUST_LOG=debug cargo run

# Esegue tutti i test
test:
	cargo test

# Build release
build:
	cargo build --release

# Costruisce l'immagine Docker
docker-build:
	docker build -t shield-proxy:local .

# Genera certificati self-signed per test HTTPS
certs:
	mkdir -p config/certs
	openssl req -x509 -newkey rsa:4096 -keyout config/certs/key.pem -out config/certs/cert.pem -days 365 -nodes -subj "/CN=localhost"