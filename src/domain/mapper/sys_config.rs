use crate::domain::table::*;
crud!(SysConfig {});


impl_select_page!(SysConfig{select_page(dto: &crate::domain::dto::ConfigPageDTO) =>
    "`where 1=1`
    if dto.configName != '':
      ` and config_name like #{'%'+dto.configName+'%'}`
    if dto.configKey != '':
      ` and config_key like #{'%'+dto.configKey+'%'}`
    if dto.configType != '':
      ` and config_type = #{dto.configType}`
    if dto.status != '':
      ` and status = #{dto.status}`
    if !sql.contains('count'):
     ` order by create_time`"});
