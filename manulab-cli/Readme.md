# Caravela Cli
A Rust based Cli to interact with the Manulab GRPC Provider
Terraform like interaction create resources in homelab/proxmox

# Resources
- dock -> Workspace to logically separate resource state
- tbd

## Commands
1. shipyard == provider (installs provider locally or connects remotely)
2. sail == apply (creates resources)
3. wreck == destroy (deletes resources)
4. spot == (list resources)


### shipyard usage
- install
- get (single, all) == GetPluginInfo
- remove

### sail usage
- dry-run: bool -> soft creates resources
- plan: bool -> prints resources it will try to create

### wreck usage
- soft: duration -> Mark as soft delete to get destroyed after X duration
- hard -> Destroy resources imediately

### spot usage
- selectors: At least one selector to list resources
  - ex: --name <get by resource name>

