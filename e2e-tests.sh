#!/bin/bash

# This script executes end-to-end tests.
#
# It is meant to be run in the CI on a daily basis and provide a report of the
# endpoint health status.
#
# It will in order:
# - index every .hurl file in the examples/ directory,
# - execute it and keep track of its success state,
# - output an inline summary in the console,
# - generate a summary.md that is reused in the CI job summary,
# - exit with an error code in case any hurl test fails.

# Initialize empty arrays to keep track of succeeding and failing files
succeeding_files=()
failing_files=()

# Define a set of excluded files that are not supported yet
excluded_files=(
  "examples/beerus-rpc/additional/starknet_l1_to_l2_message_cancellations.hurl"
  "examples/beerus-rpc/additional/starknet_l1_to_l2_message_nonce.hurl"
  "examples/beerus-rpc/additional/starknet_l1_to_l2_messages.hurl"
  "examples/beerus-rpc/additional/starknet_l2_to_l1_messages.hurl"
  "examples/beerus-rpc/starknet/starknet_getNonce.hurl"
  "examples/beerus-rpc/starknet/starknet_getStorageAt.hurl"
)

# Join the array elements with the '|' character, to create a pattern for grep
exclude_pattern=$(IFS='|'; echo "${exclude_files[*]}")

# Use find to get all .hurl files in the examples directory and its subdirectories
all_files=$(find examples -name "*.hurl" | grep -v -E -x -e "(${exclude})" | sort)

# Count all files, removing leading spaces
total_files=$(echo "$all_files" | wc -l | tr -d ' ')

# Initialize counter
count=0

# Loop over all .hurl files
while IFS= read -r file
do
    # Increment counter
    ((count++))

    # Display progress
    echo -e "\n[${count}/${total_files}] Executing: $file"

    # Execute the file with hurl and check the exit status
    if hurl --test --max-time=30 "$file"
    then
        # If the exit status is 0 (success), add file to succeeding_files
        succeeding_files+=("$file")
    else
        # If the exit status is non-zero (failure), add file to failing_files
        failing_files+=("$file")
    fi

done <<< "$all_files"

# Display failing files
if [ ${#failing_files[@]} -gt 0 ]
then
    echo "Failing files:"
    printf '%s\n' "${failing_files[@]}"
fi

# Display a one-line summary
echo "Summary: ${#failing_files[@]} failing files out of $total_files"

# Generate a markdown summary file
markdown_file="summary.md"

# Write the summary header
echo -e "## Hurl Test Execution Summary\n\
\n\
This is a summary of the different methods tested.\n\
\n\
### Failing Methods\n\
\n\
| Group | Method | Status |\n\
| --- | --- | --- |" > "$markdown_file"

# Add failing methods to the table
for method in "${failing_files[@]}"; do
    group=$(echo "$method" | awk -F'/' '{print $(NF-1)}')
    method_name=$(echo "$method" | awk -F'/' '{print $NF}' | sed 's/.hurl//')
    echo "| $group | $method_name | :x: |" >> "$markdown_file"
done

# Add the succeeding methods section
echo -e "\n\
### Succeeding Methods\n\
\n\
| Group | Method | Status |\n\
| --- | --- | --- |" >> "$markdown_file"

# Add succeeding methods to the table
for method in "${succeeding_files[@]}"; do
    group=$(echo "$method" | awk -F'/' '{print $(NF-1)}')
    method_name=$(echo "$method" | awk -F'/' '{print $NF}' | sed 's/.hurl//')
    echo "| $group | $method_name | :heavy_check_mark: |" >> "$markdown_file"
done

# If there are any failing files, return an error
if [ ${#failing_files[@]} -gt 0 ]
then
    exit 1
fi

exit 0
