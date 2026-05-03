# Vitriol

A distributed genomic workflow engine, build to be run on the cloud, or on premise.
Designed to eliminate the operational friction and resource overhead, providing a seamless transition
from local development to massive horizontal scale in the cloud.

- Declarative Usability: Workflows are defined in [KDL](https://kdl.dev/) documents for linear and clear dependency based definitions.
Featuring proactive validation on submission.
- Split Architecture: Scale your Control Plane apart from the Data Plane Agents, deploy how many machines powered by the agents,, communicating through durable queues,
ensuring at-least-once communication.
- Designed for "not on your pc": Using object storage for reliable file provision, automatic clean up of orphan files when configured, storage and compute separated from each other.
- Isolation: Every stage is run in a container technology, ensuring reproducible* reruns, and tying with the automatic clean up of resources.
- Seamless Cloud-to-Local-to-Cloud: Every component can be run and deployed into consumer hardware, by utilizing containers and self-hosted alternatives for services,
we ensure that pipelines developed locally mirrors production environments, without requiring technical knowledge.

## Overview

Vitriol is a Rust workspace containing multiple crates:
- **lapis** - Control Plane binary crate
- **shared** - Shared utilities and common code

## Contributing

Please see [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines on how to contribute to this project.

## License

Licensed under [GNU AFFERO GENERAL PUBLIC LICENSE Version 3](./LICENSE)
