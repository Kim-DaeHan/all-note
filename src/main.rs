use actix_cors::Cors;
use actix_rest_template::*;
use actix_web::{
    http::{self, StatusCode},
    middleware::{ErrorHandlers, Logger},
    web::{get, scope, Data},
    App, HttpResponse, HttpServer,
};
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

// actix-cors
// allow_any_origin():
// 모든 오리진(Origin)을 허용합니다. 크로스 도메인 요청을 보내는 모든 클라이언트를 허용하겠다는 의미입니다.
// 보안상 주의해야 하며, 프로덕션 환경에서는 필요한 경우에만 사용하는 것이 좋습니다.

// supports_credentials():
// 자격 증명(credential)을 허용합니다. 즉, 클라이언트가 인증 정보(쿠키, HTTP 인증 등)를 서버에 제공할 수 있도록 합니다.
// 이 옵션을 사용하려면 클라이언트와 서버 모두에서 자격 증명이 활성화되어 있어야 합니다.

// allowed_methods():
// 허용되는 HTTP 메서드를 지정합니다. 여기서는 "GET", "POST", "PUT", "DELETE"를 허용하도록 설정되어 있습니다.

// allowed_headers():
// 허용되는 HTTP 헤더를 지정합니다. 주로 클라이언트가 특정 헤더를 사용하여 서버에 추가 정보를 전달할 때 사용됩니다.
// 예제에서는 "Authorization"과 "Accept" 헤더를 허용하도록 설정되어 있습니다.

// max_age():
// 사전 검사 요청의 결과를 캐시할 시간을 지정합니다. 이는 브라우저에서 사전 검사 요청의 결과를 캐시하여 동일한 요청에 대해 추가적인 사전 검사를 수행하지 않도록 합니다.
// 예제에서는 3600초(1시간) 동안 캐시하도록 설정되어 있습니다.

// logging

// actix_web::middleware::logger와 env_logger는 다른 라이브러리지만 actix의 로깅시스템과 통합되어있다.
// 그래서 env_logger로 초기화를 해줘서 환경 변수와 로깅 레벨, 형식을 설정하면 actix_web::middleware::logger에도 영향을 미치게되면서 Logger::default()가 올바르게 작동을 한다.

// env_logger는 로깅 설정을 제공하고, 설정된 로깅을 실제로 출력하는 함수를 직접 제공하지 않습니다. 실제로 로그를 출력하는 부분은 log 라이브러리를 사용한 다른 로깅 백엔드에서 담당하게 됩니다. env_logger는 로깅 레벨, 로그 형식 등을 설정하는데 중점을 두며, 설정된 내용은 log 라이브러리의 로그 매크로를 통해 실제 출력되게 됩니다.

// Actix 웹에서는 Logger::default()를 사용하여 내장 로깅 미들웨어를 추가하면, 이 미들웨어가 env_logger를 통해 설정된 로깅 시스템과 연동되어 HTTP 요청 및 응답에 대한 로깅을 처리합니다. 여기서는 Actix의 내장 로깅 미들웨어를 사용하므로, 별도의 로깅 함수를 직접 호출하는 것이 아니라 log 라이브러리의 로그 매크로를 사용하여 로그를 출력합니다.

// 기본적으로는 다음과 같은 로그 매크로가 사용됩니다:

// trace!: 추적 수준의 로그
// debug!: 디버그 수준의 로그
// info!: 정보 수준의 로그
// warn!: 경고 수준의 로그
// error!: 에러 수준의 로그
// 이러한 매크로들은 log 라이브러리를 통해 제공되며, 사용 중인 로깅 백엔드에 따라 실제로 출력되거나 저장됩니다. Actix 웹에서는 Logger::default()와 함께 이러한 로그 매크로를 사용하여 HTTP 요청 및 응답에 대한 로깅을 수행합니다.

// 로깅 라이브러리

// env_logger: 환경 변수를 기반으로 로깅 설정을 제공하는 라이브러리. 로깅 레벨 설정 등을 통해 애플리케이션 전반의 로깅 환경을 조절하는 데 사용됩니다. 직접적으로 로그를 출력하는 함수는 제공하지 않습니다.

// middleware::Logger: Actix 웹에서 제공하는 내장 로깅 미들웨어. HTTP 요청과 응답에 대한 로깅을 담당하며, env_logger와 같은 설정을 통해 로깅 환경을 구성합니다. 사용자가 직접 로그를 출력하는 함수는 제공하지 않고, 자동으로 HTTP 요청이 처리될 때 로그를 출력합니다.

// log: Rust에서 사용되는 로깅 라이브러리. 여러 로깅 백엔드와 통합할 수 있도록 설계되어 있습니다. 로그 매크로(info!, warn! 등)를 통해 사용자가 직접 로그를 출력할 때 사용됩니다. Actix 웹에서는 내부적으로 log 라이브러리를 사용하여 로깅을 수행합니다.
