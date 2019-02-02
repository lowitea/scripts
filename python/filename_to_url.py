from os import listdir, rename, getcwd
from os.path import isdir, join
from transliterate import translit


def url_escape(f):
    name = str(f.lower())
    name = translit(name, 'ru', reversed=True)
    name = name.replace(' ', '_')
    return name


def ren_file(url_name, f):
    if url_name == str(f):
        print(f, "skip")
    else:
        rename(f, url_name)
        print(f, '=>', url_name)
    return


def fil(files, root):
    for f in files:
        if isdir(join(root, f)):
            fil(listdir(join(root, f)), join(root, f))
            continue
        url_name = url_escape(f)
        ren_file(join(root, url_name), join(root, f))
    return


root = getcwd()
files = listdir()

fil(files, root)
