#!/usr/bin/env bash

set -euo pipefail

ARRAY_FEATURES=(
    "array-len-size-0"
    "array-len-size-1"
    "array-len-size-2"
    "array-len-size-4"
)

STRUCT_FEATURES=(
    "struct-len-size-0"
    "struct-len-size-1"
    "struct-len-size-2"
    "struct-len-size-4"
)

for array_feature in "${ARRAY_FEATURES[@]}"; do
    for struct_feature in "${STRUCT_FEATURES[@]}"; do
        echo
        echo "========================================"
        echo "Testing: $array_feature + $struct_feature"
        echo "========================================"

        cargo test \
            --no-default-features \
            --features "$array_feature,$struct_feature"
    done
done

echo "ALL TESTS PASSED :)"