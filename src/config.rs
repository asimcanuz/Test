use std::env;
use dotenvy::dotenv;
use crate::models::AppState;

// ---------------------------------------------------------
// Konfigürasyon (Configuration)
// ---------------------------------------------------------

pub fn load_config() -> AppState {
    dotenv().ok();
    
    // Fail-Fast: Ayarlar yoksa başlatma (Don't start if config missing)
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET .env dosyasında eksik!");
    let storage_path = env::var("STORAGE_PATH").unwrap_or_else(|_| "./uploads".to_string());
    
    AppState {
        jwt_secret: secret,
        base_storage_path: storage_path,
    }
}
