use md5::Digest;
use sha1::Sha1;
use sha2::{Sha256, Sha512};
use crate::models::DocumentType;

pub enum HashAlgorithm {
    Md5,
    Sha1,
    Sha256,
    Sha512,
    None,
}

impl From<&str> for HashAlgorithm {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "md5" => HashAlgorithm::Md5,
            "sha1" => HashAlgorithm::Sha1,
            "sha256" => HashAlgorithm::Sha256,
            "sha512" => HashAlgorithm::Sha512,
            _ => HashAlgorithm::None,
        }
    }
}

pub fn anonymize_document(
    doc: &str,
    doc_type: DocumentType,
    show_ranges: Option<Vec<(usize, usize)>>,
    mask_char: Option<char>,
    hash_algorithm: HashAlgorithm,
) -> (String, Option<String>, Option<String>) {
    let mask_char = mask_char.unwrap_or('*');
    let ranges = show_ranges.unwrap_or_default();
    
    // Gera a versão mascarada
    let masked: String = doc.chars()
        .enumerate()
        .map(|(i, c)| {
            if ranges.iter().any(|(start, end)| i >= *start && i <= *end) {
                c
            } else {
                mask_char
            }
        })
        .collect();
    
    // Gera a versão com hash (se solicitado)
    let hashed_key = match hash_algorithm {
        HashAlgorithm::None => None,
        HashAlgorithm::Md5 => {
            let mut hasher = md5::Md5::new();
            hasher.update(doc.as_bytes());
            Some(format!("{:x}", hasher.finalize()))
        },
        HashAlgorithm::Sha1 => {
            let mut hasher = Sha1::new();
            hasher.update(doc.as_bytes());
            Some(format!("{:x}", hasher.finalize()))
        },
        HashAlgorithm::Sha256 => {
            let mut hasher = Sha256::new();
            hasher.update(doc.as_bytes());
            Some(format!("{:x}", hasher.finalize()))
        },
        HashAlgorithm::Sha512 => {
            let mut hasher = Sha512::new();
            hasher.update(doc.as_bytes());
            Some(format!("{:x}", hasher.finalize()))
        },
    }.map(|hash| {
        // Prefixo baseado no tipo de documento
        let prefix = match doc_type {
            DocumentType::Cpf => "cpf",
            DocumentType::Cnpj => "cnpj",
            DocumentType::Invalid => "inv",
        };
        format!("{}_{}", prefix, hash)
    });
    
    // Formata o documento mascarado
    let formatted = match doc_type {
        DocumentType::Cpf => format!("{}.{}.{}-{}", &masked[0..3], &masked[3..6], &masked[6..9], &masked[9..11]),
        DocumentType::Cnpj => format!("{}.{}.{}/{}-{}", &masked[0..2], &masked[2..5], &masked[5..8], &masked[8..12], &masked[12..14]),
        DocumentType::Invalid => masked.clone(),
    };
    
    (formatted, hashed_key, Some(masked))
}