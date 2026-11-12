#!/bin/sh
# Regenerate the PAC from svd/PIC32CK2051SG01144.svd.
#
# Needs svd2rust 0.37 and form on PATH (cargo install svd2rust@0.37.1 form),
# and python3.
# form writes `foo/mod.rs` files; they are renamed to `foo.rs`, the same
# module tree in the newer layout.
set -eu
cd "$(dirname "$0")"
svd2rust --version | grep -q ' 0\.37\.' || { echo "need svd2rust 0.37" >&2; exit 1; }

gen=$(mktemp -d)
trap 'rm -rf "$gen"' EXIT
# Microchip's SVD needs two fixes first (see fix_svd.py).
python3 -I fix_svd.py svd/PIC32CK2051SG01144.svd "$gen/fixed.svd"
svd2rust --target cortex-m -i "$gen/fixed.svd" -o "$gen"
rm -rf src
form -i "$gen/lib.rs" -o src
mv "$gen/build.rs" "$gen/device.x" .

# foo/mod.rs -> foo.rs, deepest first.
find src -name mod.rs | awk '{ print length, $0 }' | sort -rn | cut -d' ' -f2- |
while read -r f; do
    mv "$f" "$(dirname "$f").rs"
done
find src -type d -empty -delete
cargo fmt -p pic32ck-sg01-pac
