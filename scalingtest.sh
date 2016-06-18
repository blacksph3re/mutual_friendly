#/bin/bash

for i in 1 2 4 8
do
	echo "-----"
	echo "Number of threads = $i"
	time (cat input/large.in | target/release/mutual_friendly $i > output)
done
