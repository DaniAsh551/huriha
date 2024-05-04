#!/bin/bash

method=$(jq -r '.method' <<< "$@")
path=$(jq -r '.path' <<< "$@")
res_body=$(jq -r '.res_body' <<< "$@")
from="$(jq -r '.state.from' <<< "$@") and bash"

RET=$(cat <<- EOR
{
    "status": 200,
    "headers": {
        "Content-Type": "text/html; charset=utf-8",
        "bash-middleware": "yes"
    },
    "state": {},
    "terminate": true
}
EOR
)

RESP=$(cat <<- EOR
<html>
<head>
    <title>Response from $from</title>
</head>
<body>
    <p>This is a $method request to $path</p>
    <p>Hello from $from</p>
</body>
</html>
EOR
)

function next {
    echo $RET
}

function render {
    echo $RESP > $res_body
}

render
next