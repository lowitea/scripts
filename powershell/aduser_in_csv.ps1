$users = Get-ADUser -filter * -Properties LastLogonDate, Enabled
'"SamAccountName"'+','+'"GivenName"'+','+'"Surname"'+','+'"Name"'+','+'"DistinguishedName"'+','+'"LastLogonDate"'+','+'"Enabled"' > adusers.csv
foreach ($user in $users) {
    $user
    '"'+$user.SamAccountName+'","'+$user.GivenName+'","'+$user.Surname+'","'+$user.Name+'","'+$user.DistinguishedName+'","'+$user.LastLogonDate+'","'+$user.Enabled+'"' >> adusers.csv
}