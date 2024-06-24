# todayiwill

[![CI workflow](https://github.com/vncsmyrnk/todayiwill/actions/workflows/ci.yml/badge.svg)](https://github.com/vncsmyrnk/todayiwill/actions/workflows/ci.yml)
[![Release workflow](https://github.com/vncsmyrnk/todayiwill/actions/workflows/release.yml/badge.svg)](https://github.com/vncsmyrnk/todayiwill/actions/workflows/release.yml)
![Rust](https://img.shields.io/badge/rust-1.79+-green?logo=rust)
![GitHub Actions](https://img.shields.io/badge/GitHub%20Actions-gray?logo=githubactions)
[![contributions welcome](https://img.shields.io/badge/contributions-welcome-brightgreen.svg?style=flat)](https://github.com/vncsmyrnk/todayiwill/issues)
![Issue count](https://img.shields.io/github/issues-search?query=repo%3Avncsmyrnk%2Ftodayiwill&label=open%20issues)

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
