#!/bin/bash
# Firefox. Auto update dev version

DOWNLOAD_URL='https://download.mozilla.org/?product=firefox-devedition-latest-ssl&os=linux64&lang=en-US'
TEMP_DIR='/tmp/fxdev'
TEMP_TAR='/tmp/fxdev.tar.bz2'
FIREFOX_DIR='/usr/lib64/firefox-developer-edition'
BACKUP_DIR='/bkp/firefox_dev'
BACKUP_FILE=${BACKUP_DIR}'/fxdev.tar.bz2'

echo ' -- Stop firefox' && \
killall firefox-bin && sleep 5 && \
echo ' -- Stoping complete\n\n'

echo ' -- Download file' && \
wget $DOWNLOAD_URL -O $TEMP_TAR -q --show-progress && \
echo ' -- Download complete\n\n'

echo ' -- Unpacking file' && \
mkdir $TEMP_DIR && tar -vxf $TEMP_TAR -C $TEMP_DIR --totals && \
echo ' -- Unpacking complete\n\n'

echo ' -- Create new backup file' && \
mkdir -p $BACKUP_DIR && \
tar -cvzf $BACKUP_FILE $FIREFOX_DIR && \
echo ' -- Backuping complete\n\n'

echo ' -- Copy firefox files in '$FIREFOX_DIR'/' && \
cp -rv $TEMP_DIR'/firefox/.' $FIREFOX_DIR'/'
echo ' -- Copy complete\n\n'

echo ' -- Removing temp files' && \
rm -rf $TEMP_DIR && \
rm -rf $TEMP_TAR && \
echo ' -- Removing complete'

exit
