$users = Get-ADUser -filter *

foreach ($user in $users) {
    if ($user.SamAccountName -in ('admin')) {
        continue
    }
    Set-ADUser $user.SamAccountName -PasswordNeverExpires $False -ChangePasswordAtLogon $True
    $user
}