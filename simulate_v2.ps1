Write-Host "Building..."
cargo build
Write-Host "Starting Server..."
$p = Start-Process cargo -ArgumentList "run -p iris-server" -PassThru -NoNewWindow
Start-Sleep -Seconds 5
Write-Host "Starting Client..."
cargo run -p iris-client
Stop-Process -Id $p.Id -Force
