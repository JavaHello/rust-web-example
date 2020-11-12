use config::Config;
use dotenv::dotenv;
use serde::Deserialize;

///服务启动配置
#[derive(Debug, Deserialize)]
pub struct BootConfig {
    ///当前服务地址
    pub server_url: String,
    ///日志路径
    pub log_conf: String,
    ///redis地址
    pub redis_url: String,
    ///mysql地址
    pub mysql_url: String,
    /// jwt secret
    pub jwt_secret: String,
}

///默认配置
impl Default for BootConfig {
    fn default() -> Self {
        Self {
            server_url: "127.0.0.1:8000".to_owned(),
            log_conf: "config/log4rs.yaml".to_owned(),
            redis_url: "redis://127.0.0.1:6379".to_owned(),
            mysql_url: "mysql://root:123456@localhost:3306/demodb".to_owned(),
            jwt_secret: "923123456".to_owned(),
        }
    }
}

impl BootConfig {
    pub fn init() -> Self {
        dotenv().ok();
        let mut c = Config::new();
        c.merge(config::Environment::default())
            .expect("加载配置失败!!!");
        c.try_into().expect("init env error")
    }
}
