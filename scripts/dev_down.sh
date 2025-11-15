#!/usr/bin/env bash
set -euo pipefail

CLUSTER_NAME="$1"

# Uninstall HAProxy if installed
if helm list -n haproxy-controller -o json | jq -e 'any(.[]; .name == "haproxy-kubernetes-ingress")' > /dev/null; then
  helm uninstall haproxy-kubernetes-ingress --namespace haproxy-controller
fi

# Stop minikube if it's running
MINIKUBE_STATUS=$(minikube -p ${CLUSTER_NAME} status --format '{{.Host}}')
if [[ ${MINIKUBE_STATUS} =~ 'Running' ]]; then
  minikube -p ${CLUSTER_NAME} stop
  minikube -p ${CLUSTER_NAME} delete
fi

echo "Development environment on minikube cluster '${CLUSTER_NAME}' has been torn down."
