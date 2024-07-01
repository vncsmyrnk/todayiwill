#!/bin/bash

apt-get update -qq
apt-get install -qq -y curl wget

# Set up git
bash <(curl -s https://raw.githubusercontent.com/vncsmyrnk/git-config/main/install-apt.sh) "$GIT_USERNAME" "$GIT_EMAIL" --config-only

# Set up neovim
bash <(curl -s https://raw.githubusercontent.com/vncsmyrnk/vim-config/rust/install-apt.sh)

# Remove log files
rm .*log
