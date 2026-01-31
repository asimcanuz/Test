#!/bin/bash

GREEN='\033[0;32m'
CYAN='\033[0;36m'
NC='\033[0m'

echo -e "${CYAN}🚀 GitHub Kurulumu Başlıyor...${NC}"

# Git kontrolü
if ! command -v git &> /dev/null; then
    echo "❌ Git yüklü değil!"
        exit 1
        fi

        # Klasör oluşturma (uploads)
        mkdir -p uploads
        touch uploads/.gitkeep

        git init
        git add .
        git commit -m "Initial commit: High-Performance Rust CDN"
        git branch -M main

        echo -e "${CYAN}🔗 GitHub Repo Linkini Yapıştırın:${NC}"
        read REPO_URL

        if [ -z "$REPO_URL" ]; then
            echo "❌ Link girmediniz!"
                exit 1
                fi

                git remote add origin "$REPO_URL"
                git push -u origin main

                echo -e "${GREEN}✅ İşlem Tamam!${NC}"