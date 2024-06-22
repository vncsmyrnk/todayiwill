![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
<br>
[![CI workflow](https://github.com/vncsmyrnk/todayiwill/actions/workflows/ci.yml/badge.svg)](https://github.com/vncsmyrnk/todayiwill/actions/workflows/ci.yml)
[![Release workflow](https://github.com/vncsmyrnk/todayiwill/actions/workflows/release.yml/badge.svg)](https://github.com/vncsmyrnk/todayiwill/actions/workflows/release.yml)

# todayiwill

CLI app for describing and reminding what you must do today.

## Run

```bash
wget https://github.com/vncsmyrnk/todayiwill/releases/latest/download/todayiwill
chmod u+x todayiwill
todayiwill --version
```

## Development with docker

```bash
docker run --rm -it \
    -v "$(pwd)":/opt/app \
    -v ~/.ssh:/root/.ssh \
    --cpus 2 \
    --workdir /opt/app \
    rust:1.79-slim bash
```
