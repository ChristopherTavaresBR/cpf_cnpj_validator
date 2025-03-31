# CPF AND CNPJ VALIDATOR (BRAZILIAN DOCUMENT FOR PEOPLE OR EMPLOYE)

## Project Structure

```bash
src/
├── main.rs          # Routes
├── models.rs        # Structs and Types
├── validation.rs    # Validation functions
└── anonymization.rs # LGPD Anonym functions
``` 

# CPF & CNPJ Validator (Rust)

This is a high-performance microservice built in **Rust** for validating Brazilian CPF (Individual Taxpayer Registry) and CNPJ (National Registry of Legal Entities). The service supports both **single document validation** via GET requests and **batch validation** via POST requests.

## Features
- Validate CPF and CNPJ numbers.
- Identify if a document is valid.
- Return formatted, anonymized, and hashed versions of the document.
- Bulk validation via JSON API.
- Fast and lightweight using **Rust**.
- Containerized with **Docker**.

## API Endpoints

### **1. Validate a single CPF/CNPJ**
```http
GET http://localhost:8080/validate?doc=12345678909&show=3-8&mask=*
```
#### **Response**
```json
{
  "valid": true,
  "type": "CPF",
  "number": "12345678909",
  "formatted": "123.456.789-09",
  "anonymized": "***.456.789-**",
  "anonymized_key": "cpf_79b5f54918628cc7f6a900a386d14a04",
  "custom_anonymized": "***456789**",
  "region": "PR or SC"
}
```

### **2. Bulk CPF/CNPJ Validation**
```sh
curl -X POST http://localhost:3030/validate/bulk \
  -H "Content-Type: application/json" \
  -d '{
    "documents": [
      "529.982.247-25",    # Valid CPF (SP)
      "111.111.111-11"      # Invalid CPF (all digits equal)
    ]
  }'
```

## How CPF & CNPJ Validation Works
### **CPF Validation**
CPF consists of **11 digits**. The last two digits are **verification digits**, calculated using a modulus-based formula:
1. Multiply the first 9 digits by weights (10 to 2), sum the results, and get the remainder when divided by 11.
2. Subtract the remainder from 11 (if result is 10 or 11, digit becomes 0).
3. Repeat for the second verification digit using weights (11 to 2).
4. If the two calculated digits match the CPF's last two digits, it is valid.

### **CNPJ Validation**
CNPJ consists of **14 digits**, with the last two being **verification digits**:
1. Multiply the first 12 digits by specific weights, sum the results, and calculate the remainder when divided by 11.
2. Apply the same rule as CPF for digit calculation.
3. Repeat the process for the second digit.
4. If both calculated digits match the given CNPJ, it is valid.

## Deploying with Docker

### **Building the Image**
```sh
docker build -t cpf-cnpj-validator .
```

### **Running the Container**
```sh
docker run -p 8080:3030 cpf-cnpj-validator
```

### **Dockerfile**
```dockerfile
# Build stage (Rust compilation)
FROM rust:1.81-bookworm as builder
WORKDIR /app
COPY . .

# Install dependencies and compile in release mode
RUN apt-get update && apt-get install -y pkg-config libssl-dev && \
    cargo build --release && \
    test -f target/release/cpf-cnpj-validator && strip target/release/cpf-cnpj-validator

# Runtime stage (minimal image)
FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y libssl-dev && \
    apt-get clean && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/cpf-cnpj-validator .
RUN useradd -m appuser
USER appuser
EXPOSE 3030
CMD ["./cpf-cnpj-validator"]
```

## Kubernetes Deployment

### **Deployment File (k8s/deployment.yml)**
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: cpf-cnpj-validator
spec:
  replicas: 3
  selector:
    matchLabels:
      app: cpf-cnpj-validator
  template:
    metadata:
      labels:
        app: cpf-cnpj-validator
    spec:
      containers:
        - name: cpf-cnpj-validator
          image: your-docker-hub-username/cpf-cnpj-validator:latest
          ports:
            - containerPort: 3030
```

## License
MIT License

