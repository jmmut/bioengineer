#!/bin/bash

# This script just automates changing the Cargo.toml version.

# unofficial bash safe mode
set -euo pipefail

if [[ $# -ne 1 || ! ( $1 =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ) ]]
then
  echo "USAGE: $0 MAJOR.MINOR.PATH"
  echo "    where MAJOR.MINOR.PATH is the new version you are releasing."
  echo "    You will be asked to write release notes."
  exit 1
fi

NEW_VERSION=$1
echo tagging version "$NEW_VERSION"

if [[ $(git status --porcelain 2> /dev/null | grep -v "??" -c)  != "0" ]]
then
  echo -e "Your workspace is dirty, you pig! refusing to tag a version\n------\n"
  git status
  exit 1
fi

sed -i "s/version = \"[0-9]\+.[0-9]\+.[0-9]\+\"/version = \"$NEW_VERSION\"/" Cargo.toml

cargo build
git add Cargo.toml Cargo.lock
git commit -m "update Cargo version to $NEW_VERSION"
git tag -a "$NEW_VERSION"

echo -e "\n------\nnow you can push with:\ngit push origin $NEW_VERSION"




