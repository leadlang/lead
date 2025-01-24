#!/bin/sh

BLUE="\033[34m"
RED="\033[31m"
GREEN="\033[32m"
ENDCOLOR="\033[0m"

info="${BLUE}[INFO]${ENDCOLOR}"
err="${RED}[ERRR]${ENDCOLOR}"
succ="${GREEN}[SUCC]${ENDCOLOR}"

targets="x86_64-pc-windows-msvc
i686-pc-windows-msvc
aarch64-pc-windows-msvc
x86_64-unknown-linux-gnu
x86_64-20.04-linux-gnu
aarch64-unknown-linux-gnu
x86_64-unknown-linux-musl
aarch64-unknown-linux-musl
x86_64-apple-darwin
aarch64-apple-darwin
i686-unknown-linux-gnu
armv7-unknown-linux-gnueabi
armv7-unknown-linux-gnueabihf
arm-unknown-linux-gnueabi
arm-unknown-linux-gnueabihf
armv5te-unknown-linux-gnueabi
armv5te-unknown-linux-musleabi
armv7-unknown-linux-musleabi
armv7-unknown-linux-musleabihf
arm-unknown-linux-musleabi
arm-unknown-linux-musleabihf
mips-unknown-linux-gnu
mips64-unknown-linux-gnuabi64
mips64el-unknown-linux-gnuabi64
mipsel-unknown-linux-gnu
loongarch64-unknown-linux-gnu
loongarch64-unknown-linux-musl
powerpc-unknown-linux-gnu
powerpc64-unknown-linux-gnu
powerpc64le-unknown-linux-gnu
x86_64-unknown-freebsd
aarch64-unknown-freebsd
i686-unknown-freebsd
x86_64-unknown-netbsd
x86_64-unknown-illumos
x86_64-unknown-dragonfly
aarch64-linux-android
armv7-linux-androideabi
i686-linux-android
x86_64-linux-android"

# Page size
page_size=10
# Convert the targets into a list of lines (using only POSIX-compliant tools)
line_count=0
for target in $targets; do
  line_count=$((line_count + 1))
done

# Calculate total number of pages
total_pages=$((line_count / page_size))
if [ $((line_count % page_size)) -ne 0 ]; then
  total_pages=$((total_pages + 1))
fi

# Function to show targets for a given page
clear
show_page() {
  page=$1
  start_line=$((page * page_size + 1))
  end_line=$((start_line + page_size - 1))

  # Counter for target selection
  counter=1
  line_number=1

  printf "$info Page $((page+1))/$total_pages\n"
  printf "$info Select your target\n"
  # Loop through all targets
  for target in $targets; do
    if [ "$line_number" -ge "$start_line" ] && [ "$line_number" -le "$end_line" ]; then
      if [ "$counter" -lt "10" ]; then
        printf " ${GREEN}$counter.${ENDCOLOR} $target\n"
      else
        printf "${GREEN}$counter.${ENDCOLOR} $target\n"
      fi

      counter=$((counter + 1))
    fi
    line_number=$((line_number + 1))
  done
}

current_page=0
clear
show_page $current_page

while :; do
  printf "Enter a ${RED}number${ENDCOLOR} to select a target, ${BLUE}'n'${ENDCOLOR} for next page, ${BLUE}'p'${ENDCOLOR} for previous page, or ${RED}'q'${ENDCOLOR} to quit.\n"
  printf "$ "
  read -r input

  case "$input" in
    [0-9]*)
      # Check if the input is within range
      target_number=$((input))
      target_line=$((target_number + current_page * page_size))

      # Counter for line selection
      line_number=1
      selected_target=""
      for target in $targets; do
        if [ "$line_number" -eq "$target_line" ]; then
          selected_target="$target"
          break
        fi
        line_number=$((line_number + 1))
      done

      if [ -n "$selected_target" ]; then
        clear
        printf "$info You selected: $selected_target\n"
        break
      else
        printf "$err Invalid selection. Try again.\n"
      fi
      ;;
    n)
      clear
      # Next page
      if [ $current_page -lt $((total_pages - 1)) ]; then
        current_page=$((current_page + 1))
        show_page $current_page
      else
        show_page $current_page
        printf "$err ${RED}You are already on the last page.${ENDCOLOR}\n"
      fi
      ;;
    p)
      clear
      # Previous page
      if [ $current_page -gt 0 ]; then
        current_page=$((current_page - 1))
        show_page $current_page
      else
        show_page $current_page
        printf "$err ${RED}You are already on the first page.${ENDCOLOR}\n"
      fi
      ;;
    q)
      # Quit
      printf "$info Quitting.\n"
      exit 0
      ;;
    *)
      clear
      show_page $current_page
      printf "$err Invalid input. Please enter a valid number, 'n', 'p', or 'q'.\n"
      ;;
  esac
done

target="$selected_target"
tag_name="${TAG_NAME:-latest}"

printf "$info Found Lead Language Version: %s\n" "$tag_name"

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