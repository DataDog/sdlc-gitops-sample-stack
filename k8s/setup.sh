#!/bin/bash

# Configuration variables
export KUBECONFIG=`pwd`/kubeconfig.yaml
MASTER_IP="192.168.64.100"
TMP_DIR="/tmp/k3s-configs"

# Create a temporary directory for cloud-init files
mkdir -p $TMP_DIR

# Step 1: Launch the master node
echo "Launching k3s master node..."
multipass launch --name k3s-master --cloud-init cloud-init-master.yaml  --network name=en0,mode=manual,mac="52:54:00:4b:ab:cd"  --disk 20G --memory 3G --cpus 2 
echo "Bootstrapping ..."
multipass exec k3s-master -- sudo /tmp/bootstrap.sh

# Step 2: Retrieve the join command
echo "Retrieving join command from master node..."
JOIN_COMMAND=$(multipass exec k3s-master -- cat /etc/rancher/k3s/join-command.txt)
echo "Join command: $JOIN_COMMAND"

# Step 3: Modify kubeconfig within the master node
echo "Modifying kubeconfig to replace localhost with master IP..."
multipass exec k3s-master -- sudo sed "s/127.0.0.1/$MASTER_IP/" /etc/rancher/k3s/k3s.yaml > kubeconfig.yaml

# Step 5: Launch worker nodes
for i in 1 2; do
  WORKER_NAME="worker0$i"
  WORKER_IP="192.168.64.10$i"
  WORKER_MAC="52:54:00:4b:ab:$i$i"

  echo "Launching $WORKER_NAME with IP $WORKER_IP..."
  
  # Replace placeholders in the template and write to temporary directory
  sed "s|{{WORKER_IP}}|$WORKER_IP|g; s|{{WORKER_NAME}}|$WORKER_NAME|g; s|{{WORKER_MAC}}|$WORKER_MAC|g; s|{{JOIN_COMMAND}}|$JOIN_COMMAND|g" cloud-init-worker.yaml > $TMP_DIR/worker-$i.yaml
  
  # Launch the worker node
  multipass launch --name $WORKER_NAME --cloud-init $TMP_DIR/worker-$i.yaml   --network name=en0,mode=manual,mac="$WORKER_MAC" --disk 20G --memory 3G --cpus 2 
  multipass exec $WORKER_NAME -- sudo /tmp/bootstrap.sh
done

# Step 7: Verify cluster setup using kubectl from the local machine
echo "Verifying cluster setup using kubectl..."
kubectl get nodes

echo "K3s cluster setup complete!"

# Clean up temporary files
rm -rf $TMP_DIR
