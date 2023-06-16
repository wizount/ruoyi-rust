use crate::domain::table::*;
crud!(SysDictData {});


impl_select_page!(SysDictData{select_page(dto: &crate::domain::dto::DictDataPageDTO) =>
    "`where 1=1 `
    if dto.dictType != '':
      ` and dict_type = #{dto.dictType}`
    if dto.dictLabel != '':
      ` and dict_label like #{'%'+dto.dictLabel+'%'}`
    if dto.status != '':
      ` and status = #{dto.status}`
    if !sql.contains('count'):
     ` order by dict_sort`"});
impl_select!(SysDictData{select_by_dict_type(dict_type:&String)=>"`where dict_type =#{dict_type} order by dict_sort`"});
