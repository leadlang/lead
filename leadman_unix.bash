#!/bin/bash

os=$(uname)
arch=$(uname -m)

BLUE="\e[34m"
RED="\e[31m"
GREEN="\e[32m"
ENDCOLOR="\e[0m"

info="${BLUE}[INFO]${ENDCOLOR}"
err="${RED}[ERRR]${ENDCOLOR}"
succ="${GREEN}[SUCC]${ENDCOLOR}"

download=""

echo -e $info Checking OS

tag_name=$(([[ $TAG_NAME != "" ]] && echo $TAG_NAME) || echo "latest")

echo -e $info Found Lead Language Version: $tag_name

if [[ $os == 'Linux' ]]; then
  echo -e $info Linux Detected
  if [[ $arch == 'x86_64' ]]; then
    echo -e $info Getting Leadman x86_64-unknown-linux-gnu

    download=$(([[ $tag_name == "latest" ]] && echo "https://github.com/ahq-softwares/lead/releases/latest/download/leadman_x86_64-unknown-linux-gnu") || echo "https://github.com/ahq-softwares/lead/releases/download/$tag_name/leadman_x86_64-unknown-linux-gnu")
  elif [[ $arch == 'aarch64' ]]; then
    echo -e $err aarch64 version of lead lang is not yet supported on Linux
    exit 1

    download=$(([[ $tag_name == "latest" ]] && echo "https://github.com/ahq-softwares/lead/releases/latest/download/leadman_aarch64-unknown-linux-gnu") || echo "https://github.com/ahq-softwares/lead/releases/download/$tag_name/leadman_aarch64-unknown-linux-gnu")
  else
    echo -e $err Unsupported architecture $arch
    exit 1
  fi
elif [[ $os == 'Darwin' ]]; then
  echo -e $info MacOS detected
  if [[ $arch == 'x86_64' ]]; then
    echo -e $info Getting Leadman x86_64-apple-darwin
    
    download=$(([[ $tag_name == "latest" ]] && echo "https://github.com/ahq-softwares/lead/releases/latest/download/leadman_x86_64-apple-darwin") || echo "https://github.com/ahq-softwares/lead/releases/download/$tag_name/leadman_x86_64-apple-darwin")
  elif [[ $arch == 'aarch64' ]]; then
    echo -e $info Getting Leadman aarch64-apple-darwin

    download=$(([[ $tag_name == "latest" ]] && echo "https://github.com/ahq-softwares/lead/releases/latest/download/leadman_aarch64-apple-darwin") || echo "https://github.com/ahq-softwares/lead/releases/download/$tag_name/leadman_aarch64-apple-darwin")
  else
    echo -e $err Unsupported architecture $arch
    exit 1
  fi
else
  echo -e $err Unsupported OS $os
  exit 1
fi

tmp=$(mktemp -d 2>/dev/null || mktemp -d -t 'mytmpdir')

echo -e $info Downloading Leadman $download

curl -L $download -o $tmp/leadman_init

chmod +x $tmp/leadman_init

echo -e $info Starting leadman

$tmp/leadman_init create

rm $tmp/leadman_init
rmdir $tmp

echo -e $succ Successfully installed 🎉