mod db;
use actix_web::middleware::{Compress, Logger};
use actix_web::{
    App,          // 应用构建器
    HttpResponse, // 响应类型
    HttpServer,   // HTTP 服务器
    Responder,    // 响应 trait
    get,          // 路由宏：GET 请求
    web,          // 工具模块（数据提取、JSON 等）
};
use chrono::Utc; // 时间处理（需添加 chrono 依赖）
use dotenv::dotenv;
use log::info;
use serde::Serialize;
use std::env;

// 1. 健康检查端点（GET /health）
#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("OK") // 返回 200 OK + 文本 "OK"
}

// 2. 首页端点（GET /）
#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to Actix-web! 🚀")
}

// 3. JSON API 端点（GET /api/time）
#[derive(Serialize)] // 自动实现 JSON 序列化
struct TimeResponse {
    timestamp: i64,   // 时间戳（秒）
    datetime: String, // 格式化时间
}
#[get("/api/time")]
async fn get_current_time() -> impl Responder {
    let now = Utc::now();
    HttpResponse::Ok().json(TimeResponse {
        timestamp: now.timestamp(),
        datetime: now.format("%Y-%m-%d %H:%M:%S").to_string(),
    })
}

#[actix_web::main] // Actix 异步运行时入口（等价于 #[tokio::main]）
async fn main() -> std::io::Result<()> {
    // 加载 .env 文件（若存在）
    dotenv().ok();
    // 初始化日志（从环境变量 RUST_LOG 读取级别，默认 info）
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // 从环境变量读取端口（默认 8080）
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_addr = format!("0.0.0.0:{}", port); // 绑定所有网卡

    info!("Starting Actix-web server at http://{}", bind_addr);

    // 从环境变量读取数据库 URL（示例：postgres://user:pass@localhost/dbname）
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set");
    let pool = db::create_pool(&database_url)
        .await
        .expect("Failed to create DB pool");

    // 启动 HTTP 服务器
    HttpServer::new(move || {
        App::new() // 创建应用实例
            .app_data(web::Data::new(pool.clone()))
            .wrap(Logger::default()) // 日志中间件（记录请求方法、路径、状态码）
            .wrap(Compress::default())
            .service(health_check) // 注册健康检查路由
            .service(index) // 注册首页路由
            .service(get_current_time) // 注册 JSON API 路由
    })
    .bind(bind_addr)? // 绑定地址（失败返回错误）
    .run() // 运行服务器
    .await // 等待异步完成
}
