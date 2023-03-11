use crate::config::BOOT_CONFIG;
use rbatis::rbatis::Rbatis;

// 示例-Rbatis示例初始化(必须)
lazy_static! {
    pub static ref RB: Rbatis = {
        let rb = Rbatis::new();
        #[cfg(feature = "sqlite")]
        {
            use rbdc_sqlite::driver::SqliteDriver;
            rb.init(SqliteDriver {}, &BOOT_CONFIG.mysql_url).unwrap();
        }
        #[cfg(feature = "mysql")]
        {
            use rbdc_mysql::driver::MysqlDriver;
            rb.init(MysqlDriver {}, &BOOT_CONFIG.mysql_url).unwrap();
        }
        rb
    };
}
