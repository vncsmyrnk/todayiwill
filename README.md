# todayiwill

[![CI workflow](https://github.com/vncsmyrnk/todayiwill/actions/workflows/ci.yml/badge.svg)](https://github.com/vncsmyrnk/todayiwill/actions/workflows/ci.yml)
[![Release workflow](https://github.com/vncsmyrnk/todayiwill/actions/workflows/release.yml/badge.svg)](https://github.com/vncsmyrnk/todayiwill/actions/workflows/release.yml)
![Rust](https://img.shields.io/badge/rust-1.79+-green?logo=rust)
[![codecov](https://codecov.io/gh/vncsmyrnk/todayiwill/graph/badge.svg?token=WN27CKCC6W)](https://codecov.io/gh/vncsmyrnk/todayiwill)
![GitHub Actions](https://img.shields.io/badge/GitHub%20Actions-gray?logo=githubactions)
[![contributions](https://img.shields.io/badge/contributions-welcome-brightgreen.svg?style=flat)](https://github.com/vncsmyrnk/todayiwill/issues)
[![Issue count](https://img.shields.io/github/issues-search?query=repo%3Avncsmyrnk%2Ftodayiwill%20is%3Aopen&label=open%20issues)](https://github.com/vncsmyrnk/todayiwill/issues)

A CLI reminder app that offers a simple yet powerful solution to enhance productivity and ensure that you stay on top of your daily responsibilities. Unlike complex project management tools, a CLI app provides a lightweight and straightforward interface that integrates seamlessly into the workflows of developers, system administrators, and tech-savvy users who spend a significant portion of their day in the terminal.

## Installation

### Ubuntu/debian

```bash
bash <(wget -O- https://github.com/vncsmyrnk/todayiwill/releases/latest/download/install-linux-debian.sh 2> /dev/null)
```

## Examples

```bash
$ todayiwill add --description "Take my dog to the vet" --time "14:00"
Appointment added successfully
```

```bash
$ todayiwill list
14:00 Take my dog to the vet
```

## Coming soon...

Check the [opened issues](https://github.com/vncsmyrnk/todayiwill/issues) section to know what will soon be available.

## Development with docker

```bash
docker run --rm -it \
    -v "$(pwd)":/opt/app \
    -v ~/.ssh:/root/.ssh \
    -e GIT_USERNAME="$(git config --list | grep "user.name" | cut -d = -f2)" \
    -e GIT_EMAIL="$(git config --list | grep "user.email" | cut -d = -f2)" \
    --cpus 2 \
    --workdir /opt/app \
    rust:1.79-slim bash
```
