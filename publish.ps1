$v = cat .version

echo PowerShell

echo "TAG_NAME=$v" >> "$env:GITHUB_OUTPUT"