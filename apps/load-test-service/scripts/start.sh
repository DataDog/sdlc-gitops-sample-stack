#!/bin/bash
# Load test service start script
# Provides easy switching between different test modes

# Set defaults
SCRIPT_TYPE=${SCRIPT_TYPE:-main}
DURATION=${DURATION:-30s}

# Parse arguments
while [[ $# -gt 0 ]]; do
  case $1 in
    --type)
      SCRIPT_TYPE="$2"
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

# Map script type to actual k6 script
case $SCRIPT_TYPE in
  main|default)
    K6_SCRIPT="/app/scripts/main.js"
    echo "Running standard test sequence (main.js)"
    ;;
  continuous|background)
    K6_SCRIPT="/app/scripts/continuous.js"
    echo "Running continuous background load test (continuous.js)"
    ;;
  *)
    # If a direct path is provided, use that
    if [[ -f "$SCRIPT_TYPE" ]]; then
      K6_SCRIPT="$SCRIPT_TYPE"
      echo "Running custom script: $SCRIPT_TYPE"
    else
      echo "Invalid script type: $SCRIPT_TYPE"
      echo "Valid options: main, continuous, or a path to a custom script"
      exit 1
    fi
    ;;
esac

# Override the duration if specified
if [[ "$DURATION" != "30s" ]]; then
  export K6_DURATION=$DURATION
  echo "Using custom duration: $DURATION"
fi

# Set the K6_SCRIPT environment variable and run k6
export K6_SCRIPT
exec k6 run $K6_SCRIPT 