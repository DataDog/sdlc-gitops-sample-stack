## Setup Runtime Visibility
Now that we've got visibility into the early stages of the development lifecycle, lets move onto runtime visibility!  The **Mountain Passes** stack in this repository is designed to be deployed to a Kubernetes cluster using ArgoCD. In order to run it, you'll need an appropriate Kubernetes setup.

In particular, you will need the  **Datadog Operator** (to collect logs, traces, and metrics) and **ArgoCD** (to deploy the stack) installed and running. The easiest way to get started is using the [Datadog operator](https://docs.datadoghq.com/getting_started/containers/datadog_operator/) and the [ArgoCD getting started guided](https://argo-cd.readthedocs.io/en/stable/getting_started/). Given an existing cluster and familiarity with Kubernetes these steps shouldn’t take too long. 

> [!TIP]
> If you'd like to try this out on your laptop, [minikube](https://minikube.sigs.k8s.io/) is a great tool to quickly spin up a local cluster, and will work well with everything needed for this demo.

Once you've got Datadog and ArgoCD in place, you can move on to [deploying the app!](setup-runtime-deploy.md)

