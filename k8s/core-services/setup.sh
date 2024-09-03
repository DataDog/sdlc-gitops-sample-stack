#!/bin/bash

helm repo add gitlab https://charts.gitlab.io/
helm repo update
helm upgrade --install gitlab gitlab/gitlab \
  --timeout 600s \
  --set global.hosts.domain=localdev \
  --set global.hosts.externalIP=192.168.64.100 \
  --set certmanager-issuer.email=admin@localdev