use tokenizers::tokenizer::{Tokenizer};

pub fn encode(input: String) -> Vec<u32> {
    let tokenizer_result = Tokenizer::from_file("./tokenizer.json")
        .and_then(|tokenizer| tokenizer.encode(input, true))
        .map(|encoding| encoding.get_ids().to_vec());

    match tokenizer_result {
        Ok(tokens) => tokens,
        Err(_) => Vec::new(), // 에러 발생 시 빈 벡터 반환
    }
}