# Local interactive test build. Web SDP is manual; native signaling is unchanged.
param([int]$Port = 39091)
$repo = (Resolve-Path (Join-Path $PSScriptRoot '..\..')).Path
$exe = Join-Path $repo 'target\web-share-e2e\kiyora-web-test.exe'
if (-not (Test-Path -LiteralPath $exe)) { throw 'Build de teste ausente: target/web-share-e2e/kiyora-web-test.exe' }
$oldManual = $env:kiyora_WEB_MANUAL
$oldPort = $env:kiyora_AUTOMATION_PORT
try {
    $env:kiyora_WEB_MANUAL = '1'
    $env:kiyora_AUTOMATION_PORT = "$Port"
    Push-Location $repo
    Write-Output 'Inicie a transmissão e use Compartilhar na web > Copiar link.'
    & $exe
} finally {
    Pop-Location
    $env:kiyora_WEB_MANUAL = $oldManual
    $env:kiyora_AUTOMATION_PORT = $oldPort
}
