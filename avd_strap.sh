#! /bin/sh

tmp=$(mktemp -d 2>/dev/null || mktemp -d -t 'mytmpdir')

curl -fsSL https://leadlang.github.io/install_advanced.sh -o "$tmp/inst.sh"
chmod +x "$tmp/inst.sh"
sh "$tmp/inst.sh"

rm "$tmp/inst.sh"
rmdir "$tmp"
exit 0