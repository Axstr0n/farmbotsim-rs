use egui::{Color32, Pos2, Vec2};

use super::agent_state::AgentState;
use super::battery::BatteryPack;
use super::movement::{Movement, RombaMovement};
use super::work_schedule::WorkSchedule;
use crate::task::task::Task;
use crate::utilities::datetime::DateTimeManager;
use crate::utilities::pos2::ExtendedPos2;
use crate::utilities::utils::TOLERANCE_DISTANCE;


#[derive(Clone, Debug, PartialEq)]
pub struct Agent {
    pub id: u32,
    pub position: Pos2,
    pub direction: Vec2,
    pub movement: RombaMovement,
    pub velocity_lin: f32,
    pub velocity_ang: f32,
    pub color: Color32,
    pub spawn_position: Pos2,

    pub work_schedule: WorkSchedule,
    pub current_task: Option<Task>,
    pub completed_task_ids: Vec<u32>, // for storing so task manager can know

    pub state: AgentState,
    pub battery: BatteryPack,
}

impl Agent {
    pub fn new(id: u32, 
        position: Pos2,
        direction: Vec2,
        movement: RombaMovement,
        color: Color32) -> Self {
        Self {
            id,
            position,
            direction,
            movement,
            velocity_lin: 0.0,
            velocity_ang: 0.0,
            color,
            spawn_position: position,

            work_schedule: WorkSchedule::new(),
            current_task: None,
            completed_task_ids: vec![],

            state: AgentState::Wait,
            battery: BatteryPack::from_config("battery1".to_string(), Some(61.0)),
        }
    }
    pub fn update(&mut self, simulation_step: u32, date_time_manager: &DateTimeManager) {
        if self.state == AgentState::Discharged { return }
        self.update_state(date_time_manager);

        self.update_task_and_path();
        let inputs = self._get_inputs();
        self._move(simulation_step, inputs);
    }

    fn update_state(&mut self, date_time_manager: &DateTimeManager) {
        let mut current_state = std::mem::replace(&mut self.state, AgentState::Wait); // placeholder

        let maybe_new_state = current_state.update(self, date_time_manager);

        if let Some(mut new_state) = maybe_new_state {
            current_state.on_exit(self);
            new_state.on_enter(self);
            self.state = new_state;
        } else {
            self.state = current_state;
        }
    }
    
    fn _move(&mut self, simulation_step: u32, inputs: Vec<f32>) {
        let current_task_velocity = self.current_task.as_ref().map(|task| *task.get_velocity()).unwrap_or_default();
        let (new_position, new_direction, new_velocity_l, new_velocity_r) = self.movement.calculate_new_pose_from_inputs(
            simulation_step, inputs, self.position, self.direction, current_task_velocity
        );
    
        // Now assign the new values
        self.position = new_position;
        self.direction = new_direction;
        self.velocity_lin = new_velocity_l;
        self.velocity_ang = new_velocity_r;
    }
    
    fn _get_inputs(&self) -> Vec<f32> {
        let next_direction: Option<Vec2> = None;

        let next_position = match &self.current_task {
            Some(task) => {
                if !task.get_path().is_empty() {
                    // Follow path normally
                    task.get_path()[0]
                } else {
                    self.position
                }
                
            },
            _ => {
                self.position
            }
        };
        self.movement.calculate_inputs_for_target(
            self.position, self.direction, next_position, next_direction
        )
    }
    
    fn update_task_and_path(&mut self) {
        if let Some(task) = &mut self.current_task {
            match task {
                Task::Stationary { pos, duration, .. } => {
                    if self.position.is_close_to(*pos, TOLERANCE_DISTANCE) {
                        if *duration > 0 {
                            *duration -= 1;
                        } else if let Some(id) = task.get_id() {
                            self.completed_task_ids.push(*id);
                            self.current_task = self.work_schedule.pop_front();
                        }
                    }
                }
                Task::WaitDuration { pos, duration , ..} => {
                    if self.position.is_close_to(*pos, TOLERANCE_DISTANCE) {
                        if *duration > 0 {
                            *duration -= 1;
                        } else {
                            self.current_task = self.work_schedule.pop_front();
                        }
                    }
                }
                Task::WaitInfinite { .. } => { }
                Task::Moving { path, .. } |
                Task::Travel { path, .. } => {
                    while !path.is_empty() {
                        if self.position.is_close_to(path[0], TOLERANCE_DISTANCE) {
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


