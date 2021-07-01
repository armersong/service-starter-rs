use uuid::Uuid;

pub struct Token {}

impl Token {
    pub fn make_token<T: AsRef<str>>(key: T) -> String {
        let s = format!("{}{}", key.as_ref(), Uuid::new_v4());
        let digest = md5::compute(s.as_bytes());
        format!("{:x}", digest).to_uppercase()
    }
}

#[cfg(test)]
mod tests {
    use crate::service::token::Token;

    #[test]
    fn test_token() {
        let s = "this is ok";
        let s1 = Token::make_token(s);
        let s2 = Token::make_token(s);
        println!("key {} s1 {} s2 {}", s, s1, s2);
        assert_ne!(s1, s2);
        assert_eq!(Token::make_token("").len(), 32);
        assert_eq!(Token::make_token("1111222233334444555566667777888899990000").len(), 32);
    }
}
