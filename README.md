# Kubernetes Cluster Setup (k0s on Raspberry Pi 5 + Banana Pi M1 Workers)

This document describes how to build a Kubernetes cluster using **k0s** on:

- **1× Raspberry Pi 5** — Controller  
- **6× Banana Pi M1** — Workers  
- **Rust API application** deployed on top of the cluster

This guide covers OS preparation, cgroup configuration, k0s installation, worker registration, and deployment of the Rust API application.

---

## 1. System Preparation (All Nodes)

### 1.1 Update & Upgrade


sudo apt update && sudo apt upgrade -y


---

## 2. Enable cgroups (Required by Kubernetes)

Edit the Armbian boot configuration:


sudo nano /boot/armbianEnv.txt


Add the following line:


extraargs=cgroup_enable=cpuset cgroup_enable=memory cgroup_memory=1


Reboot:


sudo reboot


---

## 3. Set Hostnames

### Controller:


sudo hostnamectl set-hostname rpi5-controller


### Workers:


sudo hostnamectl set-hostname bananapi-1 sudo hostnamectl set-hostname bananapi-2 sudo hostnamectl set-hostname bananapi-3 sudo hostnamectl set-hostname bananapi-4 sudo hostnamectl set-hostname bananapi-5 sudo hostnamectl set-hostname bananapi-6


Reboot:


sudo reboot


---

## 4. Install k0s (All Nodes)


sudo curl -sSLf https://get.k0s.sh | sudo sh


Verify installation:


k0s version


---

## 5. Configure the Controller Node

### 5.1 Install Controller Service


sudo k0s install controller --single sudo systemctl enable --now k0scontroller systemctl status k0scontroller


### 5.2 Generate Worker Join Token


k0s token create --role=worker > worker-token


Copy token to each worker:


scp worker-token root@192.168.0.130:/root/ scp worker-token root@192.168.0.131:/root/ scp worker-token root@192.168.0.132:/root/ scp worker-token root@192.168.0.133:/root/ scp worker-token root@192.168.0.134:/root/ scp worker-token root@192.168.0.135:/root/


---

## 6. Configure Worker Nodes

### 6.1 Install Worker Service


sudo k0s install worker --token-file /root/worker-token sudo systemctl enable --now k0sworker systemctl status k0sworker


### 6.2 If worker fails to join


sudo systemctl restart k0sworker


Repeat until the controller reports the node as **Ready**.

---

## 7. Verify Cluster Status (Controller)


k0s kubectl get nodes


Expected output:


bananapi-1       Ready bananapi-2       Ready bananapi-3       Ready bananapi-4       Ready bananapi-5       Ready bananapi-6       Ready rpi5-controller  Ready


---

## 8. Deploy the Rust API Application

### 8.1 Build Multi‑Arch Docker Image


docker buildx build --platform linux/arm/v7,linux/arm64 -t <registry>/rust-api:latest --push .


### 8.2 Deployment Manifest


apiVersion: apps/v1 kind: Deployment metadata: name: rust-api spec: replicas: 3 selector: matchLabels: app: rust-api template: metadata: labels: app: rust-api spec: containers: - name: rust-api image: <registry>/rust-api:latest ports: - containerPort: 8080


Apply:


k0s kubectl apply -f deployment.yaml


### 8.3 Service Manifest


k0s kubectl apply -f service.yaml


### 8.4 Validate Application


curl http://127.0.0.1:8080/health


Expected:


OK


---

## 9. Node IP Reference Table

IP Address

Hostname

192.168.0.120

rpi5-controller

192.168.0.130

bananapi-1

192.168.0.131

bananapi-2

192.168.0.132

bananapi-3

192.168.0.133

bananapi-4

192.168.0.134

bananapi-5

192.168.0.135

bananapi-6

10. Cluster Health Checks

Node status:

k0s kubectl get nodes

Pod status:

k0s kubectl get pods -A

API health:

curl [http://127.0.0.1:8080/health](http://127.0.0.1:8080/health)
