# sa-token-kubeconfig-gen

gen `kubeconfig` to use CI/CD deployment.

1. create SA
2. rolebinding, clusterrolebinding to SA for restricted kube-api access
3. generate kubeconfig via SA token secret and current kubeconfig
