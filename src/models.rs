use serde::{Deserialize, Serialize};

// ---------------------------------------------------------
// Veri Yapıları (Data Structures)
// ---------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // İstenen dosya yolu (Requested file path)
    pub exp: usize,  // Son kullanma tarihi (Expiration timestamp)
}

pub struct AppState {
    pub jwt_secret: String,
    pub base_storage_path: String,
}
