#!/bin/bash

stderr_file=~/.todayiwill-uninstall-error-log
stdout_file=/dev/null

handle_error() {
  echo -e "\033[1;31m[Error]\033[0m $1"
  echo -e "Check file \033[1m$stderr_file\033[0m for more info"
  exit 1
}

echo -e "\033[1mRemoving app files...\033[0m"
{
  rm -rf ~/.todayiwill*
  sed -i '/^# todayiwill*/d' ~/.bashrc
  sed -i '/^export PATH=$PATH:~\/.todayiwill/d' ~/.bashrc
} > $stdout_file 2> $stderr_file || handle_error "Failed to remove app files"

echo -e "\033[1;32m\ntodayiwill successfully uninstalled\033[0m"

exit 0
