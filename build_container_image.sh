#!/usr/bin/env bash

version=$(cargo metadata --format-version 1 | jq -r '.packages[] | select(.name=="jacuzzi") | .version')
echo "manimaul/jacuzzi:${version}"
docker build -t "manimaul/jacuzzi:${version}" .
docker tag "manimaul/jacuzzi:${version}" "manimaul/jacuzzi:latest"
docker push "manimaul/jacuzzi:latest"
docker push "manimaul/jacuzzi:${version}"
