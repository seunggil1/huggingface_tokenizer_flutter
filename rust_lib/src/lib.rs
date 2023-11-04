use tokenizers::tokenizer::{Result, Tokenizer};

pub fn greet(to: String) -> String {
    format!("Hello, {}", to)
}


fn encode() -> Result<()> {
    // let tokenizer = Tokenizer::from_pretrained("TOKENIZER", None)?;
    let tokenizer = Tokenizer::from_file("./hu/tokenizer.json")?;

    let encoding = tokenizer.encode("안녕하세요 naver입니다", true)?;
    println!("{:?}", encoding.get_tokens());
    println!("{:?}", encoding.get_ids());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        let result = greet("World".to_owned());
        assert_eq!(result, "Hello, World".to_owned());
    }
}