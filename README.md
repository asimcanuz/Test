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
- 🔒 **HashiCorp Vault Entegrasyonu**: Secret'lar güvenli bir şekilde Vault'ta saklanır

### 🔐 HashiCorp Vault Entegrasyonu

Bu proje, hassas bilgileri (JWT_SECRET gibi) güvenli bir şekilde saklamak için **HashiCorp Vault** ile entegre edilmiştir.

#### Vault Kurulumu

```bash
# Docker Compose ile Vault ve CDN servisini başlatın
docker-compose up -d

# Vault'u ilklendirin (secret'ları oluşturur)
./init-vault.sh
```

#### Vault'tan Secret Okuma

Uygulama başlatıldığında, JWT_SECRET otomatik olarak Vault'tan okunur:

```bash
export VAULT_ADDR=http://localhost:8200
export VAULT_TOKEN=myroot
cargo run --release
```

#### Vault'ta Secret'ları Görüntüleme

```bash
# Vault'a bağlanın
export VAULT_ADDR=http://localhost:8200
export VAULT_TOKEN=myroot

# Secret'ları görüntüleyin
docker exec -it vault_server vault kv get secret/cdn-service
```

#### Fallback Mekanizması

Vault'a bağlanılamadığında, uygulama `.env` dosyasındaki `JWT_SECRET` değerini kullanır:

```bash
echo "JWT_SECRET=gizli_anahtar_buraya" > .env
```

### 🔧 Teknolojiler

- **Rust** - Sistem programlama dili
- **Actix-web** - Yüksek performanslı web framework
- **JWT** - Token tabanlı kimlik doğrulama
- **HashiCorp Vault** - Secret yönetimi ve güvenli depolama
- **Docker** - Konteynerizasyon
- **Tokio** - Asenkron runtime

### 📋 Gereksinimler

- Rust 1.84 veya üzeri
- Docker & Docker Compose (opsiyonel)
- HashiCorp Vault (Docker Compose ile otomatik kurulur)
- `.env` dosyasında `JWT_SECRET` değişkeni (Vault kullanılmıyorsa)

### 🚀 Kurulum ve Çalıştırma

#### Docker ile (Önerilen)

```bash
# Projeyi klonlayın
git clone https://github.com/asimcanuz/Test.git
cd Test

# Uploads klasörünü oluşturun
mkdir -p uploads

# Docker Compose ile başlatın (Vault ve CDN servisi)
docker-compose up -d

# Vault'u ilklendirin ve secret'ları oluşturun
chmod +x init-vault.sh
./init-vault.sh
```

#### Manuel Kurulum

```bash
# Bağımlılıkları yükleyin
cargo build --release

# Seçenek 1: Vault kullanarak (Önerilen)
export VAULT_ADDR=http://localhost:8200
export VAULT_TOKEN=myroot
export STORAGE_PATH=./uploads

# Seçenek 2: .env dosyası ile (Fallback)
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
├── src/
│   └── main.rs          # Ana uygulama kodu
├── Cargo.toml           # Rust bağımlılıkları
├── Dockerfile           # Docker yapılandırması
├── docker-compose.yml   # Docker Compose ayarları (Vault + CDN)
├── init-vault.sh        # Vault ilklendirme script'i
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
- 🔒 **HashiCorp Vault Integration**: Secrets are securely stored in Vault

### 🔐 HashiCorp Vault Integration

This project is integrated with **HashiCorp Vault** to securely store sensitive information (such as JWT_SECRET).

#### Vault Setup

```bash
# Start Vault and CDN service with Docker Compose
docker-compose up -d

# Initialize Vault (creates secrets)
./init-vault.sh
```

#### Reading Secrets from Vault

When the application starts, JWT_SECRET is automatically read from Vault:

```bash
export VAULT_ADDR=http://localhost:8200
export VAULT_TOKEN=myroot
cargo run --release
```

#### Viewing Secrets in Vault

```bash
# Connect to Vault
export VAULT_ADDR=http://localhost:8200
export VAULT_TOKEN=myroot

# View secrets
docker exec -it vault_server vault kv get secret/cdn-service
```

#### Fallback Mechanism

When Vault is not accessible, the application uses the `JWT_SECRET` value from the `.env` file:

```bash
echo "JWT_SECRET=your_secret_key_here" > .env
```

### 🔧 Technologies

- **Rust** - Systems programming language
- **Actix-web** - High-performance web framework
- **JWT** - Token-based authentication
- **HashiCorp Vault** - Secret management and secure storage
- **Docker** - Containerization
- **Tokio** - Asynchronous runtime

### 📋 Requirements

- Rust 1.84 or higher
- Docker & Docker Compose (optional)
- HashiCorp Vault (automatically installed with Docker Compose)
- `JWT_SECRET` variable in `.env` file (if not using Vault)

### 🚀 Installation and Running

#### With Docker (Recommended)

```bash
# Clone the project
git clone https://github.com/asimcanuz/Test.git
cd Test

# Create uploads folder
mkdir -p uploads

# Start with Docker Compose (Vault and CDN service)
docker-compose up -d

# Initialize Vault and create secrets
chmod +x init-vault.sh
./init-vault.sh
```

#### Manual Installation

```bash
# Install dependencies
cargo build --release

# Option 1: Using Vault (Recommended)
export VAULT_ADDR=http://localhost:8200
export VAULT_TOKEN=myroot
export STORAGE_PATH=./uploads

# Option 2: Using .env file (Fallback)
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
├── src/
│   └── main.rs          # Main application code
├── Cargo.toml           # Rust dependencies
├── Dockerfile           # Docker configuration
├── docker-compose.yml   # Docker Compose settings (Vault + CDN)
├── init-vault.sh        # Vault initialization script
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