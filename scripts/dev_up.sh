#!/usr/bin/env bash
set -euo pipefail

CLUSTER_NAME="$1"
NAMESPACE="$2"
HAPROXY_CONFIG_PATH=k8s/haproxy-values-dev.yaml
HAPROXY_GATEWAYCLASS_PATH=k8s/haproxy-ingress-gatewayclass.yaml
HAPROXY_GATEWAY_PATH=k8s/gateway.yaml

# Check if minikube is running; if not, start it
MINIKUBE_STATUS=$(minikube -p ${CLUSTER_NAME} status --format '{{.Host}}' || true)
if [[ ! ${MINIKUBE_STATUS} =~ 'Running' ]]; then
  minikube -p ${CLUSTER_NAME} start
  minikube -p ${CLUSTER_NAME} addons enable metrics-server
fi

# Set up Docker environment to use minikube's Docker daemon
eval $(minikube -p ${CLUSTER_NAME} docker-env)
minikube profile ${CLUSTER_NAME}

# Install HAProxy
if ! helm list -n haproxy-controller -o json | jq -e 'any(.[]; .name == "haproxy-kubernetes-ingress")' > /dev/null; then
  echo "Installing HAProxy Ingress Controller..."
  kubectl apply -f https://github.com/kubernetes-sigs/gateway-api/releases/download/v0.5.1/experimental-install.yaml
  kubectl apply -f https://raw.githubusercontent.com/haproxytech/kubernetes-ingress/master/deploy/tests/config/experimental/gwapi-rbac.yaml
  helm repo add haproxytech https://haproxytech.github.io/helm-charts
  helm repo update
  helm upgrade --install haproxy-kubernetes-ingress haproxytech/kubernetes-ingress \
    --namespace haproxy-controller \
    --create-namespace \
    -f ${HAPROXY_CONFIG_PATH}
  echo "Installing Gateway class..."
  kubectl apply -f ${HAPROXY_GATEWAYCLASS_PATH}
  echo "Installing Gateway..."
  kubectl apply -f ${HAPROXY_GATEWAY_PATH}
fi

skaffold config set --global collect-metrics false
skaffold config set --kube-context ${CLUSTER_NAME} local-cluster true

# Create namespace if it doesn't exist
if ! kubectl get namespace ${NAMESPACE} > /dev/null 2>&1; then
  kubectl create namespace ${NAMESPACE}
fi

# Create secrets
echo "Creating secrets from .env file..."
kubectl --namespace ${NAMESPACE} create secret generic ${CLUSTER_NAME}-secret --from-env-file=.env \
  --dry-run=client -o yaml | kubectl apply -f -

echo "Development environment is set up on the minikube cluster '${CLUSTER_NAME}'."

skaffold dev
