Set-Location c:\

$units = Get-ADOrganizationalUnit -Filter *
$domain = Get-ADDomain
$domain = $domain.Name

foreach ($unit in $units) {
    '-------------------------------------------------------------------------------------------- `n ' >> c:\passss
    $pass = 'xQYBhu'+[string]$(Get-Random -Maximum 9999 -Minimum 1000)
    $unit.Name >> c:\passss
    $pass >> c:\passss
    $pass = ConvertTo-SecureString -AsPlainText -String $pass -force

    $unit_name = $unit.Name
    $users = Get-ADUser -SearchBase "OU=$unit_name,DC=$domain,DC=loc" -filter *

    foreach ($user in $users) {
        if ($user.SamAccountName -in ('admin')) {
            continue
        }
        Set-ADAccountPassword $user.SamAccountName -NewPassword $pass
        Set-ADUser $user.SamAccountName -PasswordNeverExpires $False -ChangePasswordAtLogon $True
        $user.SamAccountName >> c:\passss
    }
}