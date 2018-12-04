#$Events = Get-EventLog security -message "*Тип входа:?10*" -after (get-date -hour 0 -minute 0 -second 0) | ?{$_.eventid -eq 4625 }
$Events = Get-EventLog security  -after ((get-date).addhours(-1)) | ?{$_.eventid -eq 4625 }

[string](Get-Date) + " - Запуск скрипта" >> "C:\fail2ban\log_fail2ban.log"


#$Events

$Data = New-Object System.Management.Automation.PSObject
$Data | Add-Member NoteProperty Time ($null)
$Data | Add-Member NoteProperty UserName ($null)
$Data | Add-Member NoteProperty ComputerName ($null)
$Data | Add-Member NoteProperty Address ($null)

$SpisokIP = $null

$Events | %{

$Data.time = $_.TimeGenerated

$message = $_.message.split("`n") | %{$_.trimstart()} | %{$_.trimend()}

$Data.UserName = ($message | ?{$_ -like "Имя учетной записи:*"} | %{$_ -replace "^.+:."} )
$Data.ComputerName = ($message | ?{$_ -like "Имя рабочей станции:*"} | %{$_ -replace "^.+:."}) 
$Data.Address = ($message | ?{$_ -like "Сетевой адрес источника:*"} | %{$_ -replace "^.+:."}) 


#$data

$SpisokIP += $Data.Address+" "

}

$SpisokIP = $SpisokIP.Split(" ").Trim()
#$SpisokIP

$SpisokIPCashe = @()
foreach ($ip in $SpisokIP) {
    if ($ip.Length -gt 6 -and $ip -notin $SpisokIPCashe) {
        $SpisokIPCashe += $ip
    }
}



foreach ($ip in $SpisokIPCashe) {
    if (($SpisokIP -eq $ip).Length -gt 3) {
        $ip + " - Забанен" >> "C:\fail2ban\log_fail2ban.log"
        netsh advfirewall firewall add rule name="Block_$ip" dir=in action=block enable=yes remoteip=$ip | Out-Null
    }
}
