use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Sms {
    pub user_name: String,
    pub args: HashMap<String, String>,
}
