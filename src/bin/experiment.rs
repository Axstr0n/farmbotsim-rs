use farmbotsim_rs::{
    cfg::{DEFAULT_AGENT_CONFIG_PATH, SCENE_CONFIGS_PATH},
    environment::env_module::{env::Env, env_config::EnvConfig},
    statistics::{AnalyzeEnvResult, Combination, ExperimentOutput, ExperimentParameters},
    task_module::strategies::{ChargingStrategy, ChooseStationStrategy},
    units::duration::Duration,
};

fn main() {
    // Parameters
    let scene_config_path = format!("{SCENE_CONFIGS_PATH}1s_2s.json").to_string();
    let agent_config_path = DEFAULT_AGENT_CONFIG_PATH.to_string();
    let n_episodes = 10;
    let number_agents = vec![1, 2, 3, 4, 5, 6];
    let charging_strategies = vec![
        ChargingStrategy::CriticalOnly(30.0),
        ChargingStrategy::CriticalOnly(40.0),
        ChargingStrategy::CriticalOnly(50.0),
        ChargingStrategy::CriticalOnly(60.0),
        ChargingStrategy::CriticalOnly(70.0),
        ChargingStrategy::ThresholdWithLimit(40.0, 30.0),
        ChargingStrategy::ThresholdWithLimit(50.0, 40.0),
        ChargingStrategy::ThresholdWithLimit(60.0, 50.0),
        ChargingStrategy::ThresholdWithLimit(70.0, 60.0),
    ];
    let mut station_strategies = vec![];
    let queue_weights = vec![
        0.0, // 0.25,
        0.5, // 0.75,
        1.0,
    ];
    for queue_weight in &queue_weights {
        station_strategies.push(ChooseStationStrategy::Manhattan(*queue_weight));
        station_strategies.push(ChooseStationStrategy::Path(*queue_weight));
    }

    // Get combinations
    let mut combinations = vec![];
    let mut combination_i = 1;
    for charging_strategy in &charging_strategies {
        for station_strategy in &station_strategies {
            combinations.push(Combination {
                label: format!("c{combination_i}"),
                charging_strategy: charging_strategy.clone(),
                station_strategy: station_strategy.clone(),
            });
            combination_i += 1;
        }
    }

    let results = run_combinations(
        scene_config_path.clone(),
        agent_config_path.clone(),
        n_episodes,
        &number_agents,
        &combinations,
    );

    let experiment_output = ExperimentOutput {
        parameters: ExperimentParameters {
            scene_config_path,
            agent_config_path,
            n_episodes: n_episodes as usize,
            number_agents,
            charging_strategies,
            station_strategies,
        },
        combinations,
        results,
    };

    // Write to JSON
    let json_path = "analyze/output.json";
    std::fs::write(
        json_path,
        serde_json::to_string_pretty(&experiment_output)
            .expect("Failed to serialize experiment output"),
    )
    .expect("Failed to write JSON file");

    println!("Experiment results written to {json_path}");
}

fn run_combinations(
    scene_config_path: String,
    agent_config_path: String,
    n_episodes: u32,
    number_agents: &[u32],
    combinations: &Vec<Combination>,
) -> Vec<AnalyzeEnvResult> {
    let termination_duration = Duration::days(1.0);
    let mut results = vec![];

    for n_agents in number_agents {
        for c in combinations {
            println!(
                "Combination: {}/{}, {n_agents}/{} agent",
                c.label,
                combinations.len(),
                number_agents.len()
            );
            let mut config_stats = vec![];
            let env_config = EnvConfig {
                agent_config_path: agent_config_path.clone(),
                scene_config_path: scene_config_path.clone(),
                ..Default::default()
            };
            let mut env = Env::from_config(env_config.clone());
            env.n_agents = *n_agents;
            env.task_manager.charging_strategy = c.charging_strategy.clone();
            env.task_manager.choose_station_strategy = c.station_strategy.clone();
            for i in 0..n_episodes {
                println!("Episode {}/{n_episodes}", i + 1);
                env.reset();
                while env.duration < termination_duration {
                    env.task_manager
                        .assign_tasks(&mut env.agents, &mut env.stations);
                    env.step();
                }
                let episode_stats = env.get_env_episode_stats();
                config_stats.push(episode_stats);
            }
            let env_result = AnalyzeEnvResult::from_episodes(*n_agents, c.clone(), config_stats);
            results.push(env_result);
        }
    }
    results
}
