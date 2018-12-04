#!/bin/bash
# test response time
# run: ./resp_time.sh [url] [count response]
# example: ./resp_time.sh google.com 5

if [[ -z $1 ]]; then
    echo "url not found"
    exit
fi

if [[ -z $2 ]]; then
    resp_count=20
else
    resp_count=$2
fi

while [  $resp_count -ne 0 ]; do
    curl -Is -w "%{http_code} Time: %{time_total}\n" $1 | grep 'Time' | tr , .
    let resp_count=resp_count-1 
done | awk '{print "\033[0;31mCode:\033[0m", $1, "\033[0;31mTime:\033[0m", $3} {sum+=$3} END {print "Average = ",sum/NR}'
