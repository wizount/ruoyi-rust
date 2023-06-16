use crate::domain::table::{SysConfig};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SysConfigVO {
    pub config_id: Option<String>,
    pub config_name: Option<String>,
    pub config_key: Option<String>,
    pub config_value: Option<String>,
    pub config_type: Option<char>,
    pub remark: Option<String>,
}

impl From<SysConfig> for SysConfigVO {
    fn from(arg: SysConfig) -> Self {
        Self {
            config_id: arg.config_id,
            config_name: arg.config_name,
            config_key: arg.config_key,
            config_value: arg.config_value,
            config_type: arg.config_type,
            remark: arg.remark,
        }
    }
}
