#!/bin/bash
# clean Downloads directory
# delete all files older 7 days
# for crontab:
# 0 */4 * * * ~/scripts/bash/downloads_cleaner.sh

find ~/Downloads -mtime +7 -delete

