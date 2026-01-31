# 🚀 Secure CDN Service (JWT-Protected File Server)

> 📱 **Not:** Bu README dosyası GitHub mobil uygulaması üzerinde çalışan bir yapay zeka ajanı tarafından oluşturulmuştur.
> 
> 📱 **Note:** This README was created by an AI agent running on GitHub mobile application.

---

## 🇹🇷 Türkçe

### 📝 Proje Açıklaması

Bu proje, **JWT (JSON Web Token)** ile korunan güvenli bir CDN (Content Delivery Network) servisidir. Rust programlama dili ve Actix-web framework'ü kullanılarak geliştirilmiştir. Dosyaların güvenli bir şekilde saklanması ve yetkilendirilmiş kullanıcılara sunulması için tasarlanmıştır.

### ✨ Özellikler

- 🔐 **JWT Tabanlı Kimlik Doğrulama**: Her dosya indirme isteği JWT token ile korunur
- 🛡️ **Yol Güvenliği**: Path traversal saldırılarına karşı koruma (.. saldırılarını engeller)
- ⚡ **Zero-Copy File Serving**: Yüksek performanslı dosya aktarımı
- 📦 **Range Request Desteği**: Büyük dosyalar için kısmi içerik indirme
- 🧠 **Akıllı Caching**: 
  - Medya dosyaları (jpg, png, mp4, vb.) → 1 yıl cache
  - Dokümanlar → Cache yok
- 🐳 **Docker Desteği**: Kolay dağıtım için Docker ve Docker Compose yapılandırması
- ⚙️ **Asenkron İşleme**: Tokio ile yüksek performanslı async/await desteği

### 🔧 Teknolojiler

- **Rust** - Sistem programlama dili
- **Actix-web** - Yüksek performanslı web framework
- **JWT** - Token tabanlı kimlik doğrulama
- **Docker** - Konteynerizasyon
- **Tokio** - Asenkron runtime

### 📋 Gereksinimler

- Rust 1.84 veya üzeri
- Docker & Docker Compose (opsiyonel)
- `.env` dosyasında `JWT_SECRET` değişkeni

### 🚀 Kurulum ve Çalıştırma

#### Docker ile (Önerilen)

```bash
# Projeyi klonlayın
git clone https://github.com/asimcanuz/Test.git
cd Test

# Uploads klasörünü oluşturun
mkdir -p uploads

# Docker Compose ile başlatın
docker-compose up -d
```

#### Manuel Kurulum

```bash
# Bağımlılıkları yükleyin
cargo build --release

# .env dosyası oluşturun
echo "JWT_SECRET=gizli_anahtar_buraya" > .env
echo "STORAGE_PATH=./uploads" >> .env

# Uploads klasörünü oluşturun
mkdir -p uploads

# Servisi başlatın
cargo run --release
```

### 📡 API Kullanımı

#### Dosya İndirme Endpoint'i

```
GET /download
Authorization: Bearer <JWT_TOKEN>
```

**JWT Token Formatı:**
```json
{
  "sub": "dosya/yolu.jpg",
  "exp": 1735689600
}
```

**Örnek İstek:**
```bash
curl -H "Authorization: Bearer eyJhbGc..." \
     http://localhost:8080/download
```

### 🔒 Güvenlik Özellikleri

1. **Token Doğrulama**: Her istekte JWT token kontrolü
2. **Süre Kontrolü**: Token'ın geçerlilik süresi kontrol edilir
3. **Yol Sanitizasyonu**: Path traversal saldırılarına karşı koruma
4. **Klasör Kontrolü**: Dosyaların sadece belirlenen klasörden sunulması
5. **Algorithm Restriction**: Sadece HS256 algoritması kabul edilir

### 📂 Proje Yapısı

```
.
├── main.rs              # Ana uygulama kodu
├── cargo.toml           # Rust bağımlılıkları
├── Dockerfile           # Docker yapılandırması
├── docker-compose.yml   # Docker Compose ayarları
├── setup.sh             # Kurulum script'i
└── uploads/             # Dosya depolama klasörü
```

### 🌟 Kullanım Senaryoları

- Özel medya dosyalarının güvenli dağıtımı
- Yetkilendirilmiş kullanıcılara dosya sunumu
- API'ler için güvenli CDN altyapısı
- Mikroservis mimarisinde medya servisi

---

## 🇬🇧 English

### 📝 Project Description

This project is a **JWT (JSON Web Token)** protected secure CDN (Content Delivery Network) service. It is developed using the Rust programming language and the Actix-web framework. It is designed for secure file storage and serving files to authorized users.

### ✨ Features

- 🔐 **JWT-Based Authentication**: Every file download request is protected with JWT tokens
- 🛡️ **Path Security**: Protection against path traversal attacks (blocks .. attacks)
- ⚡ **Zero-Copy File Serving**: High-performance file transfer
- 📦 **Range Request Support**: Partial content download for large files
- 🧠 **Smart Caching**: 
  - Media files (jpg, png, mp4, etc.) → 1 year cache
  - Documents → No cache
- 🐳 **Docker Support**: Docker and Docker Compose configuration for easy deployment
- ⚙️ **Asynchronous Processing**: High-performance async/await support with Tokio

### 🔧 Technologies

- **Rust** - Systems programming language
- **Actix-web** - High-performance web framework
- **JWT** - Token-based authentication
- **Docker** - Containerization
- **Tokio** - Asynchronous runtime

### 📋 Requirements

- Rust 1.84 or higher
- Docker & Docker Compose (optional)
- `JWT_SECRET` variable in `.env` file

### 🚀 Installation and Running

#### With Docker (Recommended)

```bash
# Clone the project
git clone https://github.com/asimcanuz/Test.git
cd Test

# Create uploads folder
mkdir -p uploads

# Start with Docker Compose
docker-compose up -d
```

#### Manual Installation

```bash
# Install dependencies
cargo build --release

# Create .env file
echo "JWT_SECRET=your_secret_key_here" > .env
echo "STORAGE_PATH=./uploads" >> .env

# Create uploads folder
mkdir -p uploads

# Start the service
cargo run --release
```

### 📡 API Usage

#### File Download Endpoint

```
GET /download
Authorization: Bearer <JWT_TOKEN>
```

**JWT Token Format:**
```json
{
  "sub": "file/path.jpg",
  "exp": 1735689600
}
```

**Example Request:**
```bash
curl -H "Authorization: Bearer eyJhbGc..." \
     http://localhost:8080/download
```

### 🔒 Security Features

1. **Token Validation**: JWT token check on every request
2. **Expiration Check**: Token validity period is verified
3. **Path Sanitization**: Protection against path traversal attacks
4. **Folder Verification**: Files are only served from the designated folder
5. **Algorithm Restriction**: Only HS256 algorithm is accepted

### 📂 Project Structure

```
.
├── main.rs              # Main application code
├── cargo.toml           # Rust dependencies
├── Dockerfile           # Docker configuration
├── docker-compose.yml   # Docker Compose settings
├── setup.sh             # Setup script
└── uploads/             # File storage folder
```

### 🌟 Use Cases

- Secure distribution of private media files
- File serving to authorized users
- Secure CDN infrastructure for APIs
- Media service in microservice architecture

---

## 👨‍💻 Geliştirici / Developer

Bu proje GitHub mobil uygulamasında AI agent desteği ile geliştirilmiştir.

This project was developed with AI agent support on the GitHub mobile application.

## 📄 License

MIT License

---

**⚠️ Önemli Güvenlik Notu / Important Security Note:**
Üretim ortamında `JWT_SECRET` değerini mutlaka güçlü ve rastgele bir değer olarak ayarlayın!

In production environment, make sure to set `JWT_SECRET` to a strong and random value!