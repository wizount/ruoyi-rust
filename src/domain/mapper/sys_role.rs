use crate::domain::dto::RolePageDTO;
use crate::domain::table::*;
crud!(SysRole {});
impl_select_page!(SysRole{select_page(dto:&RolePageDTO)=>
    "`where del_flag = '0'`
    if dto.roleName != '':
      ` and role_name like #{'%'+dto.roleName+'%'}`
    if dto.roleKey != '':
      ` and role_key like #{'%'+dto.roleKey+'%'}`
    if dto.status != '':
      ` and status = #{dto.status}`
    if !sql.contains('count'):
     ` order by role_sort`"});

