#!/bin/sh

CUR=$(pwd)

CURRENT=$(cd "$(dirname "$0")" || exit;pwd)
echo "${CURRENT}"

if ! (git pull --prune); then
  cd "${CUR}" || exit
  exit 1
fi

if ! (disable-checkout-persist-credentials); then
  cd "${CUR}" || exit
  exit 1
fi

set -- "actix-example"  "axum-example"  "rocket-example"
for target in "$@"; do
  if ! (cd "${CURRENT}/${target}" || exit && cargo update && cargo build); then
    cd "${CUR}" || exit
    exit 1
  fi
  echo ""
  pwd
done

if ! (cd "${CURRENT}" || exit && git add . && git commit -am "Bumps crates" && git push); then
  cd "${CUR}" || exit
  exit 1
fi

cd "${CUR}" || exit
