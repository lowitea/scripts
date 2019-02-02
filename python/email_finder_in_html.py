import re

try:
    with open('data.html', encoding='utf-8') as data:
        data = data.read()
except FileNotFoundError as err:
    print('Ошибка:', err)
    print('Рядом со скриптом необходимо положить файл data.html!')
    input()
except Exception as err:
    print('Непредвиденная ошибка: ', err)
    input()

out_se = re.findall(r"[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+", data)
print(out_se)

with open('out.txt', mode='w', encoding='utf-8') as out:
    for email in out_se:
        out.write(email+'\n')
