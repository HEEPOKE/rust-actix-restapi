use crate::schema::users;
use crate::models::{User, NewUser};
use diesel::prelude::*;
use diesel::result::Error;

pub struct UserService<'a> {
    pub conn: &'a PgConnection,
}

impl<'a> UserService<'a> {
    pub fn new(conn: &'a PgConnection) -> Self {
        UserService { conn }
    }

    pub fn get_all_users(&self) -> Result<Vec<User>, Error> {
        users::table.load::<User>(self.conn)
    }
    
    pub fn create_user(&self, new_user: NewUser) -> Result<User, Error> {
        diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(self.conn)
    }

    pub fn get_user_by_id(&self, user_id: i32) -> Result<Option<User>, Error> {
        users::table.find(user_id).first::<User>(self.conn).optional()
    }

    pub fn update_user(&self, user_id: i32, updated_user: NewUser) -> Result<User, Error> {
        let target_user = users::table.find(user_id);
        diesel::update(target_user)
            .set(&updated_user)
            .get_result(self.conn)
    }

    pub fn delete_user(&self, user_id: i32) -> Result<(), Error> {
        let target_user = users::table.find(user_id);
        diesel::delete(target_user).execute(self.conn)?;
        Ok(())
    }
}
