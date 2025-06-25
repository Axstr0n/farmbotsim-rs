# Farmbotsim-rs
*markdown is meant to be viewed in app*

This is a general overview of code.

## Logger
App includes logger that on failure will log message in `log.txt` and panics the app.

## Units

This project uses a custom unit system to represent and manage physical quantities. Each quantity is strongly typed to ensure unit correctness and allow intuitive conversions and calculations.

Each quantity follows a consistent internal format:
```rust
struct Quantity {
    value: f32,
    unit: UnitType,
}
```
Implemented quantities and their units are shown below.

| **Quantity**         | **Supported Units**    | **Base Unit** |
| -------------------- | ---------------------- | ------------- |
| **Angle**            | `deg`, `rad`           | `rad`         |
| **Angular Velocity** | `deg/s`, `rad/s`       | `rad/s`       |
| **Duration**         | `s`, `min`, `h`, `day` | `s`           |
| **Energy**           | `J`, `kJ`, `Wh`, `kWh` | `J`           |
| **Length**           | `mm`, `cm`, `m`        | `m`           |
| **Linear Velocity**  | `m/s`, `km/h`          | `m/s`         |
| **Power**            | `W`, `kW`              | `W`           |
| **Voltage**          | `V`, `mV`              | `V`           |

## Movement Module

Movement module contains logic for pose, moving, calculating inputs, inputs.

### Pose

Pose represents position and orientation of point.

```rust
pub struct Pose {
    pub position: Pos2,
    pub orientation: Angle,
}
```

It is used as pose for agent, station, ...

### Movement

This project supports a single movement model: Romba Movement, based on differential drive kinematics.

```rust
pub struct RombaMovement {
    pub max_velocity: LinearVelocity,
    pub max_angular_velocity: AngularVelocity,
    pub wheel_distance: Length,
    pub wheel_radius: Length
}
```

Romba Movement is a differential drive system where two wheels operate independently:
- Turning in place is achieved by spinning the wheels in opposite directions.
- Driving straight uses equal input on both wheels.
- There is no strafing or omnidirectional control — it's purely forward/backward and rotational.


Movement has 2 main methods:
- from current pose and target pose returns inputs for movement
- from current pose and inputs returns new pose and velocities


### Movement Inputs

Inputs for romba:

```rust
pub struct RombaMovementInputs {
    pub left: f32,
    pub right: f32,
}
```
Both left and right values are in the range -1.0..=1.0.

## Battery Module

In this project there is one battery:

```rust
pub struct Battery {
    pub voltage: Voltage,
    pub capacity: Energy,
    pub soc: f32,
    pub energy: Energy,
    pub jan_max_data: Vec<(u32, f32)>,
    pub jan_min_data: Vec<(u32, f32)>,
    pub jun_max_data: Vec<(u32, f32)>,
    pub start_index: HashMap<String, usize>,

    update_count: u32,
    pub soc_history: VecDeque<f32>,
}
```

All data fields are gathered from txt files with stats.

### Characteristics

Charging is dependant on season and is not linear.

![charging_characteristics](general_help/images/charge_characteristics.png)

Discharging is linear based on power usage.

![discharging_characteristics](general_help/images/discharge_characteristics.png)

## Agent Module

Stores all logic for agent.

### Agent
Agent represents robot/AMR that is moving, doing tasks and charging.

```rust
pub struct Agent {
    pub id: AgentId, // unique id
    pub pose: Pose,
    pub movement: Movement,
    pub velocity_lin: LinearVelocity,
    pub velocity_ang: AngularVelocity,

    pub work_schedule: WorkSchedule, // stores tasks in queue
    pub current_task: Option<Task>,

    pub state: AgentState,
    pub battery: Battery,
}
```
Agent is created with `AgentConfig`:

```rust
pub struct AgentConfig {
    pub movement: String, // path to movement config
    pub battery: String, // path to battery config
    pub battery_soc: f32, // initial state of charge
}
```

### Agent States

Agent has his own state machine with states:
- Wait
- Travel
- Work
- Charge
- Discharged

Their relations and transitions are shown below.

![agent_state_machine](general_help/images/agent_state_machine.png)


## Farm Entity Module

### Farm Entity
Is generic entity that can be worked on.
```rust
pub enum FarmEntity {
    Crop(Crop),
    Row(Row),
}
```
- **Crop** is point entity which has actions that are stationary(velocity=0)
- **Row** is line entity which has actions that are moving(velocity>0)

### Farm Entity Plan
All farm entities have plan:
```rust
pub struct FarmEntityPlan {
    pub crop_name: String,
    pub type_ : String,
    pub cycle : Option<u32>,
    pub schedule: Vec<FarmEntityAction>,
}
```
- **type** - (line or point) (point for Crop , line for Row)
- **cycle** - Represents if the schedule cycles and from which index
- **schedule** - Defines lifecycle of entity (example: plant, water, wait, fertilize, harvest)

### Farm Entity Action
Plan has actions of 3 types:
```rust
pub enum FarmEntityAction {
    Point {
        action_name: String,
        duration: Duration,
        power: Power,
    },
    Line {
        action_name: String,
        velocity: LinearVelocity,
        power: Power,
    },
    Wait {
        action_name: String,
        duration: Duration,
    }
}
```

- **Point** - point action with a fixed duration and power (stationary action)
- **Line** - line action representing movement along a path with velocity and power (moving action)
- **Wait** - wait action with a specified duration

Each action needs to be converted to task but before that is converted to **FarmEntityActionInstance** which is similar to Action but contains specific id of entity, field and position/path.

## Field Config

Has data of all fields.

There are 2 types of field:
```rust
pub enum VariantFieldConfig {
    Line(LineFieldConfig),
    Point(PointFieldConfig),
}
```

- **PointFieldConfig** - represents field with points (Crops) and has parameters to costumize dimensions
- **LineFieldConfig** - represents field with lines (Rows) and has adjustable parameters for it

Both have path to file with plan for entity.

## Charging Station

For now is stationary station where agents can come to charge.

It is created from config:
```rust
pub struct StationConfig {
    pub pose: Pose,
    pub queue_direction: Angle,
    pub waiting_offset: Length,
    pub n_slots: u32,
    pub slots_pose: Vec<Pose>,
}
```
- **pose** - position and orientation of station
- **queue direction** - direction in which agents queue
- **waiting offset** - distance between queued agents
- **n slots** - number of charging slots
- **slots pose** - relative poses for each slot

## Spawn Area

Is area where agents spawn when environment is created.

It is rectangle with **position**, **width**, **height** and **angle**.


## Scene Config

Scene represent stationary environment:
```rust
pub struct SceneConfig {
    pub field_config_path: String,
    pub station_configs: Vec<StationConfig>,
    pub spawn_area_config: SpawnAreaConfig,
}
```

- **field_config_path** - path to the field configuration file
- **station_configs** - list of configurations for stations in the scene
- **spawn_area_config** - configuration for the spawn area within the scene

## Task Module

### Intent
Represents the intention of a task.
```rust
pub enum Intent {
    /// Performing work-related tasks
    Work,
    /// Charging at a station
    Charge,
    /// Waiting in a queue for a station slot
    Queue,
    /// Idle
    Idle
}
```
### Task
Represents different types of tasks an agent can perform, including stationary/moving work, travel, and waiting.
```rust
pub enum Task {
    /// A work stationary task at a specific pose, with duration and associated metadata.
    Stationary {
        id: u32,
        pose: Pose,
        duration: Duration,
        intent: Intent,
        farm_entity_id: u32,
        field_id: u32,
        line_id: u32,
        power: Power,
        info: String,
    },
    /// A work moving task along a path at a specified velocity, with associated metadata.
    Moving {
        id: u32,
        path: VecDeque<Pose>,
        velocity: LinearVelocity,
        intent: Intent,
        field_id: u32,
        farm_entity_id: u32,
        power: Power,
        info: String,
    },
    /// A travel task representing movement along a path.
    Travel {
        path: VecDeque<Pose>,
        velocity: LinearVelocity,
        intent: Intent,
    },
    /// A waiting task for a specified duration with an intent.
    WaitDuration {
        duration: Duration,
        intent: Intent,
    },
    /// A waiting task of indefinite length with an intent.
    WaitInfinite {
        intent: Intent,
    }
}
```

### Task Manager

Manages task assignment, tracking, and execution for farm entities.

It has main method:
- **assign_tasks** - for each agent assignes most relevant task at the moment

It is created from config that stores its strategies.
```rust
pub struct TaskManagerConfig {
    pub charging_strategy: ChargingStrategy,
    pub choose_station_strategy: ChooseStationStrategy,
}
```

### Strategies

Strategy for station selection.
```rust
pub enum ChooseStationStrategy {
    /// Choose closest charging station (Manhattan)
    ClosestManhattan,
    /// Choose closest charging station (Path)
    ClosestPath,
    /// Choose closest charging station with min queue (Manhattan)
    ClosestMinQueueManhattan,
    /// Choose closest charging station with min queue (Path)
    ClosestMinQueuePath,
}
```
Strategy for when to go charging.
```rust
pub enum ChargingStrategy {
    /// Go charging only if battery is bellow critical
    CriticalOnly,
    /// Go charging if battery is bellow threshold and station is available
    /// Go charging if battery is bellow critical
    ThresholdWithLimit,
}
```

## Datetime
Datetime stores and advances time in simulation. It is necessary for battery to work because it is dependant on seasons.

It is created with config:
```rust
pub struct DateTimeConfig {
    /// Date string (format "dd.mm.yyyy").
    pub date: String,
    /// Time string (format "HH:MM:SS").
    pub time: String,
}
```

## Env
Represents the environment of the simulation including agents, field, stations, obstacles,and management of time and tasks.

It has 2 main methods:
- **step** - increments simulation by one step
- **reset** - resets env

It is created with config:
```rust
pub struct EnvConfig {
    /// Number of agents in the environment.
    pub n_agents: u32,
    /// Path to the agent configuration file.
    pub agent_config_path: String,
    /// Configuration for date and time settings.
    pub datetime_config: DateTimeConfig,
    /// Path to the scene configuration file.
    pub scene_config_path: String,
    /// Configuration for the task manager.
    pub task_manager_config: TaskManagerConfig,
}
```

## Pathfinding
Pathfinding is done with visibility graph. 

On creation it is supplied with graph points and obstacles. It connects all graph point where line between 2 point doesnt intersect obstacle.

It has main method:
- **find path** - finds path from start to end position with a*. if no path returns None

