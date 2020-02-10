#!/bin/bash
# Git. Remove merged branches.

LANG=C git branch --merged | egrep -v '\*|develop|master' | xargs -r git branch -D
LANG=C git branch -vv | grep '\[origin/.*: gone\]' | grep -v '*' | awk '{print $1}' | xargs -r git branch -D

