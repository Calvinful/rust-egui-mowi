#!/bin/bash

touch "$1"
rm "$1"
touch "$1"
echo "----properties-----" >> "$1"
cat Cargo.toml >> "$1"
echo -e "\n----------project files-----------" >> "$1"
#cat src/main/java/com/reclament/resource_server/ResourceServerApplication.java >> "$1"
echo "" >> "$1"



TOPDIR=$(pwd)
find src/ -name "*.rs" -type f | while read file; do

    echo "$file" >> "$TOPDIR/$1"
    echo "--------------------------------">> "$TOPDIR/$1"
    cat "$file" >> "$TOPDIR/$1"
    echo "\n" >>  "$TOPDIR/$1"
    echo "--------------------------------" >> "$TOPDIR/$1"
done







echo "THE PROJECT IS FORMATTED LIKE SO:" >> "$1"
echo ".
├──src
│  ├──main.rs
│  └───menu
│      ├──mod.rs
│      ├──utils.rs
│      └──ui.rs
├──Cargo.toml
└──..." >> "$1"
echo "" >> "$1"

