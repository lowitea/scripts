#!/bin/bash
# Git. Remove merget branches.

git branch --merged | grep -v '*' | grep -v 'develop' | xargs git branch -D  && git reset --hard && git clean -d -x -f
