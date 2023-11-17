use crate::models::user::{NewUser, User, UpdatedUser};
use crate::schema::schema::users;
use diesel::prelude::*;
use diesel::dsl::update;

pub struct UserService<'a> {
    pub conn: &'a mut diesel::PgConnection,
}

impl<'a> UserService<'a> {
    pub fn new(conn: &'a mut diesel::PgConnection) -> Self {
        UserService { conn }
    }

   pub fn get_all_users(&mut self) -> Result<Vec<User>, diesel::result::Error> {
        users::table.load::<User>(self.conn)
    }

    pub fn get_user_by_id(&mut self, user_id: i32) -> Result<Option<User>, diesel::result::Error> {
        users::table
        .find(user_id)
        .first::<User>(self.conn)
        .optional()
    }

    pub fn create_user(&mut self, new_user: NewUser) -> Result<User, diesel::result::Error> {
        diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(self.conn)
    }

    pub fn update_user(
        &mut self,
        user_id: i32,
        updated_user: &NewUser<'_>,
    ) -> Result<User, diesel::result::Error> {
        let updated_user_data = UpdatedUser {
            username: updated_user.username,
            email: updated_user.email,
            password: updated_user.password,
            tel: updated_user.tel,
        };
    
        let updated_rows = update(users::table.find(user_id))
            .set(&updated_user_data)
            .execute(self.conn)?;
    
        if updated_rows > 0 {
            let user = users::table.find(user_id).first(self.conn)?;
            Ok(user)
        } else {
            Err(diesel::result::Error::NotFound)
        }
    }

    pub fn delete_user(&self, user_id: i32) -> Result<(), diesel::result::Error> {
        let target_user = users::table.find(user_id);
        diesel::delete(target_user).execute(self.conn)?;
        Ok(())
    }
}
