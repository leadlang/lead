$ErrorActionPreference = "Stop"

$INFO = "$($PSStyle.Foreground.Blue)[INFO]$($PSStyle.Reset)"
$ERR = "$($PSStyle.Foreground.Red)$($PSStyle.Bold)[ERRR]$($PSStyle.Reset)"
$SUCC = "$($PSStyle.Foreground.Green)$($PSStyle.Bold)[SUCC]$($PSStyle.Reset)"

$architecture = [System.Environment]::GetEnvironmentVariable("PROCESSOR_ARCHITECTURE")
$tag = [System.Environment]::GetEnvironmentVariable("TAG_NAME")

"$INFO Checking OS"

if (![System.OperatingSystem]::IsWindows()) {
  Write-Err "$ERR Unsupported Operating System, use $($PSStyle.Bold)Windows$($PSStyle.Reset) or use the bash script"
  exit 1
}

"$INFO Getting architecture"

if ($tag.Length -eq 0) {
  "$INFO Using latest as version"

  $tag = "latest"
}
else {
  "$INFO Provided Leadman Version $tag"
}

"$INFO Found Leadman Version $tag"

if ($architecture -eq "AMD64") {
  "$INFO Getting Leadman x86_64-pc-windows-msvc"
  
  if ($tag -eq "latest") {
    $DOWNLOAD = "https://github.com/ahq-softwares/lead/releases/latest/download/leadman_x86_64-pc-windows-msvc.exe"
  }
  else {
    $DOWNLOAD = "https://github.com/ahq-softwares/lead/releases/download/$tag/leadman_x86_64-pc-windows-msvc.exe"
  }
}
elseif ($architecture -eq "ARM64") {
  "$INFO Getting Leadman aarch64-pc-windows-msvc"

  if ($tag -eq "latest") {
    $DOWNLOAD = "https://github.com/ahq-softwares/lead/releases/latest/download/leadman_aarch64-pc-windows-msvc.exe"
  }
  else {
    $DOWNLOAD = "https://github.com/ahq-softwares/lead/releases/download/$tag/leadman_aarch64-pc-windows-msvc.exe"
  }
}
else {
  Write-Err "$ERR Unknown architecture $architecture"
  exit 1
}

Invoke-WebRequest -Uri $DOWNLOAD -OutFile "$env:TEMP\leadman_init.exe"
"$INFO Starting leadman"
""

$result = Start-Process -Wait -NoNewWindow -FilePath "$env:TEMP\leadman_init.exe" -ArgumentList "create" -PassThru

if ($result.ExitCode -eq 0) {
  "$SUCC Successfully installed 🎉"
  Start-Sleep -Seconds 1
}