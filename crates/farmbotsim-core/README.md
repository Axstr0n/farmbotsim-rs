# farmbotsim-core

This crate includes core functionality of simulation.

`farmbotsim-core/`
- `src/` - Contains the core logic of the application:
  - `agent_module/` - Contains the agent struct and its associated logic, state machine.
  - `battery_module/` - Containing battery logic.
  - `environment/` - Contains all environment structs (Crop, Field, Station, Env, Config, ...).
    - `env_module/` - Contains logic for env.
    - `farm_entity_module/` - Contains logic for farm entity.
    - `spawn_area_module/` - Contains logic for spawn area.
    - `station_module/` - Contains logic for station.
    - `...`
  - `movement_module/` - Contains movement logic
  - `path_finding_module/` - Includes code related to navigation and pathfinding algorithms.
  - `task_module/` - Includes files for task creation and task handling.
  - `units/` - Unit system.
  - `utilities/` - Common utilities and helper functions used across the project.
  - `cfg.rs` - Contains constants.
  - `lib.rs` - Library for crate.
  - `logger.rs` - Logger for application.
  - `prelude.rs` - For importing whole core.
  - `statistics.rs` - Contains stats needed in simulation.
- `Cargo.toml` - Contains dependencies of crate.
- `README.md` - This file.