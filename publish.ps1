$v = cat .version

echo PowerShell

echo "TAG_NAME=$v" >> "$GITHUB_OUTPUT"