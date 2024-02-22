use crate::database::PgPool;
use crate::schema::users::{self};
use actix_web::web::Data;
use chrono::Utc;
use diesel::prelude::*;
use diesel::{result::Error, AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, Queryable)]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    pub id: String,
    pub google_id: String,
    pub email: String,
    pub user_name: String,
    pub verified: Option<bool>,
    pub provider: String,
    pub photo: String,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct UserData {
    pub id: Option<String>,
    pub google_id: String,
    pub email: String,
    pub user_name: String,
    pub verified: bool,
    pub provider: String,
    pub photo: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, AsChangeset)]
#[diesel(table_name = crate::schema::users)]
pub struct UpdateUserData {
    pub id: String,
    pub google_id: String,
    pub email: String,
    pub user_name: String,
    pub verified: bool,
    pub provider: String,
    pub photo: String,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Deserialize, Debug)]
pub struct EmailQueryParam {
    pub email: String,
}

impl User {
    pub async fn get_users(pool: &Data<PgPool>) -> Result<Vec<(String, String)>, Error> {
        let conn = &mut pool.get().expect("Couldn't get DB connection from pool");

        users::table
            .select((users::id, users::email))
            .load::<(String, String)>(conn)
    }

    pub async fn get_users_load(pool: &Data<PgPool>) -> Result<Vec<User>, Error> {
        let conn = &mut pool.get().expect("Couldn't get DB connection from pool");

        users::table.load::<User>(conn)
    }

    pub async fn get_users_by_email(email: &str, pool: &Data<PgPool>) -> Result<User, Error> {
        let conn = &mut pool.get().expect("Couldn't get DB connection from pool");
        users::table
            .filter(users::email.eq(email))
            .get_result::<User>(conn)
    }

    pub async fn get_users_by_id(user_id: &str, pool: &Data<PgPool>) -> Result<User, Error> {
        let conn = &mut pool.get().expect("Couldn't get DB connection from pool");
        users::table.find(user_id).get_result::<User>(conn)
    }

    pub fn get_users_auth(user_id: &str, pool: &Data<PgPool>) -> Result<User, Error> {
        let conn = &mut pool.get().expect("Couldn't get DB connection from pool");
        users::table.find(user_id).get_result::<User>(conn)
    }

    pub async fn delete_users_by_id(user_id: &str, pool: &Data<PgPool>) -> Result<usize, Error> {
        let conn = &mut pool.get().expect("Couldn't get DB connection from pool");
        diesel::delete(users::table.find(user_id)).execute(conn)
    }
}

impl UserData {
    pub async fn create_users(user_data: UserData, pool: &Data<PgPool>) -> Result<String, Error> {
        let user = UserData {
            id: Some(Uuid::new_v4().to_string()),
            ..user_data
        };

        let conn = &mut pool.get().expect("Couldn't get DB connection from pool");

        diesel::insert_into(users::table)
            .values(user)
            .returning(users::id)
            .get_result::<String>(conn)
    }
}

impl UpdateUserData {
    pub async fn update_users(
        user_data: UpdateUserData,
        pool: &Data<PgPool>,
    ) -> Result<usize, Error> {
        let conn = &mut pool.get().expect("Couldn't get DB connection from pool");
        let updated_date = Some(Utc::now().naive_utc());

        let user_id = user_data.id.clone();

        let user = UpdateUserData {
            updated_at: updated_date,
            ..user_data
        };

        diesel::update(users::table.find(user_id))
            .set(user)
            .execute(conn)
    }
}
