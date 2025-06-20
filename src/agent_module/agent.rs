use egui::{Color32, Pos2, Vec2};

use crate::{
    agent_module::{
        agent_config::AgentConfig, agent_state::AgentState, battery::{BatteryConfig, BatteryPack}, work_schedule::WorkSchedule
    }, cfg::TOLERANCE_DISTANCE, environment::datetime::DateTimeManager, movement_module::{is_movement::IsMovement, movement::{Movement, MovementInputs}, pose::Pose}, task_module::task::Task, units::{
        angular_velocity::AngularVelocity,
        duration::Duration,
        linear_velocity::LinearVelocity,
    }, utilities::pos2::ExtendedPos2
};


#[derive(Clone, Debug, PartialEq)]
pub struct Agent {
    pub id: u32,
    pub pose: Pose,
    pub movement: Movement,
    pub velocity_lin: LinearVelocity,
    pub velocity_ang: AngularVelocity,
    pub color: Color32,
    pub spawn_position: Pos2,

    pub work_schedule: WorkSchedule,
    pub current_task: Option<Task>,
    pub completed_task_ids: Vec<u32>, // for storing so task manager can know

    pub state: AgentState,
    pub battery: BatteryPack,
}

impl Agent {
    pub fn from_config(config: AgentConfig, id: u32, position: Pos2, direction: Vec2, color: Color32) -> Self {
        Self {
            id,
            pose: Pose::new(position, direction),
            movement: Movement::from_json_file(config.movement),
            velocity_lin: LinearVelocity::ZERO,
            velocity_ang: AngularVelocity::ZERO,
            color,
            spawn_position: position,

            work_schedule: WorkSchedule::default(),
            current_task: None,
            completed_task_ids: vec![],

            state: AgentState::Wait,
            battery: BatteryPack::from_config(BatteryConfig::from_json_file(config.battery), config.battery_soc),
        }
    }

    pub fn update(&mut self, simulation_step: Duration, date_time_manager: &DateTimeManager) {
        if self.state == AgentState::Discharged { return }
        self.update_state(simulation_step,date_time_manager);

        self.update_task_and_path(simulation_step);
        let inputs = self._get_inputs();
        self._move(simulation_step, inputs);
    }

    fn update_state(&mut self, simulation_step: Duration, date_time_manager: &DateTimeManager) {
        let mut current_state = std::mem::replace(&mut self.state, AgentState::Wait); // placeholder

        let maybe_new_state = current_state.update(simulation_step, self, date_time_manager);

        if let Some(mut new_state) = maybe_new_state {
            current_state.on_exit(self);
            new_state.on_enter(self);
            self.state = new_state;
        } else {
            self.state = current_state;
        }
    }
    
    fn _move(&mut self, simulation_step: Duration, inputs: MovementInputs) {
        let current_task_velocity = self.current_task.as_ref().map(|task| task.get_velocity()).unwrap_or(LinearVelocity::ZERO);
        let (new_pose, new_velocity_l, new_velocity_a) = self.movement.calculate_new_pose_from_inputs(
            simulation_step, inputs, self.pose.clone(), current_task_velocity
        );
    
        // Now assign the new values
        self.pose = new_pose;
        self.velocity_lin = new_velocity_l;
        self.velocity_ang = new_velocity_a;
    }
    
    fn _get_inputs(&self) -> MovementInputs {
        let next_pose = match &self.current_task {
            Some(task) => {
                if let Some(pose) = task.get_first_pose() {
                    pose.clone()
                }
                else {
                    self.pose.clone()
                }
                
            },
            _ => {
                self.pose.clone()
            }
        };
        self.movement.clone().calculate_inputs_for_target(
            self.pose.clone(), next_pose
        )
    }
    
    fn update_task_and_path(&mut self, simulation_step: Duration) {
        if let Some(task) = &mut self.current_task {
            match task {
                Task::Stationary { pose, duration, .. } => {
                    if self.pose.position.is_close_to(pose.position, TOLERANCE_DISTANCE) {
                        if *duration > Duration::ZERO {
                            *duration = *duration - simulation_step;
                        } else if let Some(id) = task.get_id() {
                            self.completed_task_ids.push(*id);
                            self.current_task = self.work_schedule.pop_front();
                        }
                    }
                }
                Task::WaitDuration { duration , ..} => {
                    if *duration > Duration::ZERO {
                        *duration = *duration - Duration::ZERO;
                    } else {
                        self.current_task = self.work_schedule.pop_front();
                    }
                }
                Task::WaitInfinite { .. } => { }
                Task::Moving { path, .. } => {
                    while !path.is_empty() {
                        if self.pose.position.is_close_to(path[0].position, TOLERANCE_DISTANCE) {
                            path.remove(0);
                        } else {
                            break;
                        }
                    }
                    if path.is_empty() {
                        if let Some(id) = task.get_id() {
                            self.completed_task_ids.push(*id);
                        }
                        self.current_task = self.work_schedule.pop_front();
                    }
                }
                Task::Travel { path, .. } => {
                    while !path.is_empty() {
                        if self.pose.position.is_close_to(path[0].position, TOLERANCE_DISTANCE) {
                            path.remove(0);
                        } else {
                            break;
                        }
                    }
                    if path.is_empty() {
                        if let Some(id) = task.get_id() {
                            self.completed_task_ids.push(*id);
                        }
                        self.current_task = self.work_schedule.pop_front();
                    }
                }
            }
        } else {
            self.current_task = self.work_schedule.pop_front();
        }
    }

}


