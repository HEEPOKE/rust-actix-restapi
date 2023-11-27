use crate::models::user::{NewUser, UpdatedUser, User};
use crate::schema::schema::users;
use crate::utils::response::CustomError;
use diesel::dsl::update;
use diesel::prelude::*;
use std::sync::{Arc, RwLock};

pub struct UserService {
    pub conn: Arc<RwLock<PgConnection>>,
}

impl UserService {
    pub fn get_all_users(&mut self) -> Result<Vec<User>, CustomError> {
        let conn = &mut *self.conn.write().map_err(|_| CustomError::InternalError)?;
        users::table
            .load::<User>(conn)
            .map_err(|e| CustomError::DieselError(e))
    }

    pub fn get_user_by_id(&mut self, user_id: i32) -> Result<Option<User>, diesel::result::Error> {
        users::table
            .find(user_id)
            .first::<User>(&mut *self.conn.write().unwrap())
            .optional()
    }

    pub fn create_user(&self, new_user: NewUser) -> Result<User, CustomError> {
        diesel::insert_into(users::table)
            .values(&new_user)
            .get_result::<User>(&mut *self.conn.write().unwrap())
            .map_err(|e| CustomError::DieselError(e))
    }

    pub fn update_user(
        &self,
        user_id: i32,
        updated_user: &NewUser<'_>,
    ) -> Result<User, CustomError> {
        let updated_user_data = UpdatedUser {
            username: updated_user.username,
            email: updated_user.email,
            password: updated_user.password,
            tel: updated_user.tel,
        };

        let updated_rows = match update(users::table.find(user_id))
            .set(&updated_user_data)
            .execute(&mut *self.conn.write().unwrap())
        {
            Ok(rows) => rows,
            Err(_) => return Err(CustomError::NotFound),
        };

        if updated_rows > 0 {
            let user = users::table
                .find(user_id)
                .first::<User>(&mut *self.conn.write().unwrap());
            match user {
                Ok(user) => Ok(user),
                Err(err) => Err(CustomError::DieselError(err)),
            }
        } else {
            Err(CustomError::NotFound)
        }
    }

    pub fn delete_user(&self, user_id: i32) -> Result<(), CustomError> {
        let target_user = users::table.find(user_id);
        let _ = diesel::delete(target_user).execute(&mut *self.conn.write().unwrap());
        Ok(())
    }
}
