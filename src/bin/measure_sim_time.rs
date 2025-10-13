use std::time::Instant;

use farmbotsim_rs::{
    cfg::{DEFAULT_AGENT_CONFIG_PATH, SCENE_CONFIGS_PATH},
    environment::env_module::{env::Env, env_config::EnvConfig},
    statistics::StatSummary,
    task_module::strategies::{ChargingStrategy, ChooseStationStrategy},
    units::duration::Duration,
};

fn main() {
    // Parameters
    let scene_config_path = format!("{SCENE_CONFIGS_PATH}1s_2s.json");
    let agent_config_path = DEFAULT_AGENT_CONFIG_PATH.to_string();
    let n_episodes = 1;
    let termination_duration = Duration::days(1.0);
    let number_agents = vec![1, 2, 3, 4, 5, 6];

    let charging_strategies = vec![
        ChargingStrategy::CriticalOnly(50.0),
        ChargingStrategy::CriticalOnly(70.0),
    ];

    let station_strategies = vec![
        ChooseStationStrategy::Manhattan(0.5),
        ChooseStationStrategy::Path(0.5),
    ];

    // Generate all combinations
    let mut combinations = vec![];
    let mut combination_i = 1;
    for charging_strategy in &charging_strategies {
        for station_strategy in &station_strategies {
            combinations.push((
                combination_i,
                charging_strategy.clone(),
                station_strategy.clone(),
            ));
            combination_i += 1;
        }
    }

    // Run and print results
    let results = run_combinations(
        scene_config_path.clone(),
        agent_config_path.clone(),
        n_episodes,
        termination_duration,
        &number_agents,
        &combinations,
    );

    println!("\n=== Real Time Stats per Agent Count ===");
    for result in results {
        println!(
            "Agents: {:<2} | Real-Time (s): min = {:>8.3}, avg = {:>8.3}, max = {:>8.3}",
            result.n_agents, result.real_time.min, result.real_time.avg, result.real_time.max
        );
    }
}

struct AEnvResult {
    real_time: StatSummary<f32>,
    n_agents: i32,
}

fn run_combinations(
    scene_config_path: String,
    agent_config_path: String,
    n_episodes: u32,
    termination_duration: Duration,
    number_agents: &[u32],
    combinations: &Vec<(usize, ChargingStrategy, ChooseStationStrategy)>,
) -> Vec<AEnvResult> {
    let mut results = vec![];

    for n_agents in number_agents {
        let mut all_episode_times: Vec<f32> = vec![];

        for (idx, charging_strategy, station_strategy) in combinations {
            println!(
                "→ Combination {}/{} | Agents: {}",
                idx,
                combinations.len(),
                n_agents
            );

            let env_config = EnvConfig {
                agent_config_path: agent_config_path.clone(),
                scene_config_path: scene_config_path.clone(),
                ..Default::default()
            };
            let mut env = Env::from_config(env_config.clone());
            env.n_agents = *n_agents;
            env.task_manager.charging_strategy = charging_strategy.clone();
            env.task_manager.choose_station_strategy = station_strategy.clone();

            for ep in 0..n_episodes {
                println!("  Episode {}/{}", ep + 1, n_episodes);
                env.reset();

                let start = Instant::now();
                while env.duration < termination_duration {
                    env.task_manager
                        .assign_tasks(&mut env.agents, &mut env.stations);
                    env.step();
                }
                let elapsed = start.elapsed().as_secs_f32();
                all_episode_times.push(elapsed);
                println!("    ⏱ {elapsed:.3} seconds");
            }
        }

        let min = all_episode_times
            .iter()
            .cloned()
            .fold(f32::INFINITY, f32::min);
        let max = all_episode_times
            .iter()
            .cloned()
            .fold(f32::NEG_INFINITY, f32::max);
        let avg = all_episode_times.iter().sum::<f32>() / all_episode_times.len() as f32;

        results.push(AEnvResult {
            real_time: StatSummary { min, avg, max },
            n_agents: *n_agents as i32,
        });
    }

    results
}

// === Real Time Stats per Agent Count ===
// Agents: 1  | Real-Time (s): min =    0.145, avg =    0.153, max =    0.162
// Agents: 2  | Real-Time (s): min =    0.275, avg =    0.283, max =    0.290
// Agents: 3  | Real-Time (s): min =    0.421, avg =    0.432, max =    0.448
// Agents: 4  | Real-Time (s): min =    0.446, avg =    0.500, max =    0.597
