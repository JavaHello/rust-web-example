use rbatis::plugin::logic_delete::RbatisLogicDeletePlugin;
use rbatis::rbatis::Rbatis;

// 示例-Rbatis示例初始化(必须)
lazy_static! {
    pub static ref RB: Rbatis = {
        let mut rb = Rbatis::new();
        let del = RbatisLogicDeletePlugin::new("del_state");
        rb.logic_plugin = Some(Box::new(del));
        rb
    };
}

#[tokio::main]
#[test]
async fn test_rbatis() {
    use crate::config::BOOT_CONFIG;
    use serde_json::json;

    RB.link(&BOOT_CONFIG.mysql_url).await.unwrap();
    let arg = &vec![json!(1)];
    let v: serde_json::Value = RB
        .fetch_prepare("", "SELECT count(1) FROM TM_USER where DEL_STATE = ?;", arg)
        .await
        .unwrap();
    println!("{}", v.to_string());
}
