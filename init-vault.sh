#!/bin/bash
# HashiCorp Vault İlklendirme Script'i (Vault Initialization Script)
# Bu script Vault'ta gerekli secret'ları oluşturur

set -e

echo "⏳ Vault'un hazır olması bekleniyor..."
sleep 5

# Vault CLI ortam değişkenlerini ayarla
export VAULT_ADDR='http://localhost:8200'
export VAULT_TOKEN='myroot'

echo "🔧 Vault'ta KV v2 secret engine'i aktifleştiriliyor..."
vault secrets enable -path=secret kv-v2 2>/dev/null || echo "Secret engine zaten aktif"

echo "🔐 JWT_SECRET oluşturuluyor..."
# Güçlü rastgele bir JWT secret oluştur
JWT_SECRET=$(openssl rand -base64 32)

# Secret'ı Vault'a kaydet
vault kv put secret/cdn-service jwt_secret="$JWT_SECRET"

echo "✅ Vault başarıyla yapılandırıldı!"
echo "📋 Secret'ları görüntülemek için: vault kv get secret/cdn-service"
