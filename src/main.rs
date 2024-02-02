use actix_cors::Cors;
use actix_web::{
    http::{self, StatusCode},
    middleware::{ErrorHandlers, Logger},
    web::{get, scope, Data},
    App, HttpResponse, HttpServer,
};
use all_note::*;
use diesel::RunQueryDsl;
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    // 로깅 초기화(로그 레벨 설정)
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    let pool = database::establish_connection();
    let mut connection = pool.get().expect("Failed to get connection from pool");

    match diesel::sql_query("SELECT 1").execute(&mut connection) {
        Ok(_) => println!("Database connection successful!"),
        Err(err) => eprintln!("Error connecting to the database: {:?}", err),
    }

    HttpServer::new(move || {
        App::new()
            // 에러 핸들러 미들웨어
            .wrap(ErrorHandlers::new().handler(
                StatusCode::INTERNAL_SERVER_ERROR,
                middleware::error_handler::error_handler,
            ))
            // 로깅 미들웨어
            .wrap(Logger::default())
            // Cors 미들웨어 추가
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .supports_credentials()
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .max_age(3600),
            )
            .wrap(middleware::req_res_middleware::SayHi)
            .app_data(Data::new(pool.clone()))
            .service(scope("/api").configure(routes::configure))
            .route(
                "/",
                get().to(|| async { HttpResponse::Ok().body("Hello, Actix!") }),
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
