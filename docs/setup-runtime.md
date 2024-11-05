## Setup Runtime Visibility
Now that we've got visibility into the early stages of the development lifecycle, lets move onto runtime visibility!  The **Mountain Passes** stack in this repository is designed to be deployed to a Kubernetes cluster using ArgoCD. In order to run it, you'll need an appropriate Kubernetes setup.

In particular, you will need the  **Datadog Operator** (to collect logs, traces, and metrics) and **ArgoCD** (to deploy the stack) installed and running. The easiest way to get started is using the [Datadog operator](https://docs.datadoghq.com/getting_started/containers/datadog_operator/) and the [ArgoCD getting started guided](https://argo-cd.readthedocs.io/en/stable/getting_started/). Given an existing cluster and familiarity with Kubernetes these steps shouldn’t take too long. 
Your operator configuration will need _at least_ the following flags configured in order to fully support the app:

```yaml
apiVersion: datadoghq.com/v2alpha1
kind: DatadogAgent
metadata:
  name: datadog
spec:
  override:
    nodeAgent:
      env:
        # Ensures that the agent will accept logs over OTLP, necessary
        # for the pass-image-api
        - name: DD_OTLP_CONFIG_LOGS_ENABLED
          value: "true"
  global:
    # Make sure to set the site correctly if you are not using Datadog US!
    site: datadoghq.com

    # Set your credentials up normally. For example ... 
    credentials:
      apiSecret:
        secretName: datadog-secret
        keyName: api-key
      appSecret:
        secretName: datadog-secret
        keyName: app-key
  features:
    # Ensure the gRPC receive is enabled for OTLP. The pass-image-api needs this
    otlp:
        receiver:
          protocols:
            grpc:
              enabled: true

    # Ensure APM is enabled, so that we can collect traces
    apm:
      enabled: true

    # Log collection is needed for pass-api and pass-summary-api to export their logs
    logCollection:
      enabled: true
      containerCollectAll: true

    # These are not directly related but are fairly typical
    liveProcessCollection:
      enabled: true
    processDiscovery:
      enabled: true
```

> [!TIP]
> If you'd like to try this out on your laptop, [minikube](https://minikube.sigs.k8s.io/) is a great tool to quickly spin up a local cluster, and will work well with everything needed for this demo.

Once you've got Datadog and ArgoCD in place, you can move on to [deploying the app!](setup-runtime-deploy.md)

