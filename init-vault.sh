#!/bin/bash
# HashiCorp Vault İlklendirme Script'i (Vault Initialization Script)
# Bu script Vault'ta gerekli secret'ları oluşturur

set -e

echo "⏳ Vault'un hazır olması bekleniyor..."
sleep 10

# Vault CLI ortam değişkenlerini ayarla
export VAULT_ADDR='http://localhost:8200'
export VAULT_TOKEN='myroot'

echo "🔧 Vault'ta KV v2 secret engine'i aktifleştiriliyor..."
docker exec -e VAULT_ADDR=$VAULT_ADDR -e VAULT_TOKEN=$VAULT_TOKEN vault_server \
  vault secrets enable -path=secret kv-v2 2>/dev/null || echo "Secret engine zaten aktif"

echo "🔐 JWT_SECRET oluşturuluyor..."
# Güçlü rastgele bir JWT secret oluştur
JWT_SECRET=$(openssl rand -base64 32)

# Secret'ı Vault'a kaydet
docker exec -e VAULT_ADDR=$VAULT_ADDR -e VAULT_TOKEN=$VAULT_TOKEN vault_server \
  vault kv put secret/cdn-service jwt_secret="$JWT_SECRET"

echo "✅ Vault başarıyla yapılandırıldı!"
echo "📋 Secret'ları görüntülemek için:"
echo "    docker exec -e VAULT_ADDR=$VAULT_ADDR -e VAULT_TOKEN=$VAULT_TOKEN vault_server vault kv get secret/cdn-service"
