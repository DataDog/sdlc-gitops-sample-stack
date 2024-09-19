
Buzzword bingo:

**Application Stack**:
- **GitHub docker build pipelines** - PR checks and automatic image creation and release onto the GitHub docker registry
- **GitOps with ArgoCD** - ArgoCD manifests to deploy the stack and application manifests for ArgoCD to deploy. GitHub pipelines are used to automatically update image versions when changes are merged into main
- **Some example REST services that do important things** - A golang service simple-api provides mountain pass information, and a java service downstream-api provides a summarised view over the top

**Datadog**:
- **Service Catalog Integration** - provides a consolidated view of the deployed services, performance insights, cost impact, and ownership
- **CI/CD pipeline visibility** - monitor and assess the performance and cost impact of the GitHub and ArgoCD pipelines as your application changes pass from `main` through to running Kubernetes environments
- **Test visibility** - promote visibility of unit tests, results, and performance into your Datadog dashboards
- **Code analysis** - assess the applications and pull requests for security vulnerabilities in their code, and known vulnerabilities in included libraries. Integrated with GitHub provides feedback on pull requests.
- **Application performance monitoring** - Provide end-to-end visibility into the running applications and the underlying Kubernetes cluster, from distributed application traces to system-level metrics
