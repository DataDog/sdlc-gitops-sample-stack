#!/bin/bash

set -e

go-licenses csv ./... | \
    awk -F',' 'BEGIN { OFS=","; print "Component,Origin,License,Copyright" } { print "pass-api", $1, $3, "" }' | \
    grep -v "apps/pass-api" > license-3rdparty.csv