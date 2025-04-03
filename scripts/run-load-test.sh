#!/bin/bash
# Helper script to run one-off load tests in Kubernetes
# Usage: ./scripts/run-load-test.sh [--type main|continuous] [--duration 30s|1m|etc]

# Set defaults
TYPE="main"
DURATION="30s"

# Parse arguments
while [[ $# -gt 0 ]]; do
  case $1 in
    --type)
      TYPE="$2"
      shift 2
      ;;
    --duration)
      DURATION="$2"
      shift 2
      ;;
    *)
      echo "Unknown option: $1"
      exit 1
      ;;
  esac
done

# Generate a unique name for the pod
POD_NAME="load-test-$(date +%s)"

# Create a temporary pod YAML
cat <<EOF > /tmp/load-test-pod.yaml
apiVersion: v1
kind: Pod
metadata:
  name: $POD_NAME
spec:
  restartPolicy: Never
  containers:
  - name: load-test
    image: load-test-service:latest
    args: ["--type", "$TYPE", "--duration", "$DURATION"]
    env:
    - name: PASS_API_URL
      value: "http://pass-api:8080"
    - name: PASS_IMAGE_API_URL
      value: "http://pass-image-api:8080"
    - name: PASS_SUMMARY_API_URL
      value: "http://pass-summary-api:8080"
EOF

echo "Starting load test pod: $POD_NAME"
echo "Type: $TYPE, Duration: $DURATION"

# Create the pod
kubectl apply -f /tmp/load-test-pod.yaml

# Follow the logs
echo "Following logs (press Ctrl+C to stop following, the test will continue):"
kubectl logs -f $POD_NAME

# Clean up advice
echo ""
echo "The test pod $POD_NAME will be automatically cleaned up when it completes."
echo "To manually delete it, run: kubectl delete pod $POD_NAME" 