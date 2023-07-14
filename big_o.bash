#!/bin/bash

# Function to measure execution time of a command
measure_execution_time() {
  start_time=$(date +"%s")
  "$@" >/dev/null 2>&1
  end_time=$(date +"%s")
  execution_time=$(echo "$end_time - $start_time" | bc)
  echo $execution_time
}

# Create a CSV file and write the header
echo "Argument,Execution Time" > output.csv

# Loop through every multiple of 100 between 1700 and 3400
for ((number = 100; number <= 3400; number += 100)); do
  # Run the binary with the current number and measure execution time
  execution_time=$(measure_execution_time ./target/release/solve $number)

  # Append the argument and execution time to the CSV file
  echo "$number,$execution_time" >> output.csv
done

echo "CSV file created: output.csv"
