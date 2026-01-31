# Test

## Yüksek Trafik CDN Servisi / High Traffic CDN Service

Bu proje, yüksek trafik senaryoları için optimize edilmiş bir Rust CDN servisidir.  
This project is a Rust CDN service optimized for high traffic scenarios.

### 🚀 Yüksek Trafik Özellikleri / High Traffic Features

#### 1. **Rate Limiting (Oran Sınırlama)**
- Her IP için saniyede 2 istek limiti (2 requests per second per IP)
- 10 isteklik ani patlama (burst) kapasitesi (Burst capacity of 10 requests)
- Binlerce eşzamanlı istek geldiğinde servisi korur (Protects service from thousands of concurrent requests)

#### 2. **Response Compression (Yanıt Sıkıştırma)**
- Gzip, Deflate ve Brotli sıkıştırma desteği (Supports gzip, deflate, and brotli compression)
- Bant genişliği kullanımını %60-80 azaltır (Reduces bandwidth usage by 60-80%)
- Otomatik içerik türü algılama (Automatic content type detection)

#### 3. **Worker Pool (İşçi Havuzu)**
- CPU çekirdek sayısı kadar worker thread (Worker threads equal to CPU cores)
- Paralel istek işleme (Parallel request processing)
- Optimal kaynak kullanımı (Optimal resource utilization)

#### 4. **Connection Management (Bağlantı Yönetimi)**
- 75 saniyelik Keep-Alive timeout (75-second Keep-Alive timeout)
- Bağlantı yeniden kullanımı (Connection reuse)
- Kaynak tasarrufu (Resource conservation)

#### 5. **Resource Limits (Kaynak Limitleri)**
- Docker container için CPU ve RAM limitleri (CPU and RAM limits for Docker container)
- Maksimum 2 CPU çekirdeği, 1GB RAM (Maximum 2 CPU cores, 1GB RAM)
- Kontrollü kaynak kullanımı (Controlled resource usage)

#### 6. **Health Check Endpoint**
- `/health` endpoint ile servis durumu kontrolü (Service health check via `/health` endpoint)
- Monitoring ve load balancer entegrasyonu için (For monitoring and load balancer integration)

### 📊 Performans / Performance

Bu özellikler sayesinde:
- Binlerce eşzamanlı istek güvenle işlenebilir (Thousands of concurrent requests can be safely handled)
- %60-80 daha az bant genişliği kullanımı (%60-80 less bandwidth usage)
- Otomatik oran sınırlama ile DDoS koruması (DDoS protection with automatic rate limiting)
- CPU'ya göre otomatik ölçeklendirme (Automatic scaling based on CPU)

### 🔧 Kurulum / Installation

```bash
# Docker ile çalıştırma / Run with Docker
docker-compose up -d

# Manuel derleme / Manual build
cargo build --release
```

### 🏥 Health Check

Servisin sağlık durumunu kontrol etmek için:  
To check the service health:

```bash
curl http://localhost:8080/health
```

Yanıt / Response:
```json
{
  "status": "healthy",
  "service": "cdn-service"
}
```