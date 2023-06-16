use crate::domain::table::*;
crud!(SysNotice {});


impl_select_page!(SysNotice{select_page(dto: &crate::domain::dto::NoticePageDTO) =>
    "`where 1=1`
    if dto.noticeTitle != '':
      ` and notice_title like #{'%'+dto.noticeTitle+'%'}`
    if dto.createBy != '':
      ` and create_by = #{dto.createBy}`
    if dto.noticeType != '':
      ` and notice_type = #{dto.noticeType}`
    if !sql.contains('count'):
     ` order by notice_title`"});

