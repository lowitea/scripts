#!/bin/bash

host="emitin@${1:6:-1}"

guake -n "ssh:${host}" -e "ssh ${host}"

