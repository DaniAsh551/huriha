#!/bin/bash

# this middleware serves static content in public directory

path=$(jq -r '.path' <<< "$@")

dir=$(dirname $path)

if [ -d ./public$dir ] && [ -f ./public$path ]; then
    res_body=$(jq -r '.res_body' <<< "$@")
    ln -sf $(realpath ./public$path) $res_body
    echo $res_body > resbody.txt

    mime=$(file -Li $res_body | awk '{print $2}')

    RET=$(cat <<- EOR
{
    "status": 200,
    "headers": {
        "Content-Type": "$mime"
    },
    "state": {},
    "terminate": true
}
EOR
)
    echo $RET
    exit 0
else
    echo $(cat <<- EOR
{
    "status": 200,
    "headers": {},
    "state": {},
    "terminate": false
}
EOR
)
fi