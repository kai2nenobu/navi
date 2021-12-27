function _navi_widget {
  $in = $null
  [Microsoft.PowerShell.PSConsoleReadLine]::GetBufferState([ref]$in, [ref]$null)
  if ([string]::IsNullOrEmpty($in)) {
    $last_command = ""
  } else {
    $last_command = ("$in" | navi fn widget::last_command) -join ""
  }
  if ([string]::IsNullOrEmpty($last_command)) {
    $output = (navi --print)
  } else {
    $find = "${last_command}_NAVIEND"
    $replacement = (navi --print --query "$last_command")
    $output = $in
    if (-not [string]::IsNullOrEmpty($replacement)) {
      $output = "${in}_NAVIEND"
      $output = $output.Replace($find, $replacement)
    }
  }
  [Microsoft.PowerShell.PSConsoleReadLine]::InvokePrompt() # Rewrite Prompt
  if (-not [string]::IsNullOrEmpty($output)) {
    [Microsoft.PowerShell.PSConsoleReadLine]::Delete(0, $in.Length)
    [Microsoft.PowerShell.PSConsoleReadLine]::Insert($output)
  }
}

Set-PSReadLineKeyHandler -Chord 'Ctrl+g' -ScriptBlock { _navi_widget }
