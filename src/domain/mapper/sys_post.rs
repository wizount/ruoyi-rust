use crate::domain::table::*;
crud!(SysPost {});


impl_select_page!(SysPost{select_page(dto: &crate::domain::dto::PostPageDTO) =>
    "`where 1=1 `
    if dto.postName != '':
      ` and post_name like #{'%'+dto.postName+'%'}`
    if dto.postCode != '':
      ` and post_code like #{'%'+dto.postCode+'%'}`
    if dto.status != '':
      ` and status = #{dto.status}`
    if !sql.contains('count'):
     ` order by post_sort`"});