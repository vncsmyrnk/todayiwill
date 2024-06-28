FROM ubuntu:22.04
SHELL ["/bin/bash", "-c"]
RUN <<EOF
apt-get update -qq
apt-get install -qq -y curl wget build-essential sudo
useradd -m -d /home/dev -s /bin/bash -u 1000 dev
adduser dev sudo
echo '%sudo ALL=(ALL) NOPASSWD:ALL' >> /etc/sudoers
EOF
USER dev
RUN <<EOF
sh <(curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs) -y
source $HOME/.cargo/env
rustup component add fmt clippy
EOF
WORKDIR /home/dev
LABEL description="This is a docker image that offers tools for \
developing rust applications as a non root user"
