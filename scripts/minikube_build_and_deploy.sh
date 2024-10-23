#!/bin/bash

#
# Build and deploy our containers into minikube running 
# on our own machine
# 

set -e

# Switch into minikube's docker-env
eval $(minikube docker-env)

#
# Build our images
#

pushd apps/pass-api
docker build . -t pass-api:latest
popd

pushd apps/pass-summary-api
docker build . -f src/main/docker/Dockerfile.jvm -t pass-summary-api:latest
popd

pushd apps/pass-image-api
docker buildx build . -t pass-image-api:latest
popd

# 
# Apply our manifests and force a restart of our pods
#
kubectl apply -k manifests/overlays/local
kubectl rollout restart deployment pass-image-api
kubectl rollout restart deployment pass-api
kubectl rollout restart deployment pass-summary-api
