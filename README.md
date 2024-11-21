apt install tree

helloworldはマルチステージビルド

## appの起動
docker compose up app --build

## redis, postgresの起動
docker compose up -d redis postgres

### 確認
apt install redis-tools
apt install postgresql-client

redis-cli -h localhost -p 6379
redis-cli -h 192.168.68.7 -p 6379   // devcontainer

psql "postgresql://192.168.68.7:5432/app?user=app&password=passwd"

## cargo-make
cargo install cargo-make

makersでも起動できる

cargo make --version  
makers --version  