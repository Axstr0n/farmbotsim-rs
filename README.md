# farmbotsim

**!! IN DEVELOPMENT !!**

**You can download the latest version of this application from the Releases page (Windows only).**

---

**farmbotsim-rs** was made to assist with real-world agricultural automation by providing a reliable simulation. In this simulation we can focus on charging strategies and productivity matrix.

**Screenshots of application:**
<table>
  <tr>
    <td><img src="media/tool_simulation.png" alt="simulation_tool" ></td>
    <td><img src="media/tool_path.png" alt="path_tool" ></td>
  </tr>
  <tr>
    <td><img src="media/tool_task.png" alt="task_tool" ></td>
    <td><img src="media/tool_editor.png" alt="editor_tool"></td>
  </tr>
  <tr>
    <td><img src="media/tool_battery.png" alt="battery_tool"></td>
    <td></td>
  </tr>
</table>

## Table of Contents
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Usage](#usage)
- [Code Structure Overview](#code-structure-overview)

## Prerequisites

Before getting started, you need to have **[Rust](https://www.rust-lang.org/tools/install)** installed on your machine.

Additionally, it's recommended to have **[Git](https://git-scm.com/)** installed to clone the repository.

## Installation

Follow these steps to get farmbotsim running locally:

1. Clone the repository:
   ```bash
   git clone https://github.com/Axstr0n/farmbotsim-rs.git
   ```

2. Navigate to the project directory:
    ```bash
    cd farmbotsim-rs
    ```

## Usage
After installation you can run main file like:
```
cargo run
```
This will run whole application. (Note: running first time takes longer to build)

## Building project
To build the project in release mode use:
```bash
cargo build --release
```

## Code Structure Overview

The **farmbotsim-rs** project is organized into several directories that help separate functionality. Here's a high-level breakdown:

`batteries/` - Contains battery configs.

`configs/`
- `crop_plans/` - Contains plans for crop growth.
- `env_configs/` - Contains configs with all parameters for env creation.

`media/`- Contains screenshots of app.

`src/` - Contains the core logic of the application:
- `agent_module/` - Contains the agent struct and its associated logic, such as movement, battery, state machine.
- `app_module/` - Main app functionality.
- `environment/` - Contains all environment structs (Crop, Field, Station, Env, Config, ...).
- `path_finding_module/` - Includes code related to navigation and pathfinding algorithms.
- `rendering/` - Responsible for rendering.
- `task_module` - Includes files for task creation and task handling.
- `tool_module/` - Contains files for app modes (simulation, editor, path, task, ...).
- `utilities/` - Common utilities and helper functions used across the project.
- `cfg.rs` - Contains constants.
- `main.rs` - Contains entry point into application.

`.gitignore` - Ignores files/folders.

`Cargo.lock` - Records the exact versions of dependencies used for this project.

`Cargo.toml` - Contains dependencies of project.

`README.md` - This file, which contains documentation and instructions for setting up and using the application.
