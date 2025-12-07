#!/bin/bash

# Set the range of days you want to run
for i in {1..12}
do
  # Format the day number with a leading zero (e.g., 01, 02)
  day=$(printf "day%02d" $i)
  
  # Print a header for clarity
  echo "======================================="
  echo "Running $day"
  echo "======================================="
  
  # Run the cargo command for the specific binary
  # Use --release for an optimized build
  cargo run --bin "$day"
  
  # Add a newline for spacing
  echo ""
done

echo "All days have been run."
