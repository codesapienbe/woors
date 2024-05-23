# WooRS

## Introduction

![woors](https://raw.githubusercontent.com/codesapienbe/woors/main/logo.svg)

## What does this application do for your website?
What will docker-compose contain?
- Database: `mysql/mariadb`
- Website: `wordpress`
- Admin: `phpmyadmin`
- Proxy: `traefik`
- Cache: `redis`
- Monitoring: `cadvisor`
- Management: `portainer`
- Vault: `a tool for securely accessing secrets`
- Code: `online vscode`
- Application: `a cross-platform native application of your webshop`
- Graphviz: `a tool for visualizing the architecture of docker-compose stack`


## CLI

## Generate Docker Compose File

```bash
cargo run --release -- docker-compose \
  --site-title mydemowebsite \
  --site-url mydemowebsite.com \
  --site-profile dev
```

## API (Actix Web)

### Generate Docker Compose File for testing purposes 

```bash
mkdir -p /tmp/test
cd /tmp/test
```


```bash
curl -X 'GET' \
  'http://localhost:8888/dc' \
  -H 'accept: application/json' \
  -H 'Content-Type: text/plain' \
    -d '{
    "site_title": "mydemowebsite",
    "site_url": "mydemowebsite.com",
    "site_profile": "dev"
}'
' >> docker-compose.yml
```
