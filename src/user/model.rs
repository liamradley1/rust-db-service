use crate::api_error::ApiError;
use crate::db;
use crate::schema::user_table;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use core::option::Option;

#[derive(Serialize, Deserialize, AsChangeset)]
#[table_name = "user_table"]
pub struct UserMessage {
    pub email: String,
    pub password: String,
}

pub fn update(user_message: UserMessage, mut user : User) -> User {
    let user_result = User::from(user_message);
    user.email = user_result.email;
    user.password = user_result.password;
    user.updated_at = Option::from(Utc::now().naive_utc());
    user
}

#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[table_name = "user_table"]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

impl User {
    pub fn find_all() -> Result<Vec<Self>, ApiError> {
        let conn = db::connection()?;

        let users = user_table::table
            .load::<User>(&conn)?;

        Ok(users)
    }

    pub fn find(id: Uuid) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let user = user_table::table
            .filter(user_table::id.eq(id))
            .first(&conn)?;

        Ok(user)
    }

    pub fn create(user: UserMessage) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let user = User::from(user);
        let user = diesel::insert_into(user_table::table)
            .values(user)
            .get_result(&conn)?;

        Ok(user)
    }

    pub fn update(id: Uuid, user: UserMessage) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let mut extracted_result = User::find(id).unwrap();

        extracted_result = update(user, extracted_result);

        let user = diesel::update(user_table::table)
            .filter(user_table::id.eq(id))
            .set(extracted_result)
            .get_result(&conn)?;

        Ok(user)
    }

    pub fn delete(id: Uuid) -> Result<usize, ApiError> {
        let conn = db::connection()?;

        let res = diesel::delete(
                user_table::table
                    .filter(user_table::id.eq(id))
            )
            .execute(&conn)?;

        Ok(res)
    }

}

impl From<UserMessage> for User {
    fn from(user: UserMessage) -> Self {
        User {
            id: Uuid::new_v4(),
            email: user.email,
            password: user.password,
            created_at: Utc::now().naive_utc(),
            updated_at: None,
        }
    }
}
