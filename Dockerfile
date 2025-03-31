# Construção em múltiplos estágios para um micro serviço Rust de validação de CPF/CNPJ

# Estágio de build - usando Rust 1.81
FROM rust:1.81 as builder

WORKDIR /usr/src/cpf-cnpj-validator

# Criar projeto vazio para aproveitar o cache de dependências
RUN cargo new --bin cpf-cnpj-validator
WORKDIR /usr/src/cpf-cnpj-validator/cpf-cnpj-validator

# Copiar arquivos de dependências
COPY Cargo.toml Cargo.lock ./

# Construir as dependências para cache
RUN cargo build --release
RUN rm src/*.rs

# Copiar código-fonte real
COPY src ./src

# Forçar recompilação com código real
RUN touch src/main.rs
RUN cargo build --release

# Estágio de produção
FROM debian:bookworm-slim

# Instalar dependências mínimas
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates curl \
    && rm -rf /var/lib/apt/lists/*

# Criar um usuário não-root para executar o serviço
RUN useradd -ms /bin/bash appuser

# Copiar o binário compilado do estágio de construção
COPY --from=builder /usr/src/cpf-cnpj-validator/cpf-cnpj-validator/target/release/cpf-cnpj-validator /usr/local/bin/

# Configurar variáveis de ambiente (valores padrão)
ENV RUST_LOG=info
ENV HOST=0.0.0.0
ENV PORT=8080

# Expor a porta definida por padrão
# Nota: se você mudar a PORT em tempo de execução, precisará mapear essa porta manualmente no docker run
EXPOSE ${PORT}

# Mudar para o usuário não-root
USER appuser

# Verificar se o serviço está funcionando (usando a variável PORT)
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD curl -f http://${HOST}:${PORT}/health || exit 1

# Comando para iniciar o serviço
CMD ["cpf-cnpj-validator"]