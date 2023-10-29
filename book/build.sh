#!/bin/bash

# Store the current branch or commit
original_branch=$(git rev-parse --abbrev-ref HEAD)
if [ "$original_branch" == "HEAD" ]; then
  original_branch=$(git rev-parse HEAD)
fi

mkdir -p versions  # Create a root directory for all versions

# Declare an associative array to map commits to custom version directory names
declare -A commit_to_version=( ["master"]="salsa2022" ["754eea8b5f8a31b1100ba313d59e41260b494225"]="salsa" )

# Loop over the keys (commit hashes) in the associative array
for commit in "${!commit_to_version[@]}"; do
  git checkout $commit
  mdbook build
  version_dir="versions/${commit_to_version[$commit]}"
  mv book/html $version_dir
  rm -rf book
done

# Move _redirects to the root directory
mv _redirects versions

# Return to the original branch or commit
git checkout $original_branch
