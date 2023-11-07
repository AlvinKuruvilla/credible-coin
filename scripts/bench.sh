#!/bin/bash

# Check if at least two arguments are given (# of arguments is -lt 2)
if [ "$#" -lt 2 ]; then
  echo "Usage: $0 <command> <iterations>"
  exit 1
fi

# Combine all arguments except the first into the command to be executed
COMMAND="${@:2}"
ITERATIONS=$1
TOTAL_TIME=0

echo "Benchmarking command: $COMMAND"
echo "Iterations: $ITERATIONS"

# Run the command the specified number of times
for ((i=1; i<=ITERATIONS; i++)); do
  # Run the command with time and use the TIMEFORMAT variable to get seconds directly
  # This avoids parsing issues across different systems
  TIME_TAKEN=$( { time -p $COMMAND; } 2>&1 | grep real | awk '{print $2}' )
  
  # Add to total time
  TOTAL_TIME=$(echo "$TOTAL_TIME + $TIME_TAKEN" | bc)
  
  echo "Iteration $i: $COMMAND took $TIME_TAKEN seconds"
done

# Calculate average time
AVERAGE_TIME=$(echo "scale=4; $TOTAL_TIME / $ITERATIONS" | bc)

echo "Average time per execution: $AVERAGE_TIME seconds"

exit 0
