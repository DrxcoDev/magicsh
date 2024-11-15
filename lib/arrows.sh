# Define the array with arrows
arrows=(
    "↪"
    "→"
)

# Common arrow is ${arrows[0]}
arrow() {
    local common_arrow=${arrows[0]}
    if [ -n "$common_arrow" ]; then
        # Use ANSI escape code for yellow color
        echo -e "Variable name: common_arrow, Value: \033[33m$common_arrow\033[0m"
    fi
}

# Call the function arrow
arrow


