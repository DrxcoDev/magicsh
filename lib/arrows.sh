# Define the array with arrows
arrows=(
    "↪"
    "→"
)

# Common arrow is ${arrows[0]}
arrow() {
    local common_arrow=${arrows[0]}
    if [ -n "$common_arrow" ]; then
        echo "$common_arrow"
    fi
}

# Call the function arrow
arrow


