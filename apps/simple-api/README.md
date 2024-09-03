# Simple-API 
[![Main Autorelease](https://github.com/johnbailye/dd-software-lifecycle-demo/actions/workflows/main-autorelease.yaml/badge.svg)](https://github.com/johnbailye/dd-software-lifecycle-demo/actions/workflows/main-autorelease.yaml)

A simple golang RESTful API that records mountain pass information instrumented with the Datadog tracing library.

## Usage
```bash
# Create 
curl -X POST http://localhost:8080/passes \
-H "Content-Type: application/json" \
-d '{
    "name": "Gotthard Pass",
    "country": "Switzerland",
    "ascent": 2106
}'

# Read
curl http://localhost:8080/passes/4

# Delete
curl -X DELETE http://localhost:8080/passes/4

```

## TODO - Setup Notes
* [deploy the DD agent](TODO)
* [setup source integration](https://docs.datadoghq.com/integrations/guide/source-code-integration/?tab=go)