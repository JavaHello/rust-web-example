use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SignInVO {
    pub user_no: Option<String>,
    pub username: Option<String>,
    pub head_icon: Option<String>,
    pub token: Option<String>,
}

impl ToString for SignInVO {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
