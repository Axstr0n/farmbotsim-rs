use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    agent_module::{agent::AgentId, agent_state::AgentState},
    environment::env_module::env_config::EnvConfig,
    logger::log_error_and_panic,
    movement_module::pose::Pose,
    task_module::{
        strategies::{ChargingStrategy, ChooseStationStrategy},
        task::Task,
    },
    units::{duration::Duration, energy::Energy, length::Length},
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
    pub discharged_time: Duration,

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
        let mut discharged_time = Duration::ZERO;

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
                Discharged => discharged_time = discharged_time + step.duration,
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
                distance_travelled =
                    distance_travelled + Length::meters(prev.position.distance(step.pose.position));
            }
            prev_pose = Some(step.pose.clone());
        }

        Self {
            work_time,
            travel_time,
            idle_time,
            charging_time,
            queue_time,
            discharged_time,
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
                agents_map
                    .entry(*agent_id)
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
        let mut total_discharged_time = Duration::ZERO;
        let mut total_energy_charged = Energy::ZERO;
        let mut total_energy_discharged = Energy::ZERO;
        let mut total_distance_travelled = Length::ZERO;

        for stats in &all_agent_stats {
            total_work_time = total_work_time + stats.work_time;
            total_travel_time = total_travel_time + stats.travel_time;
            total_idle_time = total_idle_time + stats.idle_time;
            total_charging_time = total_charging_time + stats.charging_time;
            total_queue_time = total_queue_time + stats.queue_time;
            total_discharged_time = total_discharged_time + stats.discharged_time;
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
            discharged_time: total_discharged_time,
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
    }
}

/// Result of a performance matrix evaluation.
#[derive(Debug, Serialize, Deserialize)]
pub struct PerformanceMatrixResult {
    pub start_datetime: chrono::DateTime<chrono::Local>,
    pub evaluation_duration: std::time::Duration,
    pub n_episodes: usize,
    pub scene_config_path: String,
    pub env_results: Vec<EnvResult>,
}

//----- For non app analysis -------------
// Binaries: experiment, analyze

#[derive(Debug, Serialize, Deserialize)]
pub struct ExperimentOutput {
    pub parameters: ExperimentParameters,
    pub combinations: Vec<Combination>,
    pub results: Vec<AnalyzeEnvResult>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExperimentParameters {
    pub scene_config_path: String,
    pub agent_config_path: String,
    pub n_episodes: usize,
    pub number_agents: Vec<u32>,
    pub charging_strategies: Vec<ChargingStrategy>,
    pub station_strategies: Vec<ChooseStationStrategy>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Combination {
    pub label: String,
    pub charging_strategy: ChargingStrategy,
    pub station_strategy: ChooseStationStrategy,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalyzeEnvResult {
    pub n_episodes: u32,
    pub n_agents: u32,
    pub combination: Combination,
    pub n_completed_tasks: StatSummary<f32>,
    pub env_duration: StatSummary<Duration>,
    pub agents: HashMap<AgentId, AgentResultSummary>,
    pub agent_averaged_stats: AgentEpisodeStats, // all agents combine stats for avg episode per agent
    pub agent_totaled_stats: AgentEpisodeStats,  // all agents combine stats for avg episode
}
impl AnalyzeEnvResult {
    /// Aggregates statistics across multiple env episodes.
    pub fn from_episodes(
        n_agents: u32,
        combination: Combination,
        episodes: Vec<EnvEpisodeStats>,
    ) -> Self {
        let n_episodes = episodes.len() as u32;

        // Compute min/avg/max for top-level env stats
        let vec_n_completed_tasks: Vec<f32> = episodes
            .iter()
            .map(|e| e.n_completed_tasks as f32)
            .collect();
        println!("T: {vec_n_completed_tasks:?}");

        let n_completed_tasks = summarize(episodes.iter().map(|e| e.n_completed_tasks as f32));
        let env_duration = summarize(episodes.iter().map(|e| e.env_duration));
        println!("S: {n_completed_tasks:?}");
        println!();
        // Aggregate agent stats
        let mut agents_map: HashMap<AgentId, Vec<AgentEpisodeStats>> = HashMap::new();

        for episode in &episodes {
            for (agent_id, agent_stats) in &episode.agents {
                agents_map
                    .entry(*agent_id)
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
        let mut total_discharged_time = Duration::ZERO;
        let mut total_energy_charged = Energy::ZERO;
        let mut total_energy_discharged = Energy::ZERO;
        let mut total_distance_travelled = Length::ZERO;

        for stats in &all_agent_stats {
            total_work_time = total_work_time + stats.work_time;
            total_travel_time = total_travel_time + stats.travel_time;
            total_idle_time = total_idle_time + stats.idle_time;
            total_charging_time = total_charging_time + stats.charging_time;
            total_queue_time = total_queue_time + stats.queue_time;
            total_discharged_time = total_discharged_time + stats.discharged_time;
            total_energy_charged = total_energy_charged + stats.energy_charged;
            total_energy_discharged = total_energy_discharged + stats.energy_discharged;
            total_distance_travelled = total_distance_travelled + stats.distance_travelled;
        }
        // let vec_travel: Vec<f32> = all_agent_stats
        //     .iter()
        //     .map(|s| s.distance_travelled.to_base_unit())
        //     .collect();
        // println!("Travel: {vec_travel:?}");

        // Avg per episode
        let avg_work_time = total_work_time / n_episodes as f32;
        let avg_travel_time = total_travel_time / n_episodes as f32;
        let avg_idle_time = total_idle_time / n_episodes as f32;
        let avg_charging_time = total_charging_time / n_episodes as f32;
        let avg_queue_time = total_queue_time / n_episodes as f32;
        let avg_discharged_time = total_discharged_time / n_episodes as f32;
        let avg_energy_charged = total_energy_charged / n_episodes as f32;
        let avg_energy_discharged = total_energy_discharged / n_episodes as f32;
        let avg_distance_travelled = total_distance_travelled / n_episodes as f32;

        let agent_totaled_stats = AgentEpisodeStats {
            work_time: avg_work_time,
            travel_time: avg_travel_time,
            idle_time: avg_idle_time,
            charging_time: avg_charging_time,
            queue_time: avg_queue_time,
            discharged_time: avg_discharged_time,
            energy_charged: avg_energy_charged,
            energy_discharged: avg_energy_discharged,
            distance_travelled: avg_distance_travelled,
        };

        let agent_averaged_stats = AgentEpisodeStats {
            work_time: avg_work_time / n_agents as f32,
            travel_time: avg_travel_time / n_agents as f32,
            idle_time: avg_idle_time / n_agents as f32,
            charging_time: avg_charging_time / n_agents as f32,
            queue_time: avg_queue_time / n_agents as f32,
            discharged_time: avg_discharged_time / n_agents as f32,
            energy_charged: avg_energy_charged / n_agents as f32,
            energy_discharged: avg_energy_discharged / n_agents as f32,
            distance_travelled: avg_distance_travelled / n_agents as f32,
        };

        Self {
            n_episodes,
            n_agents,
            combination,
            n_completed_tasks,
            env_duration,
            agents,
            agent_averaged_stats,
            agent_totaled_stats,
        }
    }
}
