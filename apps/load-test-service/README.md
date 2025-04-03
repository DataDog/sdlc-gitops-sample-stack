# Load Test Service

This service provides load testing for the microservices in this stack, including:
- pass-api
- pass-image-api
- pass-summary-api

## Overview

The load test service uses [k6](https://k6.io/) to generate consistent, configurable load against the other services. It has several test modes:

1. **Standard Test**: Executes a sequence of targeted tests against each service
2. **Continuous Mode**: Runs a lower-intensity, long-running test suitable for background load

## Configuration

The service can be configured using environment variables:

| Variable | Description | Default |
|----------|-------------|---------|
| PASS_API_URL | URL for the pass-api service | http://pass-api:8080 |
| PASS_IMAGE_API_URL | URL for the pass-image-api service | http://pass-image-api:8080 |
| PASS_SUMMARY_API_URL | URL for the pass-summary-api service | http://pass-summary-api:8080 |
| PASS_API_RPS | Requests per second for pass-api | 10 |
| PASS_IMAGE_API_RPS | Requests per second for pass-image-api | 5 |
| PASS_SUMMARY_API_RPS | Requests per second for pass-summary-api | 15 |
| K6_SCRIPT | Script to run | /app/scripts/main.js |

## Running the Service

### In Docker

```bash
docker-compose up load-test-service
```

### In Kubernetes

```bash
kubectl apply -f manifests/base/load-test-service
```

## Test Descriptions

### Main Test (main.js)

Runs a sequence of load tests:
1. Pass API test (30s) - Tests the pass-api endpoints
2. Pass Image API test (30s) - Tests the pass-image-api endpoints
3. Pass Summary API test (30s) - Tests the pass-summary-api endpoints
4. Combined workflow test (60s) - Tests an end-to-end workflow that touches all services

### Continuous Test (continuous.js)

Runs a continuous load test that:
- Distributes traffic across all services
- Simulates realistic access patterns
- Runs indefinitely (configured for 24h by default)
- Maintains a lower request rate to avoid overwhelming the system

## Custom Test Runs

To run a specific test script:

```bash
# In Docker
docker-compose run -e K6_SCRIPT=/app/scripts/continuous.js load-test-service

# In Kubernetes
kubectl exec -it deployment/load-test-service -- k6 run /app/scripts/continuous.js
``` 