use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationResponse {
    pub valid: bool,
    pub r#type: String,
    pub number: String,
    pub formatted: String,
    pub anonymized: String,
    pub anonymized_key: Option<String>,  // Agora é opcional
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_anonymized: Option<String>,
    pub region: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ValidateParams {
    pub doc: String,
    #[serde(default)]
    pub show: Option<String>,
    #[serde(default)]
    pub mask: Option<char>,
    #[serde(default)]
    pub hash_type: Option<String>,  // Novo campo para escolher o algoritmo
}

#[derive(Debug, Deserialize)]
pub struct BulkRequest {
    pub documents: Vec<String>,
    pub offset: Option<usize>,
    pub limit: Option<usize>,
    #[serde(default)]
    pub hash_type: Option<String>,  // Novo campo para escolher o algoritmo
}

#[derive(Debug, Serialize)]
pub struct BulkResponse {
    pub valid: bool,
    pub results: Vec<ValidationResponse>,
    pub pagination: Pagination,
}

#[derive(Debug, Serialize)]
pub struct Pagination {
    pub total: usize,
    pub offset: usize,
    pub limit: usize,
    pub next_offset: Option<usize>,
}

#[derive(Debug, Clone, Copy)]
pub enum DocumentType {
    Cpf,
    Cnpj,
    Invalid,
}

impl DocumentType {
    pub fn from_length(len: usize) -> Self {
        match len {
            11 => DocumentType::Cpf,
            14 => DocumentType::Cnpj,
            _ => DocumentType::Invalid,
        }
    }
}