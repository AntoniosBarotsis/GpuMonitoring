$ErrorActionPreference = "Stop"

Start-Job -ScriptBlock { ./server/target/release/server.exe } -Name GpuMonitoring | Out-Null

Start-Job -ScriptBlock { npx serve .\frontend } -Name NpxServe | Out-Null

Start-Process "http://localhost:3000/"

Write-Host "Started server. Press CTRL + C to exit."

# Change the default behavior of CTRL-C so that the script can intercept and use it versus just terminating the script.
[Console]::TreatControlCAsInput = $True
# Sleep for 1 second and then flush the key buffer so any previously pressed keys are discarded and the loop can monitor for the use of
#   CTRL-C. The sleep command ensures the buffer flushes correctly.
Start-Sleep -Seconds 1
$Host.UI.RawUI.FlushInputBuffer()
 
# Continue to loop while there are pending or currently executing jobs.
While ($true) {
  # If a key was pressed during the loop execution, check to see if it was CTRL-C (aka "3"), and if so exit the script after clearing
  #   out any running jobs and setting CTRL-C back to normal.
  If ($Host.UI.RawUI.KeyAvailable -and ($Key = $Host.UI.RawUI.ReadKey("AllowCtrlC,NoEcho,IncludeKeyUp"))) {
    If ([Int]$Key.Character -eq 3) {
      Write-Host "Shutting down server..."
      Stop-Job -Name GpuMonitoring
      Stop-Job -Name NpxServe
      break
    }
    # Flush the key buffer again for the next loop.
    $Host.UI.RawUI.FlushInputBuffer()
  }
}