#!/bin/bash

rust_version="1.72.0"

global_kybra_config_dir=~/.config/kybra
global_kybra_rust_dir="$global_kybra_config_dir"/rust/"$rust_version"
global_kybra_rust_bin_dir="$global_kybra_rust_dir"/bin
global_kybra_logs_dir="$global_kybra_rust_dir"/logs
global_kybra_cargo_bin="$global_kybra_rust_bin_dir"/cargo
global_kybra_rustup_bin="$global_kybra_rust_bin_dir"/rustup

export CARGO_TARGET_DIR="$global_kybra_config_dir"/rust/target
export CARGO_HOME="$global_kybra_rust_dir"
export RUSTUP_HOME="$global_kybra_rust_dir"

echo "
Preparing canister binaries for upload...
"

exec 3>&1; output=$(KYBRA_VERSION=0.5.2 $global_kybra_cargo_bin run --manifest-path=.kybra/kybra_hello_world/kybra_post_install/Cargo.toml kybra_hello_world src/main.did 2>&1 1>&3 3>&-); exit_code=$?; exec 3>&-; if [ $exit_code -ne 0 ]; then echo "$output"; exit $exit_code; fi
    