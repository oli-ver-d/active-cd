new_directory=$(cargo run)

if [[ "$new_directory" != "x" ]]; then
    # Change to the new directory
    cd "$new_directory"
fi
