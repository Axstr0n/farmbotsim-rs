# farmbotsim-analysis

This crate includes experiment and analysis of simulation.

`farmbotsim-analysis/`
- `src/bin/` - binaries for analysis
  - `analyze.rs` - Runs analysis of json file and outputs plots, tables.
  - `experiment.rs` - Runs multiple simulations and store output in json file.
  - `measure_sim_time.rs` - Runs simulations and outputs average sim time for agent counts.
- `Cargo.toml` - Contains dependencies of crate.
- `README.md` - This file.