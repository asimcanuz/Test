use actix_web::{web, HttpResponse, Responder, HttpRequest};
use actix_files::NamedFile; // Zero-copy & Range Requests için (For Zero-copy & Range Requests)
use actix_web::http::header::CACHE_CONTROL;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use std::path::Path;
use crate::models::{AppState, Claims};

// ---------------------------------------------------------
// Handler (İstek Karşılayıcı / Request Handler)
// ---------------------------------------------------------

pub async fn download_file(req: HttpRequest, data: web::Data<AppState>) -> impl Responder {
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
