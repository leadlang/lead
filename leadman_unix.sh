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

if [ "$os" = 'Linux' ] || [ "$os" = 'Darwin' ] || [ "$os" = 'FreeBSD' ] || [ "$os" = 'NetBSD' ] || [ "$os" = 'DragonFly' ]; then
  printf "$info $os detected\n"

  case "$arch" in
    x86_64|x86-64|amd64|AMD64)
      arch="x86_64"
      if [ "$os" = 'FreeBSD' ] || [ "$os" = 'NetBSD' ] || [ "$os" = 'DragonFly' ]; then
        printf "$info Lead Docs will fallback to using CLI on BSD systems\n"
      fi

      target="${arch}-unknown-linux-gnu"
      [ "$os" = 'Darwin' ] && target="${arch}-apple-darwin"
      [ "$os" = 'FreeBSD' ] && target="${arch}-unknown-freebsd"
      [ "$os" = 'NetBSD' ] && target="${arch}-unknown-netbsd"
      ;;
    aarch64|arm64|AArch64)
      arch="aarch64"
      if [ "$os" = 'NetBSD' ] || [ "$os" = 'DragonFly' ]; then
        printf "$err No prebuilt build for aarch64 $os\n"
        exit 1
      elif [ "$os" = 'FreeBSD' ] || [ "$os" = 'Linux' ]; then
        printf "$info Lead Docs will fallback to using CLI on $os aarch64 systems\n"
      fi
      
      target="${arch}-apple-darwin"
      [ "$os" = 'FreeBSD' ] && target="${arch}-unknown-freebsd"
      [ "$os" = 'Linux' ] && target="${arch}-unknown-linux-gnu"
      ;;
    aarch32|armv7l|armv6l|armv7|armv6)
      arch="armv7"
      if [ "$os" = 'NetBSD' ] || [ "$os" = 'FreeBSD' ] || [ "$os" = 'DragonFly' ]; then
        printf "$err No prebuilt build for aarch32 $os\n"
        exit 1
      elif [ "$os" = 'Linux' ] || [ "$os" = 'DragonFly' ]; then
        printf "$info Lead Docs will fallback to using CLI on $os armv7 systems\n"
      fi
      
      # target="${arch}-apple-darwin"
      # [[ "$os" = 'FreeBSD' ]] && target="${arch}-unknown-freebsd"
      [ "$os" = 'Linux' ] && target="${arch}-unknown-linux-gnu"
      ;;
    i386|i486|i586|i686)
      arch="i686"
      if [ "$os" = 'NetBSD' ] || [ "$os" = 'FreeBSD' ]; then
        printf "$err No prebuilt build for x86 $os\n"
        exit 1
      else
        printf "$err Lead Docs will fallback to using CLI on $os 32 bit systems\n"
      fi
      
      target="${arch}-unknown-freebsd"
      [ "$os" = 'Linux' ] && target="${arch}-unknown-linux-gnu"
      ;;
    *)
      clear
      printf "$warn This script cannot detect settings for $arch $os\n"
      printf "$err Using target select install script\n"
      
      curl -fsSl
      tmp=$(mktemp -d 2>/dev/null || mktemp -d -t 'mytmpdir')

      curl -fsSL "https://raw.githubusercontent.com/leadlang/lead/refs/heads/main/leadman_unix_avd.sh" -o "$tmp/inst.sh"
      chmod +x "$tmp/inst.sh"
      sh "$tmp/inst.sh"

      rm "$tmp/inst.sh"
      rmdir "$tmp"
      exit 0
      ;;
  esac
else
  if [ "$os" = 'SunOS' ] && [ "$(grep -ic solaris /etc/os-release)" -eq '0' ]; then
    printf "$info Illumos detected\n"

    if [ "$arch" != 'i86pc' ]; then
      printf "$err No prebuilt binaries for $arch $os\n"
      exit 1
    fi

    arch="x86_64"
    target="x86_64-unknown-illumos"
  else
    printf "$err No prebuilt binaries for $arch $os\n"
    exit 1
  fi
fi

printf "$info Getting Leadman %s\n" "$target"
download="https://github.com/leadlang/lead/releases/$( ([ "$tag_name" = 'latest' ] && echo 'latest/download') || echo "download/$tag_name")/leadman_$target"

tmp=$(mktemp -d 2>/dev/null || mktemp -d -t 'mytmpdir')

printf "$info Downloading Leadman $download \n"

curl -L "$download" -o "$tmp/leadman_init"

chmod +x "$tmp/leadman_init"

printf "$info Starting leadman \n"

"$tmp/leadman_init" create

rm "$tmp/leadman_init"
rmdir "$tmp"