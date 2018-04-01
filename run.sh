#!/bin/sh

node_count=$1

mkdir -p ./config
mkdir -p ./db

./target/debug/timestamping_run generate-testnet -p 5000 $node_count -o ./config

for i in $(seq 0 $((node_count - 1)))
do
	port=$((8000 + i))
	private_port=$((port + node_count))
	./target/debug/timestamping_run run --node-config config/validators/$i.toml --db-path db/$i --public-api-address 0.0.0.0:${port} --private-api-address 0.0.0.0:${private_port} &
	echo "new node with ports: $port (public) and $private_port (private)"
done

echo "$node_count nodes configured and launched"

