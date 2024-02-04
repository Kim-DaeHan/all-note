use crate::database::PgPool;
use crate::schema::posts::{self, dsl::*};
use actix_web::web::Data;
use chrono::Utc;
use diesel::prelude::*;
use diesel::{result::Error, AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, Queryable)]
#[diesel(table_name = crate::schema::posts)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

// <'a> 은 라이프타임 매개변수를 나타냄(a라는 라이프타임이 있다)
#[derive(Serialize, Deserialize, Debug, Clone, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::posts)]
pub struct PostData {
    pub id: Option<String>,
    pub title: String,
    pub body: String,
    pub published: Option<bool>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}
// Query (조회)할 때 (Queryable):

// 조회된 데이터는 데이터베이스에서 읽어오는 것이기 때문에 소유권을 가질 필요가 있습니다. 그래서 String과 같이 소유권을 가지는 타입을 사용합니다.
// String은 동적으로 크기가 조절되는 문자열을 나타내며, 이는 데이터베이스에서 읽어온 문자열의 크기가 불확실할 수 있기 때문에 적합합니다.

// Insert (삽입)할 때 (Insertable):

// 데이터를 데이터베이스에 넣을 때는 소유권을 넘기는 것이 아니라 참조만 넘기는 것이 효율적일 수 있습니다. 특히 문자열을 데이터베이스에 넣을 때 크기를 미리 알 수 없는 경우가 많기 때문에 동적인 문자열을 소유하기보다는 참조를 사용하는 것이 좋습니다.
// 따라서 &'a str과 같이 라이프타임이 있는 참조를 사용하여 문자열을 참조하고, 데이터베이스에는 참조만 전달합니다.

impl Post {
    pub async fn get_posts(
        pool: &Data<PgPool>,
    ) -> Result<Vec<(String, String, String, bool)>, Error> {
        let conn = &mut pool.get().expect("Couldn't get DB connection from pool");

        posts
            .select((body, title, posts::id, published))
            .load::<(String, String, String, bool)>(conn)
    }

    pub async fn get_posts_load(pool: &Data<PgPool>) -> Result<Vec<Post>, Error> {
        let conn = &mut pool.get().expect("Couldn't get DB connection from pool");
        // use crate::schema::posts::{dsl::*}로 인해서 posts::table을 posts로 사용가능
        posts.load::<Post>(conn)
    }

    pub async fn get_posts_by_id(
        post_id: &str,
        pool: &Data<PgPool>,
    ) -> Result<(String, String, String, bool), Error> {
        let conn = &mut pool.get().expect("Couldn't get DB connection from pool");
        posts
            .find(post_id)
            // .filter(posts::id.eq(post_id))
            .select((body, title, posts::id, published))
            // get_result: 주어진 조건에 해당하는 하나의 결과를 반환, 결과가 여러 개거나 없으면 에러(정확히 하나의 결과가 예상되는 상황)
            .get_result::<(String, String, String, bool)>(conn)
        // first: 조건에 해당하는 모든 결과 중 첫 번째 결과 반환
        // .first::<(String, String, String, bool)>(conn)
        // load: 여러 레코드를 로드하고 벡터로 반환, 결과를 단일 값이 아닌 여러 레코드로 받아오려 할 때 사용
        // .load::<(String, String, String, bool)>(conn)
    }

    pub async fn delete_posts_by_id(post_id: &str, pool: &Data<PgPool>) -> Result<usize, Error> {
        let conn = &mut pool.get().expect("Couldn't get DB connection from pool");
        diesel::delete(posts.find(post_id)).execute(conn)
    }
}

impl PostData {
    pub async fn create_posts(post_data: PostData, pool: &Data<PgPool>) -> Result<(), Error> {
        let post = PostData {
            id: Some(Uuid::new_v4().to_string()),
            ..post_data
        };

        let conn = &mut pool.get().expect("Couldn't get DB connection from pool");

        diesel::insert_into(posts).values(post).execute(conn)?;
        Ok(())
    }

    pub async fn update_posts(post_data: PostData, pool: &Data<PgPool>) -> Result<usize, Error> {
        let conn = &mut pool.get().expect("Couldn't get DB connection from pool");
        let updated_date = Some(Utc::now().naive_utc());

        let post = PostData {
            id: None,
            updated_at: updated_date,
            ..post_data
        };

        diesel::update(posts.find(post_data.id.unwrap()))
            .set(post)
            // .get_result::<Post>(conn)
            .execute(conn)
    }
}
