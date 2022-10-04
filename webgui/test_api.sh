#!/bin/zsh

curl --header "Content-Type: application/json" \
  --request POST \
  --data '{"name":"Sean","rate":2}' \
  http://localhost:3030/render