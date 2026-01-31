use actix_web::{web, App, HttpServer};

mod models;
mod config;
mod handlers;

use config::load_config;
use handlers::download_file;

// ---------------------------------------------------------
// Main (Giriş / Entry)
// ---------------------------------------------------------

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(load_config());

    println!("🚀 CDN Servisi Başlatıldı (Port: 8080)");
    println!("📂 Depolama Yolu: {}", state.base_storage_path);

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/download", web::get().to(download_file))
    })
    .bind(("0.0.0.0", 8080))? // 0.0.0.0 Docker için gereklidir (Required for Docker)
    .run()
    .await
}
