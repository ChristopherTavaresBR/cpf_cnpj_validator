use regex::Regex;
use crate::models::{DocumentType, ValidationResponse};
use crate::anonymization::{anonymize_document, HashAlgorithm};

pub fn validate_document(
    doc: &str,
    show_ranges: Option<Vec<(usize, usize)>>,
    mask_char: Option<char>,
    hash_type: Option<&str>,
) -> Option<ValidationResponse> {
    let cleaned_doc = Regex::new(r"[^\d]").unwrap().replace_all(doc, "").to_string();
    let doc_type = DocumentType::from_length(cleaned_doc.len());
    
    let valid = match doc_type {
        DocumentType::Cpf => validate_cpf(&cleaned_doc),
        DocumentType::Cnpj => validate_cnpj(&cleaned_doc),
        DocumentType::Invalid => return None,
    };
    
    let hash_algo = HashAlgorithm::from(hash_type.unwrap_or("md5"));
    let (anonymized, anonymized_key, custom_anonymized) = anonymize_document(
        &cleaned_doc,
        doc_type,
        show_ranges,
        mask_char,
        hash_algo,
    );
    
    Some(ValidationResponse {
        valid,
        r#type: match doc_type {
            DocumentType::Cpf => "CPF".to_string(),
            DocumentType::Cnpj => "CNPJ".to_string(),
            DocumentType::Invalid => "INVALID".to_string(),
        },
        number: cleaned_doc.clone(),
        formatted: format_document(&cleaned_doc, doc_type),
        anonymized,
        anonymized_key,
        custom_anonymized,
        region: if matches!(doc_type, DocumentType::Cpf) {
            get_cpf_region(&cleaned_doc)
        } else {
            None
        },
    })
}

fn validate_cpf(cpf: &str) -> bool {
    if cpf.chars().all(|c| c == cpf.chars().next().unwrap()) {
        return false;
    }

    let digits: Vec<u32> = cpf.chars().filter_map(|c| c.to_digit(10)).collect();

    let first_digit = (0..9).map(|i| digits[i] * (10 - i as u32)).sum::<u32>() % 11;
    let first_digit = if first_digit < 2 { 0 } else { 11 - first_digit };

    let second_digit = (0..10).map(|i| digits[i] * (11 - i as u32)).sum::<u32>() % 11;
    let second_digit = if second_digit < 2 { 0 } else { 11 - second_digit };

    digits[9] == first_digit && digits[10] == second_digit
}

fn validate_cnpj(cnpj: &str) -> bool {
    let weights = [6, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
    let digits: Vec<u32> = cnpj.chars().filter_map(|c| c.to_digit(10)).collect();

    let first_digit = (0..12).map(|i| digits[i] * weights[i+1] as u32).sum::<u32>() % 11;
    let first_digit = if first_digit < 2 { 0 } else { 11 - first_digit };

    let second_digit = (0..13).map(|i| digits[i] * weights[i] as u32).sum::<u32>() % 11;
    let second_digit = if second_digit < 2 { 0 } else { 11 - second_digit };

    digits[12] == first_digit && digits[13] == second_digit
}

fn format_document(doc: &str, doc_type: DocumentType) -> String {
    match doc_type {
        DocumentType::Cpf => format!("{}.{}.{}-{}", &doc[0..3], &doc[3..6], &doc[6..9], &doc[9..11]),
        DocumentType::Cnpj => format!("{}.{}.{}/{}-{}", &doc[0..2], &doc[2..5], &doc[5..8], &doc[8..12], &doc[12..14]),
        DocumentType::Invalid => doc.to_string(),
    }
}

fn get_cpf_region(cpf: &str) -> Option<String> {
    let region_digit = cpf.chars().nth(8)?.to_digit(10)?;
    let region = match region_digit {
        0 => "RS",
        1 => "DF, GO, MS, MT ou TO",
        2 => "AC, AM, AP, PA, RO ou RR",
        3 => "CE, MA ou PI",
        4 => "AL, PB, PE ou RN",
        5 => "BA ou SE",
        6 => "MG",
        7 => "ES ou RJ",
        8 => "SP",
        9 => "PR ou SC",
        _ => return None,
    };
    Some(region.to_string())
}