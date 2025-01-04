#!/usr/bin/env fish

echo "Running Cargo check..."
# capture cargo check
set cargo_output (cargo check --message-format short --color=always 2>&1)
# immediately catch status
set cargo_status $status

# empty list to store printlines
set filtered_output

# parse output line by line
for line in $cargo_output
    # if contains "error" at all, "warning" in beginning for warning summary
    # or "Checking" which should be first line
    if echo $line | string match -q "*error*"
        set filtered_output $filtered_output $line
    else if echo $line | string sub -l 40 | string match -q "*Checking*"
        set filtered_output $filtered_output $line
    else if echo $line | string sub -l 20 | string match -q "*warning*"
        set filtered_output $filtered_output $line
    end
end

# echo "Filtered output: $filtered_output"
# Check if `cargo check` failed
if test $cargo_status -ne 0
    # print filtered lines
    for line in $filtered_output
        echo $line
    end
    exit 1
end

echo "Running cargo test..."

# If `cargo check` succeeded run `cargo test -- --nocapture` and suppress warnings
cargo test -- --nocapture --color=always --skip test_binstall 2>/dev/null

