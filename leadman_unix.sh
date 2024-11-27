#!/usr/bin/env bash

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

if [[ $os == 'Linux' || $os == 'Darwin' || $os == 'FreeBSD' ]]; then
  echo -e "$info $os detected"

  case "$arch" in
    x86_64)
      if [[ $os == 'FreeBSD' ]]; then
        echo -e "$info Lead Docs will fallback to using CLI on FreeBSD systems"
      fi

      target="${arch}-unknown-linux-gnu"
      [[ $os == 'Darwin' ]] && target="${arch}-apple-darwin"
      [[ $os == 'FreeBSD' ]] && target="${arch}-unknown-freebsd"

      echo -e "$info Getting Leadman $target"
      download="https://github.com/ahq-softwares/lead/releases/$([[ $tag_name == 'latest' ]] && echo 'latest/download' || echo "download/$tag_name")/leadman_$target"
      ;;
    aarch64)
      if [[ $os == 'FreeBSD' ]]; then
        echo -e "$err aarch64 version of Lead Lang is not supported on FreeBSD"
        exit 1
      elif [[ $os == 'Linux' ]]; then
        echo -e "$info Lead Docs will fallback to using CLI on Linux aarch64 systems"
      fi
      
      target="${arch}-apple-darwin"
      [[ $os == 'Linux' ]] && target="${arch}-unknown-linux-gnu"

      echo -e "$info Getting Leadman $target"
      download="https://github.com/ahq-softwares/lead/releases/$([[ $tag_name == 'latest' ]] && echo 'latest/download' || echo "download/$tag_name")/leadman_$target"
      ;;
    i386)
      if [[ $os == 'FreeBSD' ]]; then
        echo -e "$err Lead Docs will fallback to using CLI on FreeBSD 32 bit systems"
      elif [[ $os == 'Linux' ]]; then
        echo -e "$info Lead Docs will fallback to using CLI on FreeBSD 32 bit systems"
      fi
      
      target="${arch}-unknown-freeebsd"
      [[ $os == 'Linux' ]] && target="${arch}-unknown-linux-gnu"

      echo -e "$info Getting Leadman $target"
      download="https://github.com/ahq-softwares/lead/releases/$([[ $tag_name == 'latest' ]] && echo 'latest/download' || echo "download/$tag_name")/leadman_$target"
      ;;
    *)
      echo -e "$err Unsupported architecture: $arch"
      exit 1
      ;;
  esac
else
  echo -e "$err Unsupported OS: $os"
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