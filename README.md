# Datadog Software Lifecycle Kubernetes Extravaganza

> [!IMPORTANT]  
> This repo is a WIP and not yet ready to publish. It is being shared in this state internally in order to solicit feedback from the rest of the Datadog pack. 

[![Main Autorelease](https://github.com/scottgerring/dd-software-lifecycle-demo/actions/workflows/main-autorelease.yaml/badge.svg)](https://github.com/scottgerring/dd-software-lifecycle-demo/actions/workflows/main-autorelease.yaml)

This repo gives you a complete, cloneable, end-to-end demo of an golang application hooked up to DataDog and Kubernetes.
Buzzword bingo:

* **Github pipelines CI/CD** -> Datadog Software Delivery stack 
* Shaped like a cloneable ArgoCD GitOps repository, complete with Github pipelines to do the boring/necessary gitops bits
* Datadog logs/traces/profiles integration out of the box 

Want to get started with Datadog + end-to-end SDLC onto Kubernetes? You're in the right place.

# TODO

* [ ] Remove "Just deploy it"
* [ ] Add arch diagrams 
* [ ] Explain setting up secrets - 

# Getting Started


### 1. Just Deploy it 🚀
_**(See what it does)**_

If you want to see your logs/traces/metrics from the app you'll first need to install the Datadog agent in your cluster.
The easiest way to get started is using the [Datadog operator](https://docs.datadoghq.com/getting_started/containers/datadog_operator/). 

Although you can simple check this repository out and apply the manifests directly, you'll need ArgoCD for the full _GitOps experience_. 
This is simply a matter of [applying a manifest to your cluster](https://argo-cd.readthedocs.io/en/stable/getting_started/). 

Finally, with all that in place, point your ArgoCD instance at: 

* **Repo URL**: `https://github.com/scottgerring/dd-software-lifecycle-demo.git`
* **Path**: `manifests/overlays/dev`

That's it! Changes to this repository can be synced manually or automatically into your cluster. 

### 2. Fork it 🍴
_**(Use it as a base for your own GitOps+Datadog setup)**_

#### 2.1 - Fork and build

First, we need to fork the repository and trigger its builds, so that our fork contains published
docker images.

* **Fork the repository using the Github UI**

#### 2.2 - Integrate with Datadog CD Visibility
TODO

#### 2.3 Build It

* **Create a PR to your fork**  - This should trigger a build action on the PR dialog, validating the code base and the build setup
* **Merge the PR** - This should trigger the release action on the `main` branch - watch the progress from the `Actions` section of the UI. After 10 minutes, you should have a docker package published to your project's releases and a new commit on `main` referencing it.

#### 2.4 Prepare Cluster

* TODO Deploy Datadog operator
* TODO Deploy ArgoCD


TODO - Test Change - Remove
#### 2.5 Deploy It

TODO

