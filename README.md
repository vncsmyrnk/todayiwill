# todayiwill

[![CI workflow](https://github.com/vncsmyrnk/todayiwill/actions/workflows/ci.yml/badge.svg)](https://github.com/vncsmyrnk/todayiwill/actions/workflows/ci.yml)
[![Release workflow](https://github.com/vncsmyrnk/todayiwill/actions/workflows/release.yml/badge.svg)](https://github.com/vncsmyrnk/todayiwill/actions/workflows/release.yml)
![Rust](https://img.shields.io/badge/rust-1.79+-green?logo=rust)
[![codecov](https://codecov.io/gh/vncsmyrnk/todayiwill/graph/badge.svg?token=WN27CKCC6W)](https://codecov.io/gh/vncsmyrnk/todayiwill)
[![GitHub Actions](https://img.shields.io/badge/GitHub%20Actions-gray?logo=githubactions)](https://github.com/vncsmyrnk/todayiwill/actions)
[![contributions](https://img.shields.io/badge/contributions-welcome-brightgreen.svg?style=flat)](https://github.com/vncsmyrnk/todayiwill/issues)
[![Issue count](https://img.shields.io/github/issues-search?query=repo%3Avncsmyrnk%2Ftodayiwill%20is%3Aopen&label=open%20issues)](https://github.com/vncsmyrnk/todayiwill/issues)

A CLI reminder app that offers a simple yet powerful solution to enhance productivity and ensure that you stay on top of your daily responsibilities. Unlike complex project management tools, a CLI app provides a lightweight and straightforward interface that integrates seamlessly into the workflows of developers, system administrators, and tech-savvy users who spend a significant portion of their day in the terminal.

![gif demonstrating the utilization](https://raw.githubusercontent.com/vncsmyrnk/todayiwill/main/assets/demonst.gif)

## 🚀 Examples

### Documentation

```bash
$ todayiwill
```
```
A CLI for remembering what you need to do today.
Checkout the project on https://github.com/vncsmyrnk/todayiwill for submitting requests and rating the app.

Usage: todayiwill [OPTIONS] <COMMAND>

Commands:
  add      Add appointment for today
  copy     Copies the appointments from a specific date to today
  clear    Clear all the appointments added for today
  list     List the appointments to come for today
  history  List the appointments for other days
  remove   Removes a future appointment
  help     Print this message or the help of the given subcommand(s)

Options:
  -c, --current-time <HH:MM>  Current time, defaults to system time [default: 23:59]
  -h, --help                  Print help
  -V, --version               Print version
```
### Add appointments

```bash
$ todayiwill add --description "Take my dog to the vet" --time "14:00"
```
```
Appointment added successfully
```

```bash
$ echo "19:30 Wash the dishes" | todayiwill add --stdin
```
```
Appointment added successfully
```
### List appointments

```bash
$ todayiwill list
```
```
[14:00] Take my dog to the vet
```
## 💽 Install

![gif showing the installation process](https://raw.githubusercontent.com/vncsmyrnk/todayiwill/main/assets/install.gif)

All packages are made to `x86_64` architecture.

### Ubuntu/debian (bash)

```bash
bash <(wget -O- https://github.com/vncsmyrnk/todayiwill/releases/latest/download/install-linux-debian.sh 2> /dev/null)
```

### Ubuntu/debian (`deb` package)

```bash
curl -L -O https://github.com/vncsmyrnk/todayiwill/releases/latest/download/todayiwill_0.6.0_amd64.deb # or other version
sudo apt install ./todayiwill_0.6.0_amd64.deb
```

### Homebrew

```bash
brew tap vncsmyrnk/todayiwill https://github.com/vncsmyrnk/todayiwill.git
brew install vncsmyrnk/todayiwill/app
```

### Arch

```bash
curl  -L -O https://github.com/vncsmyrnk/todayiwill/releases/latest/download/PKGBUILD
makepkg -si
```

## 🪄 Coming soon...

Check the [opened issues](https://github.com/vncsmyrnk/todayiwill/issues) section to know what will soon be available.

Access the [closed PR](https://github.com/vncsmyrnk/todayiwill/pulls?q=is%3Apr+is%3Aclosed) section to know what was done so far.

## 🗑 Uninstall

### Ubuntu/debian

```bash
bash <(wget -O- https://github.com/vncsmyrnk/todayiwill/releases/latest/download/uninstall-linux-debian.sh 2> /dev/null)
```

## 🔧 Development with docker

```bash
docker run --rm -it \
    -v "$(pwd)":/home/dev/app \
    -v ~/.ssh:/home/dev/.ssh \
    -e GIT_USERNAME="$(git config --list | grep "user.name" | cut -d = -f2)" \
    -e GIT_EMAIL="$(git config --list | grep "user.email" | cut -d = -f2)" \
    -u dev \
    --cpus 2 \
    --workdir /home/dev/app \
    ghcr.io/vncsmyrnk/rust-dev:latest bash
```

### Documentation

Go to [docs](https://vncsmyrnk.github.io/todayiwill/) for checking out the code documentation.

### Dev Tools

Once inside the container, you can run `$ sudo -E ./dev-setup.sh` to install dev dependencies like `git` and `nvim`.
