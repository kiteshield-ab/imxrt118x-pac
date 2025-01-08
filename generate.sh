#!/usr/bin/env bash

set -euxo pipefail

prepend_to_file() {
    file=$1
    text=$2

    if ! [[ -f $file ]] then
        touch $file
    fi

    echo "$text" | cat - $file > $file.new

    mv -f $file.new $file
}

chiptool generate --svd imxrt1189_cm33.svd.patched --transform transform.yaml
prepend_to_file lib.rs "#![allow(non_snake_case)]"
prepend_to_file lib.rs "#![allow(non_camel_case_types)]"
rm -rf src

form -i lib.rs -o src/
rm lib.rs
# mkdir -p src
# mv lib.rs src/lib.rs
 
cargo fmt
git diff --stat

