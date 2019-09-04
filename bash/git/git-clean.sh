#!/bin/bash
# Git. Remove merged branches.

git branch --merged | grep -v '*' | grep -v 'develop' | xargs git branch -D
git branch -vv | grep '\[origin/.*: gone\]' | grep -v '*' | grep -v 'develop' | awk '{print $1}' | xargs git branch -D

