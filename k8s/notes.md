## Setup

### 1. Kubernetes
Used [This guide](https://blog.kubesimplify.com/kubernetes-on-apple-macbooks-m-series) to start with. Automated in `setup.sh`




### 2. Datadog
Use the operator to get started
TODO - system-probe doesn't work in k3d

### 3. Setup Gitlab
[deployment guide](https://docs.gitlab.com/charts/installation/deployment.html) for dev env with helm

Get the root secret:
```
kubectl get secret gitetlab-gitlab-initial-root-password -ojsonpath='{.data.password}' | base64 --decode ; echo
```


### 4. Setup Argo



## Setup - Round 2
Let's use minikube - https://devopscube.com/minikube-mac/

```bash
 minikube start --driver qemu --network socket_vmnet
 ```

 no good - 
 ```
 failed to start manager: failed to clean up tracefs: read kprobe_events: open /sys/kernel/tracing/kprobe_events: no such file or directory. Falling back to runtime_helpers.go:93 in compileToObjectFile | Successfully compiled runtime version of oom-kill.c
error creating module oom_kill_probe: unable to start the OOM kill probe: failed to start manager: failed to clean up tracefs: read kprobe_events: open /sys/kernel/tracing/kprobe_events: no such file or directory
failed to run constant fetcher: failed to clean up tracefs: read kprobe_events: open /sys/kernel/tracing/kprobe_events: no such file or directory her.go:110 in fillConstantCacheIfNeeded
Instantiating CWS rule engine go:57 in createEventMonitorModule | Event monitoring CWS consumer initialised
event monitoring network consumer initialised go:70 in createEventMonitorModule
tagger stream established successfully ger.go:390 in func1
remote tagger initialised successfully ger.go:160 in Start
error registering HTTP endpoints for module event_monitor: failed to set up probe: failed to clean up tracefs: read kprobe_events: open /sys/kernel/tracing/kprobe_events: no such file or directory :103 in Register
error while starting API server, exiting: failed to create system probe: no module could be loaded command.go:353 in startSystemProbe
```

## Setup - Round 3
Let's use minikube, and turn on cilium too:

```bash
 minikube start # --cni=cilium # Don't use the cilium CNI, it breaks `minikube tunnel`

 # Enable the ingress controller
 minikube addons enable ingress
 ```

 Now the DD agent deploys with the operator.
 ```bash
helm repo add datadog https://helm.datadoghq.com && helm install my-datadog-operator datadog/datadog-operator
kubectl create secret generic datadog-secret --from-literal api-key={{your_api_key}} --from-literal app-key={{your_app_key}}
kubectl apply -f core-services/dd-agent.yaml
 ```


Let's install bits we need:

```bash
https://argo-cd.readthedocs.io/en/stable/getting_started/

```bash
# Install argo
kubectl create namespace argocd
kubectl apply -n argocd -f https://raw.githubusercontent.com/argoproj/argo-cd/stable/manifests/install.yaml

# Get argoCD default password
# User is 'admin'
kubectl get secret argocd-initial-admin-secret -n argocd -o jsonpath="{.data.password}" | base64 -d

# Access ArgoCD using minikube service
minikube service argocd-server -n argocd --url
```

Cool - let's go and deploy some uncontroversial argocd project from https://github.com/argoproj/argocd-example-apps.git
```bash 
kubectl create namespace guestbook

# Go and create the app in the argoCD UI
```

# Access ArgoCD using tunnel (this is a bit jank so _NO_)

```
Let's setup some host maps in `/etc/hosts` for the services we want:
```
127.0.0.1    gitlab.localdev
127.0.0.1    argo.localdev
```

And we can start `minikube tunnel` to open a listener on localhost:80 and localhost:443 to pick these up:
```bash
minikube tunnel
```

# Setup Gitlab
```bash
kubectl apply -f gitlab.yaml # TODO - this is my local copy, hacked to work with the ARM64 images
```

"Connection failed. Check your integration settings."