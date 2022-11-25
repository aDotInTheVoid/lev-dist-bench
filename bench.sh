#!/bin/bash
set -eou pipefail

for i in $(seq 1 10)
do
	 cargo nextest run --release -j16 2> data/run_$i.txt
 done
