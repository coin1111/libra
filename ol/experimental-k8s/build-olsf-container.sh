#!/bin/bash
set -x
set -e
olsf_container=libra-olsf

if [ "$1" == "-h" ]; then
	echo "Usage: build-olsf-container.sh <use_local_bits> [-p <project>]"
	echo -e "\t Build container with olsf binaries"
	echo -e "\t -p -publish to dockerhub to <repo>, e.g coin1111/olsf-libra"
	exit 0
fi
use_local_bits="$1"

if [ "$use_local_bits" == 1 ]; then
  pushd ../..
  pwd
  dockerfile=Dockerfile.local
  docker build -t "$olsf_container" -f ol/experimental-k8s/"$dockerfile" .
  popd
else
  dockerfile=Dockerfile.olsf
  docker build -t "$olsf_container" -f "$dockerfile" .
fi
ids=$(docker images | grep "$olsf_container" | grep latest | grep -v "gcr" | awk '{print $3}')

for id in "$ids"
do
	break
done

if [ "$2" == "-p" ]; then
    if [ "$3" == "" ]; then
	    repo=coin1111/olsf-libra
    else
	    repo="$3"
    fi
    tag="$repo:$id"
    docker tag "$id" "$tag"
	docker push "$tag"
fi

