#!/bin/bash

language=$1
num_runs=$2
command_to_run="/home/furnic/Dev/rust_projects/hunter/target/debug/hunter -l $language -s ./src/utils.rs -v mutate"
previous_output=""

for ((i=1; i<=num_runs; i++))
do
    # Run the command and extract the line with "Mutation score:"
    current_output=$($command_to_run | grep "Mutation score:")

    if [ $i -gt 1 ] && [ "$current_output" != "$previous_output" ]
    then
        echo "false"
        exit 0
    fi
    previous_output=$current_output
done
echo "true"