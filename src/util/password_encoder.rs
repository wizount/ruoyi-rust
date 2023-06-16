use bcrypt::{DEFAULT_COST, hash, verify};

pub struct PasswordEncoder {}

impl PasswordEncoder {
    pub fn encode(raw_password: &str) -> String {
      hash(raw_password,DEFAULT_COST).unwrap()
    }
    pub fn verify(password: &str, raw_password: &str) -> bool {
        verify(raw_password, password).unwrap()
    }
}

#[cfg(test)]
mod test {
    use crate::util::password_encoder::PasswordEncoder;

    #[test]
    fn test_encode() {
        let s = PasswordEncoder::encode("123456");
        println!("{}", s);
        assert_eq!(
            PasswordEncoder::encode("123456"),
            PasswordEncoder::encode("123456")
        )
    }

    #[test]
    fn test_verify() {
        let password = "12345";
        let raw_password = "12345";

        assert!(PasswordEncoder::verify(password, raw_password));

        let encode_password = PasswordEncoder::encode(password);
        assert!(PasswordEncoder::verify(&encode_password, password));
    }
}
