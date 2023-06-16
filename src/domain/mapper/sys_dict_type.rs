use crate::domain::table::*;
crud!(SysDictType {});

impl_select_page!(SysDictType{select_page(dto: &crate::domain::dto::DictTypePageDTO) =>
    "`where 1=1 `
    if dto.dictType != '':
      ` and dict_type like #{'%'+dto.dictType+'%'}`
    if dto.dictName != '':
      ` and dict_name like #{'%'+dto.dictName+'%'}`
    if dto.status != '':
      ` and status = #{dto.status}`
    if !sql.contains('count'):
     ` order by create_time`"});
