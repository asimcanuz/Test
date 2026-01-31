use actix_web::{web, App, HttpServer, HttpResponse, Responder, HttpRequest};
use actix_files::NamedFile; // Zero-copy & Range Requests için (For Zero-copy & Range Requests)
use actix_web::http::header::CACHE_CONTROL; // Header modülleri
use serde::{Deserialize, Serialize};
use std::env;
use std::path::Path;
use dotenvy::dotenv;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};
use vaultrs::kv2;

// ---------------------------------------------------------
// 1. Veri Yapıları (Data Structures)
// ---------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String, // İstenen dosya yolu (Requested file path)
    exp: usize,  // Son kullanma tarihi (Expiration timestamp)
}

#[derive(Debug, Serialize, Deserialize)]
struct VaultSecrets {
    jwt_secret: String,
}

struct AppState {
    jwt_secret: String,
    base_storage_path: String,
}

// ---------------------------------------------------------
// 2. Vault İşlemleri (Vault Operations)
// ---------------------------------------------------------

/// Vault'tan secret'ları çeker (Fetch secrets from Vault)
async fn fetch_secrets_from_vault() -> Result<String, Box<dyn std::error::Error>> {
    let vault_addr = env::var("VAULT_ADDR")
        .unwrap_or_else(|_| "http://localhost:8200".to_string());
    let vault_token = env::var("VAULT_TOKEN")
        .expect("VAULT_TOKEN ortam değişkeni gerekli! (VAULT_TOKEN env var required!)");

    println!("🔗 Vault'a bağlanılıyor: {}", vault_addr);

    // Vault istemcisi oluştur (Create Vault client)
    let client = VaultClient::new(
        VaultClientSettingsBuilder::default()
            .address(&vault_addr)
            .token(&vault_token)
            .build()?
    )?;

    println!("📥 Vault'tan secret'lar alınıyor...");

    // KV v2 engine'den secret'ı oku (Read secret from KV v2 engine)
    let secrets: VaultSecrets = kv2::read(&client, "secret", "cdn-service").await?;

    println!("✅ Secret'lar başarıyla alındı!");

    Ok(secrets.jwt_secret)
}

// ---------------------------------------------------------
// 3. Handler (İstek Karşılayıcı / Request Handler)
// ---------------------------------------------------------

                async fn download_file(req: HttpRequest, data: web::Data<AppState>) -> impl Responder {
                    // A. Token Kontrolü (Token Check) 🛡️
                        let auth_header = match req.headers().get("Authorization") {
                                Some(h) => h.to_str().unwrap_or(""),
                                        None => return HttpResponse::Unauthorized().body("Token yok! (No Token!)"),
                                            };

                                                if !auth_header.starts_with("Bearer ") {
                                                        return HttpResponse::Unauthorized().body("Gecersiz Token formati! (Invalid format!)");
                                                            }

                                                                let token_string = &auth_header[7..];
                                                                    let key = DecodingKey::from_secret(data.jwt_secret.as_bytes());
                                                                        // Sadece HS256 algoritmasını kabul et (Only accept HS256)
                                                                            let validation = Validation::new(Algorithm::HS256);

                                                                                // Token'ı çöz ve süresini kontrol et (Decode token and check expiration)
                                                                                    let file_sub_path = match decode::<Claims>(token_string, &key, &validation) {
                                                                                            Ok(token_data) => token_data.claims.sub,
                                                                                                    Err(_) => return HttpResponse::Unauthorized().body("Token gecersiz veya süresi dolmuş! (Invalid or expired token!)"),
                                                                                                        };

                                                                                                            // B. Yol Güvenliği (Path Sanitization) 🧹
                                                                                                                // ".." saldırılarını engelle (Prevent ".." attacks)
                                                                                                                    let base_path = Path::new(&data.base_storage_path);
                                                                                                                        let requested_path = Path::new(&file_sub_path);

                                                                                                                            // Yolu birleştir ve temizle (Join and canonicalize path)
                                                                                                                                let full_path = match base_path.join(requested_path).canonicalize() {
                                                                                                                                        Ok(path) => path,
                                                                                                                                                Err(_) => return HttpResponse::NotFound().body("Dosya bulunamadı! (File not found!)"),
                                                                                                                                                    };

                                                                                                                                                        // Dosya gerçekten bizim klasörde mi? (Is file really in our folder?)
                                                                                                                                                            if !full_path.starts_with(base_path) {
                                                                                                                                                                    return HttpResponse::Forbidden().body("Yasaklı Bölge! (Access Denied!)");
                                                                                                                                                                        }

                                                                                                                                                                            // C. Dosya Gönderimi (File Serving) 🚀
                                                                                                                                                                                // 'open_async' thread'i bloklamaz (Does not block thread)
                                                                                                                                                                                    let named_file = match NamedFile::open_async(&full_path).await {
                                                                                                                                                                                            Ok(file) => file,
                                                                                                                                                                                                    Err(_) => return HttpResponse::NotFound().body("Dosya diskte yok! (File missing on disk!)"),
                                                                                                                                                                                                        };

                                                                                                                                                                                                            // D. Akıllı Caching (Smart Caching) 🧠
                                                                                                                                                                                                                let extension = full_path.extension().and_then(|e| e.to_str()).unwrap_or("");
                                                                                                                                                                                                                    let mut response = named_file.into_response(&req);

                                                                                                                                                                                                                        if ["jpg", "jpeg", "png", "webp", "gif", "mp4"].contains(&extension) {
                                                                                                                                                                                                                                // Medya dosyaları: 1 Yıl Cache (Media files: 1 Year Cache)
                                                                                                                                                                                                                                        response.headers_mut().insert(
                                                                                                                                                                                                                                                    CACHE_CONTROL, 
                                                                                                                                                                                                                                                                "public, max-age=31536000, immutable".parse().unwrap()
                                                                                                                                                                                                                                                                        );
                                                                                                                                                                                                                                                                            } else {
                                                                                                                                                                                                                                                                                    // Dokümanlar: Cache yok (Docs: No Cache)
                                                                                                                                                                                                                                                                                            response.headers_mut().insert(
                                                                                                                                                                                                                                                                                                        CACHE_CONTROL, 
                                                                                                                                                                                                                                                                                                                    "private, no-cache".parse().unwrap()
                                                                                                                                                                                                                                                                                                                            );
                                                                                                                                                                                                                                                                                                                                }

                                                                                                                                                                                                                                                                                                                                    response
                                                                                                                                                                                                                                                                                                                                    }

                                                                                                                                                                                                                                                                                                                                    // ---------------------------------------------------------
                                                                                                                                                                                                                                                                                                                                    // 4. Main (Giriş / Entry)
                                                                                                                                                                                                                                                                                                                                    // ---------------------------------------------------------

                                                                                                                                                                                                                                                                                                                                    #[actix_web::main]
                                                                                                                                                                                                                                                                                                                                    async fn main() -> std::io::Result<()> {
                                                                                                                                                                                                                                                                                                                                        dotenv().ok();
                                                                                                                                                                                                                                                                                                                                            
                                                                                                                                                                                                                                                                                                                                                // Vault'tan secret'ları getir (Fetch secrets from Vault)
                                                                                                                                                                                                                                                                                                                                                    let secret = match fetch_secrets_from_vault().await {
                                                                                                                                                                                                                                                                                                                                                            Ok(s) => s,
                                                                                                                                                                                                                                                                                                                                                                    Err(e) => {
                                                                                                                                                                                                                                                                                                                                                                                eprintln!("❌ Vault'tan secret alınamadı: {}", e);
                                                                                                                                                                                                                                                                                                                                                                                        eprintln!("💡 Fallback: .env dosyasından JWT_SECRET okunacak");
                                                                                                                                                                                                                                                                                                                                                                                                env::var("JWT_SECRET")
                                                                                                                                                                                                                                                                                                                                                                                                            .expect("JWT_SECRET ne Vault'ta ne de .env dosyasında bulunamadı!")
                                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                                                    };
                                                                                                                                                                                                                                                                                                                                                                                                                        let storage_path = env::var("STORAGE_PATH").unwrap_or_else(|_| "./uploads".to_string());

                                                                                                                                                                                                                                                                                                                                                            let state = web::Data::new(AppState {
                                                                                                                                                                                                                                                                                                                                                                    jwt_secret: secret,
                                                                                                                                                                                                                                                                                                                                                                            base_storage_path: storage_path,
                                                                                                                                                                                                                                                                                                                                                                                });

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