set shell := ["bash", "-cu"]

default: factory-status

factory-doctor:
  ./scripts/factory doctor

factory-status:
  ./scripts/factory status

factory-check:
  ./scripts/factory check

factory-ready change:
  ./scripts/factory ready "{{change}}"

factory-receipt change:
  ./scripts/factory receipt "{{change}}"

quality:
  nix flake check
