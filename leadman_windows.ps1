$ErrorActionPreference = "Stop"

$INFO = "$($PSStyle.Foreground.Blue)[INFO]$($PSStyle.Reset)"
$ERR = "$($PSStyle.Foreground.Red)$($PSStyle.Bold)[ERRR]$($PSStyle.Reset)"
$SUCC = "$($PSStyle.Foreground.Green)$($PSStyle.Bold)[SUCC]$($PSStyle.Reset)"

$architecture = $env:PROCESSOR_ARCHITECTURE
$tag = $env:TAG_NAME

"$INFO Checking OS"

if ([System.Environment]::OSVersion.Platform -ne "Win32NT") {
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

$isWin11 = (Get-WmiObject Win32_OperatingSystem).Caption -Match "Windows 11"

"$INFO Found Leadman Version $tag"

switch ($architecture) {
  "AMD64" {
    $arch = "x86_64"
    break
  }
  "ARM64" {
    $arch = "aarch64"

    if ($isWin11) {
      "$INFO Lead Language will soon introduce ARM64EC support for Windows 11 once Microsoft supports it on GitHub"
      #Write-Host -NoNewline "$INFO Would you like to use the ARM64EC version? [Y/n]"
      #$ask = (Read-Host).ToLower()
      
      #if ($ask -eq "y") {
      #  $arch = "arm64ec"
      #}
    }
    break
  }
  "x86" {
    $arch = "i686"
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

$nt = [Environment]::OSVersion.Version.Major

$DOWNLOAD = "https://github.com/leadlang/lead/releases/$tag/leadman_$arch-pc-windows-msvc.exe"

if ($nt -eq 10) {
  "$INFO Using win7 compat binary as NT Version $nt is less than NT 10.0"
  $DOWNLOAD = "https://github.com/leadlang/lead/releases/$tag/leadman_$arch-win7-windows-msvc.exe"
}

Invoke-WebRequest -Uri $DOWNLOAD -OutFile "$env:TEMP\leadman_init.exe"; "$INFO Starting leadman"; ""

$result = Start-Process -Wait -NoNewWindow -FilePath "$env:TEMP\leadman_init.exe" -ArgumentList "create" -PassThru

if ($result.ExitCode -eq 0) {
  "$SUCC Successfully installed"
  Start-Sleep -Seconds 1
}

$ver = $PSVersionTable.PSVersion.Major

if (-not $PSVersionTable -or $ver -lt 7) {
  "$INFO We recommend you to upgrade to PowerShell 7.0 or higher. Current version: $($PSVersionTable.PSVersion)"
}