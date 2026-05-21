#!/bin/bash

# Uncordon all nodes that are Ready but SchedulingDisabled
nodes=$(k0s kubectl get nodes --no-headers | awk '/Ready,SchedulingDisabled/ {print $1}')

if [ -z "$nodes" ]; then
  echo "No nodes need uncordoning"
  exit 0
fi

for node in $nodes; do
  echo "Uncordoning node: $node"
  k0s kubectl uncordon "$node"
done