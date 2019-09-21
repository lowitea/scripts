#!/bin/bash
# clean Downloads directory
# delete all files older 15 days
# for crontab:
# 0 */4 * * * ~/scripts/bash/downloads_cleaner.sh

find ~/Downloads -mtime +15 -delete

