#!/bin/bash
FILES=(
  ".github/workflows/build.yml"
  "app/src/main/AndroidManifest.xml"
  "app/src/main/res/values/styles.xml"
  "Cargo.toml"
  "app/build.gradle"
  "src/lib.rs"
)

for f in "${FILES[@]}"; do
  if [ -f "$f" ]; then
    echo "========================================"
    echo " FILE: $f"
    echo "========================================"
    cat "$f"
    echo -e "\n"
  else
    echo -e "!!! File $f TIDAK DITEMUKAN !!!\n"
  fi
done
