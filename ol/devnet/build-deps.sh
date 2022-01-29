pushd ../..
cargo build -p diem-node -p cli
pushd language/move-stdlib
cargo run --release
popd
pushd language/diem-framework/modules
cargo run --release
popd
pushd language/diem-framework
cargo run --release
popd
popd
