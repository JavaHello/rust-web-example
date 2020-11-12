pub struct PasswordEncoder {}

impl PasswordEncoder {
    pub fn encode(raw_password: &str, salt: &str) -> String {
        let digest = md5::compute(format!("{}-{}", raw_password, salt));
        format!("{:x}", digest)
    }
    pub fn verify(password: &str, raw_password: &str, salt: &str) -> bool {
        let hashed = PasswordEncoder::encode(raw_password, salt);
        password.eq(&hashed)
    }
}

#[test]
fn test_encode() {
    let s = PasswordEncoder::encode("123456", "123");
    println!("{}", s);
    assert_eq!(
        PasswordEncoder::encode("123456", "123"),
        PasswordEncoder::encode("123456", "123")
    )
}
