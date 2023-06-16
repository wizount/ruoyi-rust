use crate::domain::table::*;
crud!(SysDept {});


impl_select!(SysDept{select_all_query(dto: &crate::domain::dto::DeptQueryDTO) =>
  "`where parent_id IS NOT NULL`
    if dto.deptName != '':
      ` and dept_name like #{'%'+dto.deptName+'%'}`
    if dto.status != '':
      ` and status = #{dto.status}`
    if !sql.contains('count'):
     ` order by order_num`"});

