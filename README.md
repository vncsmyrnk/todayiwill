![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)

# todayiwill

CLI app for describing and reminding what you must do today

## Development with docker

```bash
docker run --rm -it \
    -v "$(pwd)":/opt/app \
    -v ~/.ssh:/root/.ssh \
    --cpus 2 \
    --workdir /opt/app \
    rust:1.79-slim bash
```
