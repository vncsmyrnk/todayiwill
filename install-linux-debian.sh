#!/bin/bash

stderr_file=~/.todayiwill-install-error-log
stdout_file=/dev/null

handle_error() {
  echo -e "\033[1;31m[Error]\033[0m $1"
  echo -e "Check file \033[1m$stderr_file\033[0m for more info. Make sure to uninstall previous installations"
  exit 1
}

mkdir ~/.todayiwill \
  > $stdout_file \
  2> $stderr_file \
  || handle_error "Failed to create app folder"

echo -e "\033[1mDownloading latest version...\033[0m"
wget -P ~/.todayiwill https://github.com/vncsmyrnk/todayiwill/releases/latest/download/todayiwill \
  > $stdout_file \
  2> $stderr_file \
  || handle_error "Failed to download latest version"

echo -e "\033[1mInstalling...\033[0m"
{
  chmod u+x ~/.todayiwill/todayiwill
  echo -e "\n# todayiwill path\nexport PATH=\$PATH:~/.todayiwill" >> ~/.bashrc
} > $stdout_file 2> $stderr_file || handle_error "Failed to install latest version"

echo -e "\033[1;32m\nInstallation completed\033[0m"
echo -e "Run \033[1msource ~/.bashrc\033[0m for the command \033[1mtodayiwill\033[0m to be available"

exit 0
