#! /bin/sh

os=$(uname)
arch=$(uname -m)

BLUE="\033[34m"
RED="\033[31m"
GREEN="\033[32m"
ENDCOLOR="\033[0m"

info="${BLUE}[INFO]${ENDCOLOR}"
err="${RED}[ERRR]${ENDCOLOR}"
succ="${GREEN}[SUCC]${ENDCOLOR}"

download=""

printf "$info Checking OS\n"

tag_name="${TAG_NAME:-latest}"

printf "$info Found Lead Language Version: %s\n" "$tag_name"

if [ "$os" = 'Linux' ] || [ "$os" = 'Darwin' ] || [ "$os" = 'FreeBSD' ] || [ "$os" = 'NetBSD' ]; then
  printf "$info $os detected\n"

  case "$arch" in
    x86_64|x86-64|amd64|AMD64)
      arch="x86_64"
      if [ "$os" = 'FreeBSD' ] || [ "$os" = 'NetBSD' ]; then
        printf "$info Lead Docs will fallback to using CLI on BSD systems\n"
      fi

      target="${arch}-unknown-linux-gnu"
      [ "$os" = 'Darwin' ] && target="${arch}-apple-darwin"
      [ "$os" = 'FreeBSD' ] && target="${arch}-unknown-freebsd"
      [ "$os" = 'NetBSD' ] && target="${arch}-unknown-netbsd"

      printf "$info Getting Leadman $target\n"
      download="https://github.com/leadlang/lead/releases/$( ([ "$tag_name" = 'latest' ] && echo 'latest/download') || echo "download/$tag_name")/leadman_$target"
      ;;
    aarch64|arm64|AArch64)
      arch="aarch64"
      if [ "$os" = 'NetBSD' ]; then
        printf "$err aarch64 version of lead lang is not supported on NetBSD\n"
        exit 1
      elif [ "$os" = 'FreeBSD' ] || [ "$os" = 'Linux' ]; then
        printf "$info Lead Docs will fallback to using CLI on $os aarch64 systems\n"
      fi
      
      target="${arch}-apple-darwin"
      [ "$os" = 'FreeBSD' ] && target="${arch}-unknown-freebsd"
      [ "$os" = 'Linux' ] && target="${arch}-unknown-linux-gnu"

      printf "$info Getting Leadman $target\n"
      download="https://github.com/leadlang/lead/releases/$( ([ "$tag_name" = 'latest' ] && echo 'latest/download') || echo "download/$tag_name")/leadman_$target"
      ;;
    aarch32|armv7l|armv6l|armv7|armv6)
      arch="armv7"
      if [ "$os" = 'NetBSD' ] || [ "$os" = 'FreeBSD' ]; then
        printf "$err aarch32 version of lead lang is not supported on BSD systems\n"
        exit 1
      elif [ "$os" = 'Linux' ]; then
        printf "$info Lead Docs will fallback to using CLI on $os armv7 systems\n"
      fi
      
      # target="${arch}-apple-darwin"
      # [[ "$os" = 'FreeBSD' ]] && target="${arch}-unknown-freebsd"
      [ "$os" = 'Linux' ] && target="${arch}-unknown-linux-gnu"

      printf "$info Getting Leadman $target"
      download="https://github.com/leadlang/lead/releases/$( ([ "$tag_name" = 'latest' ] && echo 'latest/download') || echo "download/$tag_name")/leadman_$target"
      ;;
    i386|i486|i586|i686)
      arch="i686"
      if [ "$os" = 'NetBSD' ]; then
        printf "$err 32-bit version of lead lang is not supported on BSD systems\n"
        exit 1
      else
        printf "$err Lead Docs will fallback to using CLI on $os 32 bit systems\n"
      fi
      
      target="${arch}-unknown-freeebsd"
      [ "$os" = 'Linux' ] && target="${arch}-unknown-linux-gnu"

      printf "$info Getting Leadman $target"
      download="https://github.com/leadlang/lead/releases/$( ([ "$tag_name" = 'latest' ] && echo 'latest/download') || echo "download/$tag_name")/leadman_$target"
      ;;
    *)
      printf "$err Unsupported architecture: $arch"
      exit 1
      ;;
  esac
else
  printf "$err Unsupported OS: $os"
  exit 1
fi


tmp=$(mktemp -d 2>/dev/null || mktemp -d -t 'mytmpdir')

printf "$info Downloading Leadman $download \n"

curl -L "$download" -o "$tmp/leadman_init"

chmod +x "$tmp/leadman_init"

printf "$info Starting leadman \n"

"$tmp/leadman_init" create

rm "$tmp/leadman_init"
rmdir "$tmp"

printf "$succ Successfully installed 🎉 \n"