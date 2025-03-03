# Go though every directory and run cargo update

Get-ChildItem -Directory | ForEach-Object { cargo update -v --manifest-path $_/Cargo.toml }