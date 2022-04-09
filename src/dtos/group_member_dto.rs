use crate::schema::group_members;

#[derive(Debug, Insertable)]
#[table_name = "group_members"]
pub struct GroupMemberDto {
    pub group_id: i32,
    pub user_id: i32,
    pub admin: bool,
}

impl GroupMemberDto {
    pub fn new(group_id: i32, user_id: i32, admin: bool) -> Self {
        GroupMemberDto { group_id, user_id, admin }
    }
}

