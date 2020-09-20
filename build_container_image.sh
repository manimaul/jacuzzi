#!/usr/bin/env bash

#set -x # trace every command executed
set -u # fail on unset variables
set -e # fail on commands that return non-zero exit codes

version=$(cargo metadata --format-version 1 | jq -r '.packages[] | select(.name=="jacuzzi") | .version')
echo "building image: manimaul/jacuzzi:${version}"
docker build -t "manimaul/jacuzzi:${version}" .
docker tag "manimaul/jacuzzi:${version}" "manimaul/jacuzzi:latest"

publish() {
  echo "publishing version $version"
  docker push "manimaul/jacuzzi:latest"
  docker push "manimaul/jacuzzi:${version}"
}

while true; do
    read -p "Do you wish to publish to Docker Hub?" yn
    case $yn in
        [Yy]* ) publish; break;;
        [Nn]* ) exit;;
        * ) echo "Please answer yes or no.";;
    esac
done
