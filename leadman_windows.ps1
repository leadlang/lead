$ErrorActionPreference = "Stop"

$INFO = "$($PSStyle.Foreground.Blue)[INFO]$($PSStyle.Reset)"
$ERR = "$($PSStyle.Foreground.Red)$($PSStyle.Bold)[ERRR]$($PSStyle.Reset)"
$SUCC = "$($PSStyle.Foreground.Green)$($PSStyle.Bold)[SUCC]$($PSStyle.Reset)"

$architecture = [System.Environment]::GetEnvironmentVariable("PROCESSOR_ARCHITECTURE")
$tag = [System.Environment]::GetEnvironmentVariable("TAG_NAME")

"$INFO Checking OS"

if (![System.OperatingSystem]::IsWindows()) {
  Write-Err "$ERR Unsupported Operating System, run this in $($PSStyle.Bold)Windows$($PSStyle.Reset) or use the $($PSStyle.Bold)bash script$($PSStyle.Reset)"
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

$arch = switch ($architecture) {
  "AMD64" {
    "x86_64"
    break
  }
  "ARM64" {
    "aarch64"
    break
  }
  "x86" {
    "i686"
    break
  }
  default {
    Write-Err "$ERR Unknown architecture $architecture"
    exit 1
  }
}

"$INFO Found Architecture $arch"

if ($tag -eq "latest") {
  $tag = "latest/download"
}
else {
  $tag = "download/$tag"
}

$DOWNLOAD = "https://github.com/leadlang/lead/releases/$tag/leadman_$arch-pc-windows-msvc.exe"

Invoke-WebRequest -Uri $DOWNLOAD -OutFile "$env:TEMP\leadman_init.exe"; "$INFO Starting leadman"; ""

$result = Start-Process -Wait -NoNewWindow -FilePath "$env:TEMP\leadman_init.exe" -ArgumentList "create" -PassThru

if ($result.ExitCode -eq 0) {
  "$SUCC Successfully installed 🎉"
  Start-Sleep -Seconds 1
}