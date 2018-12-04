$computers = Get-ADComputer -filter * -Properties *
'"DNSHostName"'+','+'"Created"'+','+'"DistinguishedName"'+','+'"Enabled"'+','+'"IPv4Address"'+','+'"LastLogonDate"'+','+'"OperatingSystem"'+','+'"PasswordLastSet"'+','+'"Modified"' >> adcomputers.csv
foreach ($computer in $computers) {
    $computer
    '"'+$computer.DNSHostName+'","'+$computer.Created+'","'+$computer.DistinguishedName+'","'+$computer.Enabled+'","'+$computer.IPv4Address+'","'+$computer.LastLogonDate+'","'+$computer.OperatingSystem+'","'+$computer.PasswordLastSet+'","'+$computer.Modified+'"' >> adcomputers.csv
}