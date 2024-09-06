#!/bin/bash

# Usage: ./prepare_argo_release.sh NEW_VERSION ENV BRANCH_NAME
# Example: ./prepare_argo_release.sh 123 dev deploy

set -e

# Check if the correct number of arguments is provided
if [ "$#" -ne 3 ]; then
  echo "Usage: $0 NEW_VERSION ENV BRANCH_NAME"
  exit 1
fi

NEW_VERSION="$1"
ENV="$2"
BRANCH_NAME="$3"

# Define the script path for updating the manifests
SCRIPTS_DIR=${BASH_SOURCE%/*}
UPDATE_SCRIPT="${SCRIPTS_DIR}/update_manifests_for_release.sh"

# Only switch branches and merge if the branch is not 'main'
echo "Looks like we're not merging to main but rather '$BRANCH_NAME'; checking out target branch" 
if [ "$BRANCH_NAME" != "main" ]; then
  
  # Check if the branch exists
  echo Checking if "$BRANCH_NAME" exists 
  if ! git rev-parse --verify "$BRANCH_NAME" >/dev/null 2>&1; then
    # Create the branch if it doesn't exist
    echo "Branch '$BRANCH_NAME' does not exist. Creating it from 'main'."
    git checkout -b "$BRANCH_NAME" main
  fi

  # Checkout the specified branch and merge main into it
  echo checkout "$BRANCH_NAME"
  git checkout "$BRANCH_NAME"
  echo git merge main --no-edit
  git merge main --no-edit
fi

# Run the update script
echo Running update script
echo $UPDATE_SCRIPT "$NEW_VERSION" "$ENV"
$UPDATE_SCRIPT "$NEW_VERSION" "$ENV"

# Commit and push changes
git add "manifests/overlays/${ENV}/kustomization.yaml"
git commit -m "Updated environment: ${ENV} with version ${NEW_VERSION}"
echo git push origin "HEAD:$BRANCH_NAME" --force
git push origin "HEAD:$BRANCH_NAME" --force

echo "Pushed changes to branch: ${BRANCH_NAME}"
