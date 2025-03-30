# Build stage (usa Rust + Cargo pra compilar)
FROM rust:1.70-slim as builder

WORKDIR /app
COPY . .

# Instala dependências de build e compila em release
RUN apt-get update && apt-get install -y pkg-config libssl-dev && \
    cargo build --release

# Runtime stage (imagem final minimalista)
FROM debian:bookworm-slim

WORKDIR /app
COPY --from=builder /app/target/release/cpf-cnpj-validator .

# Expõe a porta do microsservice
EXPOSE 3030

# Roda o serviço
CMD ["./cpf-cnpj-validator"]