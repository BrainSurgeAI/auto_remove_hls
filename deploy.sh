#!/bin/bash

set -o errexit
set -o pipefail
set -o nounset
set +x

path=${1:-/home/send/encrypt_hls}
threshold=${2:-20}
host=${3:-127.0.0.1}
log_path=${4:/home/send/remove.log}

echo "59 23 * * * root video_cleaner $path $threshold $host $log_path" >> /etc/crontab