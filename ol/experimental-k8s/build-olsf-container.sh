#!/bin/bash
#set -x
set -e
olsf_container=libra-olsf

if [ "$1" == "-h" ]; then
	echo "Usage: build-olsf-container.sh [-p <project>]"
	echo -e "\t Build container with olsf binaries"
	echo -e "\t -p -publish to dockerhub to <repo>, e.g coin1111/olsf-libra"
fi

docker build -t "$olsf_container" -f Dockerfile.olsf .
ids=$(docker images | grep "$olsf_container" | grep latest | grep -v "gcr" | awk '{print $3}')

for id in "$ids"
do
	break
done

if [ "$1" == "-p" ]; then

    if [ "$2" == "" ]; then
	    repo=coin1111/olsf-libra
    else
	    repo="$2"
    fi
    tag="$repo:$id"
    docker tag "$id" "$tag"
	docker push "$tag"
fi

