#!/bin/bash

set -e 

invoke generate-licenses
sed -i '' 's/^core,/pass-api,/g' LICENSE-3rdparty.csv 
echo "CSV generated at $OUTPUT_FILE"
