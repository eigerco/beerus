#!/bin/bash

# Initialize empty arrays to keep track of succeeding and failing files
succeeding_files=()
failing_files=()

# Use find to get all .hurl files in examples/ directory and its subdirectories
all_files=$(find examples/ -name "*.hurl" | head -n 10)

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

    # Execute the file with hurl and check the exit status, suppressing output
    if hurl --test --max-time=10 "$file"
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

# Display the one-line summary
echo "Summary: ${#failing_files[@]} failing files out of $total_files"

# If there are any failing files, return an error
if [ ${#failing_files[@]} -gt 0 ]
then
    exit 1
fi

exit 0
