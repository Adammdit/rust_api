#!/bin/bash

# All the workers were added to /etc/hosts with these names, so we can use them directly
WORKERS=(
  bananapi-1
  bananapi-2
  bananapi-3
  bananapi-4
  bananapi-5
  bananapi-6
)

echo "=== Draining workers ==="
for NODE in "${WORKERS[@]}"; do
  echo "Draining $NODE..."
  k0s kubectl drain "$NODE" --ignore-daemonsets --delete-emptydir-data --force --timeout=20s
done

echo "=== Stopping worker services ==="
for NODE in "${WORKERS[@]}"; do
  echo "Stopping services on $NODE..."
  ssh "$NODE" "sudo systemctl stop k0sworker || true"
  ssh "$NODE" "sudo systemctl stop k0skubelet || true"
  ssh "$NODE" "sudo systemctl stop containerd || true"
done

echo "=== Powering off workers ==="
for NODE in "${WORKERS[@]}"; do
  echo "Powering off $NODE..."
  ssh "$NODE" "sudo systemctl poweroff"
done

echo "=== Stopping controller services ==="
sudo systemctl stop k0scontroller || true
sudo systemctl stop k0sapi || true
sudo systemctl stop containerd || true

echo "=== Powering off controller ==="
sudo systemctl poweroff