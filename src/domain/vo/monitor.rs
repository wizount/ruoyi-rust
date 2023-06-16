use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};
/*
服务器VO
 */
#[derive(Clone, Debug, Deserialize,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerVO {
    pub cpu: CpuVO,
    pub mem: Memory,
    pub sys: SysVO,
    pub sys_files: Vec<DiskVO>,
}

#[derive(Clone, Debug, Deserialize,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CpuVO {
    //CPU核心数
    pub cpu_num: u8,
    //空闲率
    pub free: f32,
    //系统使用率
    pub sys: f32,
    pub total: u64,
    //用户使用率
    pub used: f32,
    pub wait: f32,
}


#[derive(Clone, Debug, Deserialize,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Memory {
    //空闲率
    pub free: String,
    //内存大小（G）
    pub total: String,
    //已用
    pub used: String,
    //用户使用率
    pub usage: String,
}

#[derive(Clone, Debug, Deserialize,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SysVO {
    //IP地址
    pub computer_ip: String,
    //计算机名称
    pub computer_name: String,
    //64位还是32位
    pub os_arch: String,
    //操作系统名称
    pub os_name: String,
}

#[derive(Clone, Debug, Deserialize,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DiskVO {
    //盘符
    pub dir_name: String,
    //剩下空间
    pub free: String,
    //磁盘格式
    pub sys_type_name: String,
    //磁盘名称
    pub type_name: String,
    //总容量
    pub total: String,
    //
    //使用空间
    pub used: String,
    //使用率
    pub usage: String,
}

//在线用户
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SysUserOnlineVO {
    pub token_id: Option<String>,
    pub dept_name: Option<String>,
    pub user_name: Option<String>,
    pub ipaddr: Option<String>,
    pub login_location: Option<String>,
    pub phonenumber: Option<String>,
    pub browser: Option<String>,
    pub os: Option<String>,
    pub login_time: Option<DateTime>
}
