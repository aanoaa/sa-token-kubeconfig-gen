# sa-token-kubeconfig-gen

gen `kubeconfig` to use CI/CD deployment.

1. create SA
2. rolebinding to SA for restricted kube-api access
3. generate kubeconfig via SA secret

## Example

```
# rolebinding to sa
$ cat <<EOF | kubectl apply -f -
---
apiVersion: rbac.authorization.k8s.io/v1
kind: Role
metadata:
  namespace: default
  name: my-app-updater
rules:
- apiGroups: ["apps"]
  resources: ["deployments"]
  verbs: ["get", "list", "update", "patch"]
---
apiVersion: rbac.authorization.k8s.io/v1
kind: RoleBinding
metadata:
  name: my-app-updater
  namespace: default
subjects:
- kind: ServiceAccount
  name: my-app-sa
  namespace: default
roleRef:
  kind: Role
  name: my-app-updater
  apiGroup: rbac.authorization.k8s.io
EOF

# capture sa token from sa-secret
$ TOKEN=`kubectl get secret my-app-token-pr7z7 -o jsonpath='{.data.token}' | base64 --decode`

## generate kubeconfig for deployment

# 1. token via stdin
$ echo $TOKEN | cargo run > deploy-kubeconfig.yaml
$ cargo run > deploy-kubeconfig.yaml

# 2. token via current kubeconfig context secret
$ cargo run -- some-namespace my-app-token-pr7z7 > deploy-kubeconfig.yaml

# verify
$ kubectl get deploy --kubeconfig ./deploy-kubeconfig.yaml
```
