$ErrorActionPreference = "Stop"

$ERR = "$($PSStyle.Foreground.Red)$($PSStyle.Bold)[ERRR]$($PSStyle.Reset)"

if ($args.Count -gt 0 -and $args[0]?.StartsWith("+")) {
  $data = "+stable", "+nightly", "+current"
  if ($data -contains $args[0]) {
    $channel = $args[0].Substring(1)

    $path = "$env:LEAD_HOME/versions/$channel"
    if (!(Test-Path $path)) {
      Write-Error "$ERR The channel $channel is not yet installed. Use $($PSStyle.Foreground.Green)leadman install$($PSStyle.Reset)"
    }
    $ver = Get-Content $path
  }
  else {
    $ver = $args[0].Replace("+", "")
  }
}
else {
  $channel = "current"

  $path = "$env:LEAD_HOME/versions/current"
  if (!(Test-Path $path)) {
    Write-Error "$ERR The channel $channel is not yet installed. Use $($PSStyle.Foreground.Green)leadman install$($PSStyle.Reset)"
  }
  $ver = Get-Content $path
}

if ($ver.Length -eq 0) {
  Write-Error "$ERR No version has been marked as $($PSStyle.Foreground.Cyan)$channel$($PSStyle.Reset). Use $($PSStyle.Foreground.Green)lead [+stable / +nightly / +version] [args]$($PSStyle.Reset)"
  exit 1
}

$exec = "$env:LEAD_HOME/versions/$ver/lead.exe"

if (!(Test-Path -Path $exec)) {
  Write-Error "$ERR Your provided version $($PSStyle.Foreground.Cyan)$ver$($PSStyle.Reset) is invalid or not installed. Use $($PSStyle.Foreground.Green)lead [+stable / +nightly / +version] [args]$($PSStyle.Reset)"
}

if ($args.Count -gt 0 -and $args[0]?.StartsWith("+")) {
  $arglist = $args[1..$args.Length]
}
else {
  $arglist = $args[0..$args.Length]
}

Start-Process -NoNewWindow -Wait -FilePath $exec -ArgumentList $arglist