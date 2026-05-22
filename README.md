# Kubernetes Cluster Setup (k0s on Raspberry Pi 5 + Banana Pi M1 Workers)

This document describes how to build a Kubernetes cluster using **k0s** on:

- **1 × Raspberry Pi 5** - Controller (64bit)
  - CPU: Broadcom BCM2712 2.4GHz quad-core 64-bit Arm Cortex-A76 CPU, with Cryptographic Extension, 512KB per-core L2 caches, and a 2MB shared L3 cache
  - CPU Arch: ARM64 (aarch64)
  - RAM: 8GB LPDDR4X-4267 SDRAM
  - OS: Debian GNU/Linux 13 (trixie)
- **6 × Banana Pi BPI-M1** - Workers (32bit)
  - CPU: Dual-core A20 ARM 1.0GHzCortex-A7
  - CPU Arch: ARMv7 (armv7l)
  - RAM: 1GB DDR3 SDRAM
  - OS: Armbian 26.2.1 trixie
- **Rust API application** deployed on top of the cluster

This guide covers OS preparation, cgroup configuration, k0s installation, worker registration, and deployment of the Rust API application.

---

## 1. System Preparation (All Nodes)

**Update & Upgrade**

- **Controller**
  - Run: `sudo apt-get update`
- **Workers**
  - Run: `sudo apt-get update && sudo armbian-upgrade`
  - Repeat above steps on all workers

---

## 2. Enable cgroups (Required by Kubernetes)

**Edit boot configuration:**

- **Controller**
  - Run: `sudo nano /boot/firmware/cmdline.txt`
  - Add line: `cgroup_enable=cpuset cgroup_enable=memory cgroup_memory=1`
  - Run: `sudo reboot`
- **Workers**
  - Run: `sudo nano /boot/armbianEnv.txt`
  - Add line: `extraargs=cgroup_enable=cpuset cgroup_enable=memory cgroup_memory=1`
  - Run: `sudo reboot`
  - Repeat above steps on all workers

---

## 3. Set Hostnames

**Set hostnames for the cluster:**

- **Controller**
  - Run: `sudo hostnamectl set-hostname controller`
- **Workers**
  - Run: `sudo hostnamectl set-hostname bananapi-1`
  - Run: `sudo hostnamectl set-hostname bananapi-2`
  - ...
  - Run: `sudo hostnamectl set-hostname bananapi-6`
  - Repeat above steps on all workers

---

## 4. Install k0s (All Nodes)

- Run: `sudo curl -sSLf https://get.k0s.sh | sudo sh`
- Verify installation: `k0s version`

---

## 5. Configure the Controller Node

### 5.1 Install Controller Service

- Run to install: `sudo k0s install controller`
- Run to enable: `sudo systemctl enable --now k0scontroller`
- Run to verify: `systemctl status k0scontroller`

### 5.2 Generate Worker Join Token

- Run: `sudo k0s token create --role=worker > /root/worker-token`
- Copy token to bananapi-1: `sudo scp worker-token root@192.168.0.130:/root/`
- Copy token to bananapi-2: `sudo scp worker-token root@192.168.0.131:/root/`
- ...
- Copy token to bananapi-6: `sudo scp worker-token root@192.168.0.135:/root/`
- Repeat for all workers

---

## 6. Configure Worker Nodes

### 6.1 Install Worker Service

- Run to install: `sudo k0s install worker --token-file /root/worker-token`
- Run to enable: `sudo systemctl enable --now k0sworker`
- Run to verify: `systemctl status k0sworker`

### 6.2 If worker fails to join

- Rum: `sudo systemctl restart k0sworker`
- Repeat until the controller reports the node as **Ready**.

---

## 7. Verify Cluster Status (Controller)

- Run: `sudo k0s kubectl get nodes`
- Expected output: You should see all the workers listed READY

---

## 8. Deploy the Rust API Application

### 8.1 Build Multi‑Arch Docker Image

- Run: `docker login`
- Run: `docker buildx build --platform linux/arm/v7 -t adammdit/rust-api:latest --push .`

### 8.2 Deployment & Service Manifest

- Apply the manifest from the repository:
  - `sudo k0s kubectl apply -f k8s/deployment.yaml`

### 8.3 Validate Application

- Run: `curl http://192.168.0.133:30979/health`
- Expected: OK
- Works with any worker IP.

---

## 9. Cluster Health Checks

- Nods status: `sudo k0s kubectl get nodes`
- Pods status: `sudo k0s kubectl get pods -A`
- Pods & nodes: `sudo k0s kubectl get pods -o wide`
- Svc status: `sudo k0s kubectl get svc`
- Noded perf: `sudo k0s kubectl top nodes`
- Pods perf: `sudo k0s kubectl top pods -A`
- Verify deployment: `sudo k0s kubectl rollout status deployment rust-api`
- Check the running image: `sudo k0s kubectl get pod -l app=rust-api -o jsonpath='{.items[0].spec.containers[0].image}'`
- Get into a pod: `sudo k0s kubectl exec -it <your-pod-name> -- sh`
- Describe a pot: `sudo k0s kubectl describe pod rust-api-7cdf44d9f-6shh5`

---

## 10. Rust API Application

This repository also contains a small Actix-web API with embedded persistent storage using `sled`.

### Data persistence

- Data is stored under `data/db` using an embedded key/value database.
- Items survive restarts and are serialized as JSON by UUID key.

### API endpoints

- `GET /health` — health check
- `GET /items` — list all items
- `GET /items?completed=true` — list completed items
- `POST /items` — create a new item
- `GET /items/{id}` — get item by UUID
- `PATCH /items/{id}` — update item fields
- `DELETE /items/{id}` — delete item by UUID

### Example requests

Create item:

```bash
curl -X POST http://192.168.0.133:30979/items \
  -H "Content-Type: application/json" \
  -d '{"name":"Write README","description":"Add persistent storage docs"}'
```

Update item:

```bash
curl -X PATCH http://192.168.0.133:30979/items/<uuid> \
  -H "Content-Type: application/json" \
  -d '{"completed":true}'
```

List items:

```bash
curl http://http://192.168.0.133:30979/items
```

List completed items:

```bash
curl http://192.168.0.133:30979/items?completed=true
```

### Run locally

```bash
cargo run
```

The API listens on `0.0.0.0:8080` by default.

If you want the local API only, run `cargo run` and use `http://localhost:8080`.