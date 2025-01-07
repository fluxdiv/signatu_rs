#!/usr/bin/env fish

echo "Cleaning build dirs..."
cargo clean --color=always
echo "Build dirs cleaned"

echo "Testing build & installing latest binary..."
cargo test test_binstall 2>/dev/null
set binstall_status $status

if test $binstall_status -ne 0
    echo "Installing latest binary failed, exiting"
    exit 1
end
echo "Latest binary installed"

./scripts/test_no_warnings.fish

