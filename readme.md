
## check k8s api

### extract pem files from kubeconfig
```
yq '.clusters[1].cluster.certificate-authority-data' < ~/.kube/config | base64 -d > ca.pem
yq '.users[1].user.client-certificate-data' < ~/.kube/config | base64 -d > client-cert.pem
yq '.users[1].user.client-key-data' < ~/.kube/config | base64 -d > client-key.pem
```

### connect k8s api endpoint by curl

```
curl -H "Accept: application/json" 'https://127.0.0.1:63454/api/v1/namespaces/default/pods?limit=500' --cacert ca.pem --cert client-cert.pem --key client-key.pem
```