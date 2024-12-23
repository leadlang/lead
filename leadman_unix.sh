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

if [[ $os == 'Linux' || $os == 'Darwin' || $os == 'FreeBSD' || $os == 'NetBSD' ]]; then
  echo -e "$info $os detected"

  case "$arch" in
    x86_64|x86-64|amd64|AMD64)
      arch="x86_64"
      if [[ $os == 'FreeBSD' || $os == 'NetBSD' ]]; then
        echo -e "$info Lead Docs will fallback to using CLI on BSD systems"
      fi

      target="${arch}-unknown-linux-gnu"
      [[ $os == 'Darwin' ]] && target="${arch}-apple-darwin"
      [[ $os == 'FreeBSD' ]] && target="${arch}-unknown-freebsd"
      [[ $os == 'NetBSD' ]] && target="${arch}-unknown-netbsd"

      echo -e "$info Getting Leadman $target"
      download="https://github.com/ahq-softwares/lead/releases/$([[ $tag_name == 'latest' ]] && echo 'latest/download' || echo "download/$tag_name")/leadman_$target"
      ;;
    aarch64|arm64|AArch64)
      arch="aarch64"
      if [[ $os == 'NetBSD' ]]; then
        echo -e "$err aarch64 version of lead lang is not supported on NetBSD"
        exit 1
      elif [[ $os == 'FreeBSD' || $os == 'Linux' ]]; then
        echo -e "$info Lead Docs will fallback to using CLI on $os aarch64 systems"
      fi
      
      target="${arch}-apple-darwin"
      [[ $os == 'FreeBSD' ]] && target="${arch}-unknown-freebsd"
      [[ $os == 'Linux' ]] && target="${arch}-unknown-linux-gnu"

      echo -e "$info Getting Leadman $target"
      download="https://github.com/ahq-softwares/lead/releases/$([[ $tag_name == 'latest' ]] && echo 'latest/download' || echo "download/$tag_name")/leadman_$target"
      ;;
    aarch32|armv7l|armv6l|armv7|armv6)
      arch="armv7"
      if [[ $os == 'NetBSD' || $os == 'FreeBSD' ]]; then
        echo -e "$err aarch32 version of lead lang is not supported on BSD systems"
        exit 1
      elif [[ $os == 'Linux' ]]; then
        echo -e "$info Lead Docs will fallback to using CLI on $os armv7 systems"
      fi
      
      # target="${arch}-apple-darwin"
      # [[ $os == 'FreeBSD' ]] && target="${arch}-unknown-freebsd"
      [[ $os == 'Linux' ]] && target="${arch}-unknown-linux-gnu"

      echo -e "$info Getting Leadman $target"
      download="https://github.com/ahq-softwares/lead/releases/$([[ $tag_name == 'latest' ]] && echo 'latest/download' || echo "download/$tag_name")/leadman_$target"
      ;;
    i386|i486|i586|i686)
      arch="i686"
      if [[ $os == 'NetBSD' ]]; then
        echo -e "$err 32-bit version of lead lang is not supported on BSD systems"
        exit 1
      else
        echo -e "$err Lead Docs will fallback to using CLI on $os 32 bit systems"
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