echo "REDIS_PASSWORD_1=$(openssl rand -base64 24)" >> .env
echo "REDIS_PASSWORD_2=$(openssl rand -base64 24)" >> .env
echo "REDIS_PASSWORD_3=$(openssl rand -base64 24)" >> .env


docker compose up -d --build