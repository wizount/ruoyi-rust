use crate::domain::table::*;
crud!(SysLogininfor {});


impl_select_page!(SysLogininfor{select_page(dto: &crate::domain::dto::LogininforPageDTO) =>
    "``
      if !sql.contains('count'):
         ` order by login_time desc`"});
