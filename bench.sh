#!/bin/bash
set -eoxu pipefail

for i in $(seq 1 100)
do
	cargo nextest run --release -j1 2> data/run_$i.txt
done
