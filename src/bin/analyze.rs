use plotters::prelude::*;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;

use farmbotsim_rs::statistics::{AnalyzeEnvResult, Combination, ExperimentOutput};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json_path = "analyze/output.json";
    let data = fs::read_to_string(json_path)?;
    let experiment_output: ExperimentOutput = serde_json::from_str(&data)?;

    let combinations = &experiment_output.combinations;
    let results = &experiment_output.results;

    write_combinations_latex("analyze/combinations.tex", combinations)?;

    let best_combinations = select_best_combination(results);
    println!("{best_combinations:?}");

    // Plots: n_agents-line, x-combinations
    {
        // Tasks completed
        plot_metric(
            "analyze/plots/task_completed.png",
            combinations,
            results,
            "Število opravljenih nalog",
            |r| r.n_completed_tasks.avg,
        )?;

        // Work time
        plot_metric_pair(
            "work_time",
            combinations,
            results,
            "Čas dela [s]",
            |r| r.agent_averaged_stats.work_time.to_base_unit(),
            |r| r.agent_totaled_stats.work_time.to_base_unit(),
        )?;

        // Travel time
        plot_metric_pair(
            "travel_time",
            combinations,
            results,
            "Čas vožnje [s]",
            |r| r.agent_averaged_stats.travel_time.to_base_unit(),
            |r| r.agent_totaled_stats.travel_time.to_base_unit(),
        )?;

        // Idle time
        plot_metric_pair(
            "idle_time",
            combinations,
            results,
            "Čas mirovanja [s]",
            |r| r.agent_averaged_stats.idle_time.to_base_unit(),
            |r| r.agent_totaled_stats.idle_time.to_base_unit(),
        )?;

        // Queue time
        plot_metric_pair(
            "queue_time",
            combinations,
            results,
            "Čas čakanja v čakalni vrsti [s]",
            |r| r.agent_averaged_stats.queue_time.to_base_unit(),
            |r| r.agent_totaled_stats.queue_time.to_base_unit(),
        )?;

        // Discharged time
        plot_metric_pair(
            "discharged_time",
            combinations,
            results,
            "Čas spraznjenosti [s]",
            |r| r.agent_averaged_stats.discharged_time.to_base_unit(),
            |r| r.agent_totaled_stats.discharged_time.to_base_unit(),
        )?;

        // Charging time
        plot_metric_pair(
            "charging_time",
            combinations,
            results,
            "Čas polnjenja [s]",
            |r| r.agent_averaged_stats.charging_time.to_base_unit(),
            |r| r.agent_totaled_stats.charging_time.to_base_unit(),
        )?;

        // Distance travelled
        plot_metric_pair(
            "distance_travelled",
            combinations,
            results,
            "Prepotovana pot [m]",
            |r| r.agent_averaged_stats.distance_travelled.to_base_unit(),
            |r| r.agent_totaled_stats.distance_travelled.to_base_unit(),
        )?;

        // Energy charged
        plot_metric_pair(
            "energy_charged",
            combinations,
            results,
            "Energija polnjenja [J]",
            |r| r.agent_averaged_stats.energy_charged.to_base_unit(),
            |r| r.agent_totaled_stats.energy_charged.to_base_unit(),
        )?;

        // Energy discharged
        plot_metric_pair(
            "energy_discharged",
            combinations,
            results,
            "Energija porabljena [J]",
            |r| r.agent_averaged_stats.energy_discharged.to_base_unit(),
            |r| r.agent_totaled_stats.energy_discharged.to_base_unit(),
        )?;
    }

    plot_best_combinations_time(
        "analyze/plots/time_segmentation.png",
        best_combinations.clone(),
        results,
    )?;
    plot_best_combinations_percentage(
        "analyze/plots/time_segmentation_percentage.png",
        best_combinations.clone(),
        results,
    )?;
    Ok(())
}

/// For getting same combination for latex document
fn write_combinations_latex(
    file_path: &str,
    combinations: &Vec<Combination>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(file_path)?;
    let caption = "Kombinacije strategij";
    let label = "tab:kombinacije_strategij";

    // Begin table environment
    writeln!(file, "\\begin{{table}}[H]")?;
    writeln!(file, "\\caption{{{}}}", caption)?;
    writeln!(file, "\\label{{{}}}", label)?;
    writeln!(file, "\\centering")?;

    writeln!(file, "\\begin{{tabular}}{{|c|c|c|}}")?;
    writeln!(file, "\\hline")?;
    writeln!(
        file,
        "Oznaka & Strategija polnjenja & Strategija izbire postaje \\\\"
    )?;
    writeln!(file, "\\hline")?;

    for c in combinations {
        let label = c.label.clone();
        let charging_str = format!("{}", c.charging_strategy);
        let station_str = format!("{}", c.station_strategy);

        writeln!(file, "{label} & {charging_str} & {station_str} \\\\")?;
        writeln!(file, "\\hline")?;
    }

    writeln!(file, "\\end{{tabular}}")?;
    writeln!(file, "\\end{{table}}")?;
    Ok(())
}

fn select_best_combination(results: &[AnalyzeEnvResult]) -> HashMap<u32, Combination> {
    let mut best_per_agents: HashMap<u32, Combination> = HashMap::new();

    // Group results by n_agents
    let mut results_by_agents: HashMap<u32, Vec<&AnalyzeEnvResult>> = HashMap::new();
    for r in results {
        results_by_agents.entry(r.n_agents).or_default().push(r);
    }

    // Select best combination for each n_agents
    for (&n_agents, group) in &results_by_agents {
        if let Some(best) = group
            .iter()
            .max_by(|a, b| score(a).partial_cmp(&score(b)).unwrap())
        {
            best_per_agents.insert(n_agents, best.combination.clone());
        }
    }

    best_per_agents
}

fn score(r: &AnalyzeEnvResult) -> f32 {
    let w_work_time = 5.0;
    let w_travel_time = 0.5;
    let w_tasks = 2.0;
    let w_queue = 0.2;
    let w_discharged = 10.0;

    let work_time = r.agent_averaged_stats.work_time.to_base_unit();
    let travel_time = r.agent_averaged_stats.travel_time.to_base_unit();
    let tasks_completed = r.n_completed_tasks.avg;
    let queue = r.agent_averaged_stats.queue_time.to_base_unit();
    let discharged = r.agent_averaged_stats.discharged_time.to_base_unit();

    w_work_time * work_time // maximize
    - w_travel_time * travel_time // minimize
    + w_tasks * tasks_completed // maximize
    - w_queue * queue // minimize
    - w_discharged * discharged // minimize
}

fn plot_metric<F>(
    png_path: &str,
    combinations: &[Combination],
    results: &[AnalyzeEnvResult],
    y_label: &str,
    metric_fn: F,
) -> Result<(), Box<dyn std::error::Error>>
where
    F: Fn(&AnalyzeEnvResult) -> f32,
{
    #[derive(Debug)]
    struct PlotPoint {
        combination_label: String,
        n_agents: u32,
        value: f32,
    }

    // Flatten results into PlotPoint structs
    let plot_data: Vec<PlotPoint> = results
        .iter()
        .map(|r| PlotPoint {
            combination_label: r.combination.label.clone(),
            n_agents: r.n_agents,
            value: metric_fn(r),
        })
        .collect();

    // Unique agent counts
    let mut agent_counts: Vec<u32> = plot_data
        .iter()
        .map(|p| p.n_agents)
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();
    agent_counts.sort();

    // Map combination labels to x-axis indices
    let label_to_idx: std::collections::HashMap<String, i32> = combinations
        .iter()
        .enumerate()
        .map(|(i, c)| (c.label.clone(), i as i32))
        .collect();

    let x_len = combinations.len() as i32;

    // Create drawing area
    let root = BitMapBackend::new(png_path, (600, 400)).into_drawing_area();
    root.fill(&WHITE)?;

    let y_min = plot_data.iter().map(|p| p.value).fold(f32::MAX, f32::min);
    let y_max = plot_data.iter().map(|p| p.value).fold(f32::MIN, f32::max);

    let mut chart = ChartBuilder::on(&root)
        .margin(10)
        .margin_left(30)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(-1..x_len, (y_min * 0.9)..(y_max * 1.1))?;

    chart
        .configure_mesh()
        .y_max_light_lines(3)
        .x_labels(combinations.len() + 1)
        .x_label_formatter(&|x| {
            let idx = *x as usize;
            if idx < combinations.len() {
                combinations[idx].label.clone()
            } else {
                "".to_string()
            }
        })
        .x_desc("Kombinacija strategij")
        .y_desc(y_label)
        .draw()?;

    use plotters::style::Palette99;

    // Draw lines for each agent count
    for (i, &n_agents) in agent_counts.iter().enumerate() {
        let color = Palette99::pick(i);

        let mut points: Vec<(i32, f32)> = plot_data
            .iter()
            .filter(|p| p.n_agents == n_agents)
            .map(|p| {
                let x = *label_to_idx.get(&p.combination_label).unwrap();
                (x, p.value)
            })
            .collect();

        points.sort_by_key(|(x, _)| *x);

        if !points.is_empty() {
            chart
                .draw_series(LineSeries::new(points.iter().map(|&(x, y)| (x, y)), &color))?
                .label(format!("{n_agents} agents"))
                .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &color));

            let point_color = Palette99::pick(i);
            chart.draw_series(
                points
                    .iter()
                    .map(|&(x, y)| Circle::new((x, y), 3, point_color.filled())),
            )?;
        }
    }

    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .draw()?;

    Ok(())
}

fn plot_metric_pair<F1, F2>(
    base_name: &str,
    combinations: &[Combination],
    results: &[AnalyzeEnvResult],
    y_label: &str,
    avg_metric: F1,
    total_metric: F2,
) -> Result<(), Box<dyn std::error::Error>>
where
    F1: Fn(&AnalyzeEnvResult) -> f32,
    F2: Fn(&AnalyzeEnvResult) -> f32,
{
    plot_metric(
        &format!("analyze/plots/{base_name}_avg.png"),
        combinations,
        results,
        y_label,
        avg_metric,
    )?;
    plot_metric(
        &format!("analyze/plots/{base_name}_total.png"),
        combinations,
        results,
        y_label,
        total_metric,
    )?;

    Ok(())
}

fn plot_best_combinations_time(
    png_path: &str,
    best_combinations: HashMap<u32, Combination>,
    results: &[AnalyzeEnvResult],
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(png_path, (800, 500)).into_drawing_area();
    root.fill(&WHITE)?;

    // Sort agent counts
    let mut agent_counts: Vec<u32> = best_combinations.keys().copied().collect();
    agent_counts.sort();

    // Find max total time
    let mut max_total_time = 0.0;
    for n_agents in &agent_counts {
        if let Some(best_combo) = best_combinations.get(n_agents) {
            if let Some(res) = results
                .iter()
                .find(|r| r.n_agents == *n_agents && r.combination.label == best_combo.label)
            {
                let total = res.agent_averaged_stats.work_time.to_base_unit()
                    + res.agent_averaged_stats.travel_time.to_base_unit()
                    + res.agent_averaged_stats.charging_time.to_base_unit()
                    + res.agent_averaged_stats.queue_time.to_base_unit()
                    + res.agent_averaged_stats.discharged_time.to_base_unit()
                    + res.agent_averaged_stats.idle_time.to_base_unit();
                println!("Total: {total}");
                if total > max_total_time {
                    max_total_time = total;
                }
            }
        }
    }

    // Build chart
    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .x_label_area_size(50)
        .y_label_area_size(70)
        .build_cartesian_2d(
            -0.5f32..(agent_counts.len() as f32 - 0.5),
            0f32..(max_total_time * 1.1),
        )?;

    chart
        .configure_mesh()
        .disable_mesh()
        .x_labels(agent_counts.len())
        .x_label_formatter(&|idx: &f32| {
            let i = idx.round() as usize;
            agent_counts
                .get(i)
                .map(|v| v.to_string())
                .unwrap_or_default()
        })
        .y_desc("Time [s]")
        .x_desc("Number of Agents")
        .draw()?;

    let bar_width = 0.5;
    let mut added_labels = std::collections::HashSet::new();

    // Draw bars
    for (idx, n_agents) in agent_counts.iter().enumerate() {
        let idx = idx as i32;
        if let Some(best_combo) = best_combinations.get(n_agents) {
            if let Some(res) = results
                .iter()
                .find(|r| r.n_agents == *n_agents && r.combination.label == best_combo.label)
            {
                let segments = vec![
                    (
                        "Work",
                        res.agent_averaged_stats.work_time.to_base_unit(),
                        &BLUE,
                    ),
                    (
                        "Travel",
                        res.agent_averaged_stats.travel_time.to_base_unit(),
                        &GREEN,
                    ),
                    (
                        "Charging",
                        res.agent_averaged_stats.charging_time.to_base_unit(),
                        &RED,
                    ),
                    (
                        "Queue",
                        res.agent_averaged_stats.queue_time.to_base_unit(),
                        &YELLOW,
                    ),
                    (
                        "Idle",
                        res.agent_averaged_stats.idle_time.to_base_unit(),
                        &MAGENTA,
                    ),
                    (
                        "Discharged",
                        res.agent_averaged_stats.discharged_time.to_base_unit(),
                        &CYAN,
                    ),
                ];

                let mut start = 0.0;
                let idx = idx as f32;
                for (label, value, color) in segments {
                    let rect = Rectangle::new(
                        [
                            (idx - bar_width / 2.0, start),
                            (idx + bar_width / 2.0, start + value),
                        ],
                        color.filled(),
                    );

                    if added_labels.insert(label) {
                        // First time we see this label → add to legend
                        chart
                            .draw_series(std::iter::once(rect))?
                            .label(label)
                            .legend(move |(x, y)| {
                                Rectangle::new([(x, y - 5), (x + 10, y + 5)], color.filled())
                            });
                    } else {
                        // Already added → just draw without legend
                        chart.draw_series(std::iter::once(rect))?;
                    }

                    start += value;
                }
                let combo_label = &best_combo.label;
                chart.draw_series(std::iter::once(Text::new(
                    combo_label.clone(),
                    (idx, start + max_total_time * 0.05), // place above bar
                    ("sans-serif", 15).into_font().color(&BLACK),
                )))?;
            }
        }
    }

    // Add legend
    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .draw()?;

    Ok(())
}

fn plot_best_combinations_percentage(
    png_path: &str,
    best_combinations: HashMap<u32, Combination>,
    results: &[AnalyzeEnvResult],
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(png_path, (600, 400)).into_drawing_area();
    root.fill(&WHITE)?;

    // Sort agent counts
    let mut agent_counts: Vec<u32> = best_combinations.keys().copied().collect();
    agent_counts.sort();

    // Build chart with 0-100% y axis
    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .x_label_area_size(50)
        .y_label_area_size(70)
        .build_cartesian_2d(
            -0.5f32..(agent_counts.len() as f32 - 0.5 + 1.0),
            0f32..100.0,
        )?;

    chart
        .configure_mesh()
        .disable_mesh()
        .x_labels(agent_counts.len() + 1)
        .x_label_formatter(&|idx: &f32| {
            let i = idx.round() as usize;
            agent_counts
                .get(i)
                .map(|v| v.to_string())
                .unwrap_or_default()
        })
        .y_desc("Porabljen čas [%]")
        .x_desc("Število agentov")
        .draw()?;

    let bar_width = 0.5;
    let mut added_labels = std::collections::HashSet::new();

    // Draw normalized stacked bars
    for (idx, n_agents) in agent_counts.iter().enumerate() {
        let idx = idx as f32;
        if let Some(best_combo) = best_combinations.get(n_agents) {
            if let Some(res) = results
                .iter()
                .find(|r| r.n_agents == *n_agents && r.combination.label == best_combo.label)
            {
                // Compute total time
                let total_time = res.agent_averaged_stats.work_time.to_base_unit()
                    + res.agent_averaged_stats.travel_time.to_base_unit()
                    + res.agent_averaged_stats.charging_time.to_base_unit()
                    + res.agent_averaged_stats.queue_time.to_base_unit()
                    + res.agent_averaged_stats.idle_time.to_base_unit()
                    + res.agent_averaged_stats.discharged_time.to_base_unit();

                if total_time == 0.0 {
                    continue;
                }

                let segments = vec![
                    (
                        "Work",
                        res.agent_averaged_stats.work_time.to_base_unit(),
                        &BLUE,
                    ),
                    (
                        "Travel",
                        res.agent_averaged_stats.travel_time.to_base_unit(),
                        &GREEN,
                    ),
                    (
                        "Charging",
                        res.agent_averaged_stats.charging_time.to_base_unit(),
                        &RED,
                    ),
                    (
                        "Queue",
                        res.agent_averaged_stats.queue_time.to_base_unit(),
                        &YELLOW,
                    ),
                    (
                        "Idle",
                        res.agent_averaged_stats.idle_time.to_base_unit(),
                        &MAGENTA,
                    ),
                    (
                        "Discharged",
                        res.agent_averaged_stats.discharged_time.to_base_unit(),
                        &CYAN,
                    ),
                ];

                let mut start = 0.0;
                for (label, value, color) in segments {
                    let percent = value / total_time * 100.0; // convert to percentage

                    let rect = Rectangle::new(
                        [
                            (idx - bar_width / 2.0, start),
                            (idx + bar_width / 2.0, start + percent),
                        ],
                        color.filled(),
                    );

                    if added_labels.insert(label) {
                        chart
                            .draw_series(std::iter::once(rect))?
                            .label(label)
                            .legend(move |(x, y)| {
                                Rectangle::new([(x, y - 5), (x + 10, y + 5)], color.filled())
                            });
                    } else {
                        chart.draw_series(std::iter::once(rect))?;
                    }

                    start += percent;
                }

                // Draw combination label above bar
                let combo_label = &best_combo.label;
                chart.draw_series(std::iter::once(Text::new(
                    combo_label.clone(),
                    (idx, 110.0),
                    ("sans-serif", 15).into_font().color(&BLACK),
                )))?;
            }
        }
    }

    // Add legend
    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .draw()?;

    Ok(())
}
