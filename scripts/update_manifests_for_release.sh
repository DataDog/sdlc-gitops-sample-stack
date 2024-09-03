#!/bin/bash

# Usage: ./update_manifests_for_release.sh NEW_VERSION ENV
# Example: ./update_manifests_for_release.sh 123 dev

set -e

# Check if the correct number of arguments is provided
if [ "$#" -ne 2 ]; then
  echo "Usage: $0 NEW_VERSION ENV"
  exit 1
fi

NEW_VERSION="$1"
ENV="$2"

# Define paths
BASE_DIR="manifests/overlays/${ENV}"

# Change to the directory for the specified environment
if [ -d "$BASE_DIR" ]; then
  cd "$BASE_DIR"
else
  echo "Directory $BASE_DIR does not exist."
  exit 1
fi

# Update DD_VERSION and newTag using sed
sed -i -e "s/DD_VERSION=[^ ]*/DD_VERSION=${NEW_VERSION}/" kustomization.yaml
sed -i -e "s/newTag: [^ ]*/newTag: \"${NEW_VERSION}\"/" kustomization.yaml

echo "Updated kustomization.yaml for environment: ${ENV} with version ${NEW_VERSION}"
