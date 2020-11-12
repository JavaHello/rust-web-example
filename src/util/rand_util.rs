use rand;
use rand::Rng;
pub struct RandUtil {}

impl RandUtil {
    /// 生成随机数字串
    pub fn rand_code(len: usize) -> String {
        let mut rng = rand::thread_rng();
        let mut r = String::new();
        for _ in 0..len {
            let c = rng.gen_range(0, 9);
            r.push_str(&c.to_string());
        }
        r
    }

    /// 生成随机随机字母
    pub fn rand_abc_code(len: usize) -> String {
        let mut rng = rand::thread_rng();
        let mut r = String::new();
        for _ in 0..len {
            // 65 -> A
            // 97 -> a
            let c: u8 = rng.gen_range(65, 65 + 26);
            r.push(c as char);
        }
        r
    }
}
#[test]
fn test_rand() {
    let r = RandUtil::rand_code(10);
    println!("{}", r);
    assert_eq!(10, r.len());

    let r = RandUtil::rand_abc_code(10);
    println!("{}", r);
    assert_eq!(10, r.len());
}
