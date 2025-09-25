use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::{
    agent_module::{agent::AgentId, agent_state::AgentState},
    environment::env_module::env_config::EnvConfig,
    logger::log_error_and_panic,
    movement_module::pose::Pose,
    task_module::task::Task,
    units::{duration::Duration, energy::Energy, length::Length}
};

// ---------- Single timestep ----------

/// Represents the state of an agent at a single timestep in the environment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentTimestep {
    pub duration: Duration,
    pub state: AgentState,
    pub pose: Pose,
    pub battery_energy: Energy,
    pub task: Option<Task>,
}

// ---------- Single Episode ----------

/// Contains aggregated statistics for a single agent over one episode.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AgentEpisodeStats {
    pub work_time: Duration,
    pub travel_time: Duration,
    pub idle_time: Duration,
    pub charging_time: Duration,
    pub queue_time: Duration,

    pub energy_charged: Energy,
    pub energy_discharged: Energy,
    pub distance_travelled: Length,
}
impl AgentEpisodeStats {
    /// Creates an `AgentEpisodeStats` from a slice of `AgentTimestep`s.
    pub fn from_timesteps(timesteps: &[AgentTimestep]) -> Self {
        use crate::agent_module::agent_state::AgentState::*;

        let mut work_time = Duration::ZERO;
        let mut travel_time = Duration::ZERO;
        let mut idle_time = Duration::ZERO;
        let mut charging_time = Duration::ZERO;
        let mut queue_time = Duration::ZERO;

        let mut energy_charged = Energy::ZERO;
        let mut energy_discharged = Energy::ZERO;
        let mut distance_travelled = Length::ZERO;

        let mut prev_pose: Option<Pose> = None;
        let mut prev_battery: Option<Energy> = None;

        for step in timesteps {
            // Accumulate durations based on state and task intent
            match step.state {
                Work => work_time = work_time + step.duration,
                Travel => travel_time = travel_time + step.duration,
                Charging => charging_time = charging_time + step.duration,
                Wait => {
                    // Check if waiting in a queue
                    if let Some(task) = &step.task {
                        if *task.get_intent() == crate::task_module::task::Intent::Queue {
                            queue_time = queue_time + step.duration;
                        } else {
                            idle_time = idle_time + step.duration;
                        }
                    } else {
                        idle_time = idle_time + step.duration;
                    }
                }
                Discharged => {} // do nothing
            }

            // Compute energy delta
            if let Some(prev) = prev_battery {
                let delta = step.battery_energy - prev;
                if delta > Energy::ZERO {
                    energy_charged = energy_charged + delta;
                } else {
                    energy_discharged = energy_discharged - (delta);
                }
            }
            prev_battery = Some(step.battery_energy);

            // Compute distance travelled
            if let Some(prev) = prev_pose {
                distance_travelled = distance_travelled + Length::meters(prev.position.distance(step.pose.position));
            }
            prev_pose = Some(step.pose.clone());
        }

        Self {
            work_time,
            travel_time,
            idle_time,
            charging_time,
            queue_time,
            energy_charged,
            energy_discharged,
            distance_travelled,
        }
    }
}

/// Contains statistics for an environment episode, including all agents.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EnvEpisodeStats {
    pub n_completed_tasks: u32,
    pub env_duration: Duration,
    pub agents: HashMap<AgentId, AgentEpisodeStats>,
}

// ---------- Aggregated Types ----------

/// Represents a min/average/max summary of a set of values.
#[derive(Debug, Serialize, Deserialize)]
pub struct StatSummary<T> {
    pub min: T,
    pub avg: T,
    pub max: T,
}

/// Computes a `StatSummary` (min, avg, max) over an iterator of values.
pub fn summarize<T, I>(values: I) -> StatSummary<T>
where
    T: Copy + PartialOrd + std::ops::Add<Output = T> + std::ops::Div<f32, Output = T>,
    I: Iterator<Item = T>,
{
    let mut count = 0f32;
    let mut sum: Option<T> = None;
    let mut min: Option<T> = None;
    let mut max: Option<T> = None;

    for v in values {
        count += 1.0;
        sum = Some(match sum {
            Some(s) => s + v,
            None => v,
        });
        min = Some(match min {
            Some(m) if m < v => m,
            _ => v,
        });
        max = Some(match max {
            Some(m) if m > v => m,
            _ => v,
        });
    }

    let min = min.unwrap_or_else(|| {
        let msg = "No min in empty iterator.";
        log_error_and_panic(msg);
    });
    let max = max.unwrap_or_else(|| {
        let msg = "No max in empty iterator.";
        log_error_and_panic(msg);
    });
    let avg = if let Some(s) = sum {
        s / count
    } else {
        let msg = "Cannot summarize empty iterator";
        log_error_and_panic(msg);
    };

    StatSummary { min, avg, max }
}


/// Aggregated agent stats across multiple episodes
#[derive(Debug, Serialize, Deserialize)]
pub struct AgentResultSummary {
    pub work_time: StatSummary<Duration>,
    pub travel_time: StatSummary<Duration>,
    pub idle_time: StatSummary<Duration>,
    pub charging_time: StatSummary<Duration>,
    pub queue_time: StatSummary<Duration>,

    pub energy_charged: StatSummary<Energy>,
    pub energy_discharged: StatSummary<Energy>,
    pub distance_travelled: StatSummary<Length>,
}
impl AgentResultSummary {
    pub fn from_episodes(stats: &[AgentEpisodeStats]) -> Self {
        Self {
            work_time: summarize(stats.iter().map(|s| s.work_time)),
            travel_time: summarize(stats.iter().map(|s| s.travel_time)),
            idle_time: summarize(stats.iter().map(|s| s.idle_time)),
            charging_time: summarize(stats.iter().map(|s| s.charging_time)),
            queue_time: summarize(stats.iter().map(|s| s.queue_time)),

            energy_charged: summarize(stats.iter().map(|s| s.energy_charged)),
            energy_discharged: summarize(stats.iter().map(|s| s.energy_discharged)),
            distance_travelled: summarize(stats.iter().map(|s| s.distance_travelled)),
        }
    }
}

/// Aggregated results across multiple episodes
#[derive(Debug, Serialize, Deserialize)]
pub struct EnvResult {
    pub n_episodes: u32,
    pub env_config: EnvConfig,
    pub n_completed_tasks: StatSummary<f32>,
    pub env_duration: StatSummary<Duration>,
    pub agents: HashMap<AgentId, AgentResultSummary>,
    pub combined_agents: AgentEpisodeStats,
}
impl EnvResult {
    /// Aggregates statistics across multiple env episodes.
    pub fn from_episodes(env_config: EnvConfig, episodes: Vec<EnvEpisodeStats>) -> Self {
        let n_episodes = episodes.len() as u32;

        // Compute min/avg/max for top-level env stats
        let n_completed_tasks = summarize(episodes.iter().map(|e| e.n_completed_tasks as f32));
        let env_duration = summarize(episodes.iter().map(|e| e.env_duration));

        // Aggregate agent stats
        let mut agents_map: HashMap<AgentId, Vec<AgentEpisodeStats>> = HashMap::new();

        for episode in &episodes {
            for (agent_id, agent_stats) in &episode.agents {
                agents_map.entry(*agent_id)
                    .or_default()
                    .push(agent_stats.clone());
            }
        }

        let agents: HashMap<AgentId, AgentResultSummary> = agents_map
            .clone()
            .into_iter()
            .map(|(id, stats)| (id, AgentResultSummary::from_episodes(&stats)))
            .collect();

        // Combine all agent stats across all episodes
        let mut all_agent_stats: Vec<AgentEpisodeStats> = Vec::new();
        for stats in agents_map.clone().values() {
            all_agent_stats.extend_from_slice(stats);
        }
        // Compute summed totals
        let mut total_work_time = Duration::ZERO;
        let mut total_travel_time = Duration::ZERO;
        let mut total_idle_time = Duration::ZERO;
        let mut total_charging_time = Duration::ZERO;
        let mut total_queue_time = Duration::ZERO;
        let mut total_energy_charged = Energy::ZERO;
        let mut total_energy_discharged = Energy::ZERO;
        let mut total_distance_travelled = Length::ZERO;

        for stats in &all_agent_stats {
            total_work_time = total_work_time + stats.work_time;
            total_travel_time = total_travel_time + stats.travel_time;
            total_idle_time = total_idle_time + stats.idle_time;
            total_charging_time = total_charging_time + stats.charging_time;
            total_queue_time = total_queue_time + stats.queue_time;
            total_energy_charged = total_energy_charged + stats.energy_charged;
            total_energy_discharged = total_energy_discharged + stats.energy_discharged;
            total_distance_travelled = total_distance_travelled + stats.distance_travelled;
        }

        let combined_agents = AgentEpisodeStats {
            work_time: total_work_time,
            travel_time: total_travel_time,
            idle_time: total_idle_time,
            charging_time: total_charging_time,
            queue_time: total_queue_time,
            energy_charged: total_energy_charged,
            energy_discharged: total_energy_discharged,
            distance_travelled: total_distance_travelled,
        };

        Self {
            n_episodes,
            env_config,
            n_completed_tasks,
            env_duration,
            agents,
            combined_agents,
        }

        // Self {
        //     n_episodes,
        //     env_config,
        //     n_completed_tasks,
        //     env_duration,
        //     agents,
        // }
    }
}

/// Result of a performance matrix evaluation.
#[derive(Debug, Serialize, Deserialize)]
pub struct PerformanceMatrixResult  {
    pub start_datetime: chrono::DateTime<chrono::Local>,
    pub evaluation_duration: std::time::Duration,
    pub n_episodes: usize,
    pub scene_config_path: String,
    pub env_results: Vec<EnvResult>,
}
