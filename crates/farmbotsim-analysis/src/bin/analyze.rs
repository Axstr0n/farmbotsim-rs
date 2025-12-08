use farmbotsim_core::prelude::{ChargingStrategy, ChooseStationStrategy};
use plotters::prelude::*;
use plotters::style::text_anchor::{HPos, Pos, VPos};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

use farmbotsim_core::statistics::{AnalyzeEnvResult, Combination, ExperimentOutput};

const DISCHARGED_SCORE: f32 = -1_000_000.0;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json_path = "analyze/output.json";
    let data = fs::read_to_string(json_path)?;
    let experiment_output: ExperimentOutput = serde_json::from_str(&data)?;

    let combinations = &experiment_output.combinations;
    let results = &experiment_output.results;
    let parameters = &experiment_output.parameters;
    let num_agents = parameters.number_agents.clone();

    prep_folder(&num_agents)?;

    // Write combination tables
    for layout in [TableLayout::Normal, TableLayout::Matrix] {
        write_combinations_table_latex("analyze/tables/combinations", combinations, layout)?;
    }

    let best_combinations = select_best_combination(results);
    write_best_combinations_table_latex("analyze/tables/best_combinations", &best_combinations)?;
    println!("{best_combinations:?}");

    // Plots: n_agents-line, x-combinations
    {
        // Tasks completed
        plot_metric(
            "analyze/plots/task_completed.png",
            combinations,
            results,
            &DisplayValue::NTasks,
            |r| r.n_completed_tasks.avg,
        )?;

        // Work time
        plot_metric_pair(
            combinations,
            results,
            DisplayValue::WorkTime,
            |r| r.agent_averaged_stats.work_time.to_base_unit(),
            |r| r.agent_totaled_stats.work_time.to_base_unit(),
        )?;

        // Travel time
        plot_metric_pair(
            combinations,
            results,
            DisplayValue::TravelTime,
            |r| r.agent_averaged_stats.travel_time.to_base_unit(),
            |r| r.agent_totaled_stats.travel_time.to_base_unit(),
        )?;

        // Idle time
        plot_metric_pair(
            combinations,
            results,
            DisplayValue::WaitTime,
            |r| r.agent_averaged_stats.idle_time.to_base_unit(),
            |r| r.agent_totaled_stats.idle_time.to_base_unit(),
        )?;

        // Queue time
        plot_metric_pair(
            combinations,
            results,
            DisplayValue::QueueTime,
            |r| r.agent_averaged_stats.queue_time.to_base_unit(),
            |r| r.agent_totaled_stats.queue_time.to_base_unit(),
        )?;

        // Discharged time
        plot_metric_pair(
            combinations,
            results,
            DisplayValue::DischargeTime,
            |r| r.agent_averaged_stats.discharged_time.to_base_unit(),
            |r| r.agent_totaled_stats.discharged_time.to_base_unit(),
        )?;

        // Charging time
        plot_metric_pair(
            combinations,
            results,
            DisplayValue::ChargingTime,
            |r| r.agent_averaged_stats.charging_time.to_base_unit(),
            |r| r.agent_totaled_stats.charging_time.to_base_unit(),
        )?;

        // Distance travelled
        plot_metric_pair(
            combinations,
            results,
            DisplayValue::TravelDistance,
            |r| r.agent_averaged_stats.distance_travelled.to_base_unit(),
            |r| r.agent_totaled_stats.distance_travelled.to_base_unit(),
        )?;

        // Energy charged
        plot_metric_pair(
            combinations,
            results,
            DisplayValue::EnergyCharged,
            |r| r.agent_averaged_stats.energy_charged.to_base_unit(),
            |r| r.agent_totaled_stats.energy_charged.to_base_unit(),
        )?;

        // Energy discharged
        plot_metric_pair(
            combinations,
            results,
            DisplayValue::EnergyDischarged,
            |r| r.agent_averaged_stats.energy_discharged.to_base_unit(),
            |r| r.agent_totaled_stats.energy_discharged.to_base_unit(),
        )?;
    }

    // Best combinations
    for mode in [TimePlotMode::Absolute, TimePlotMode::Percentage] {
        plot_best_combinations(
            "analyze/plots/best_combination",
            best_combinations.clone(),
            results,
            mode,
        )?;
    }

    // Matrices for all num of agents
    for n_agents in num_agents {
        plot_matrix_value(
            n_agents,
            results,
            score,
            DisplayValue::Score,
            MetricDirection::HigherIsBetter,
        )?;
        plot_matrix_value(
            n_agents,
            results,
            |r| r.n_completed_tasks.avg,
            DisplayValue::NTasks,
            MetricDirection::HigherIsBetter,
        )?;
        plot_matrix_value(
            n_agents,
            results,
            |r| r.agent_averaged_stats.work_time.to_base_unit(),
            DisplayValue::WorkTime,
            MetricDirection::HigherIsBetter,
        )?;
        plot_matrix_value(
            n_agents,
            results,
            |r| r.agent_averaged_stats.queue_time.to_base_unit(),
            DisplayValue::QueueTime,
            MetricDirection::LowerIsBetter,
        )?;
        plot_matrix_value(
            n_agents,
            results,
            |r| r.agent_averaged_stats.travel_time.to_base_unit(),
            DisplayValue::TravelTime,
            MetricDirection::LowerIsBetter,
        )?;
        plot_matrix_value(
            n_agents,
            results,
            |r| r.agent_averaged_stats.charging_time.to_base_unit(),
            DisplayValue::ChargingTime,
            MetricDirection::LowerIsBetter,
        )?;
        plot_matrix_value(
            n_agents,
            results,
            |r| r.agent_averaged_stats.idle_time.to_base_unit(),
            DisplayValue::WaitTime,
            MetricDirection::LowerIsBetter,
        )?;
        plot_matrix_value(
            n_agents,
            results,
            |r| r.agent_averaged_stats.discharged_time.to_base_unit(),
            DisplayValue::DischargeTime,
            MetricDirection::LowerIsBetter,
        )?;
        plot_matrix_value(
            n_agents,
            results,
            |r| r.agent_averaged_stats.distance_travelled.to_base_unit(),
            DisplayValue::TravelDistance,
            MetricDirection::HigherIsBetter,
        )?;
        plot_matrix_value(
            n_agents,
            results,
            |r| r.agent_averaged_stats.energy_charged.to_base_unit(),
            DisplayValue::EnergyCharged,
            MetricDirection::HigherIsBetter,
        )?;
        plot_matrix_value(
            n_agents,
            results,
            |r| r.agent_averaged_stats.energy_discharged.to_base_unit(),
            DisplayValue::EnergyDischarged,
            MetricDirection::HigherIsBetter,
        )?;
    }

    Ok(())
}

/// Prepers the folder structure
fn prep_folder(num_agents: &[u32]) -> io::Result<()> {
    let analyze_path = Path::new("analyze");

    // Ensure analyze folder exists
    if !analyze_path.exists() {
        fs::create_dir_all(analyze_path)?;
    }

    // Clean analyze folder but keep output.json
    if analyze_path.is_dir() {
        for entry in fs::read_dir(analyze_path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str())
                    && name != "output.json"
                {
                    fs::remove_file(path)?;
                }
            } else if path.is_dir() {
                fs::remove_dir_all(path)?;
            }
        }
    }

    // Create folders
    fs::create_dir_all(analyze_path.join("tables"))?;

    fs::create_dir_all(analyze_path.join("matrices"))?;
    let matrices_path = analyze_path.join("matrices");
    for &n in num_agents {
        let agent_folder = matrices_path.join(format!("{n}_agents"));
        fs::create_dir_all(agent_folder)?;
    }

    fs::create_dir_all(analyze_path.join("plots"))?;

    Ok(())
}

/// Represents every labeled value
#[derive(Debug, Clone)]
enum DisplayValue {
    Score,
    NTasks,
    WorkTime,
    QueueTime,
    TravelTime,
    ChargingTime,
    WaitTime,
    DischargeTime,
    TravelDistance,
    EnergyCharged,
    EnergyDischarged,
}
impl DisplayValue {
    fn to_path_str(&self) -> &str {
        match self {
            Self::Score => "score",
            Self::NTasks => "n_tasks",
            Self::WorkTime => "work_time",
            Self::QueueTime => "queue_time",
            Self::TravelTime => "travel_time",
            Self::ChargingTime => "charging_time",
            Self::WaitTime => "wait_time",
            Self::DischargeTime => "discharge_time",
            Self::TravelDistance => "travel_distance",
            Self::EnergyCharged => "energy_charged",
            Self::EnergyDischarged => "energy_discharged",
        }
    }
    fn to_slovene_plot_label(&self) -> &str {
        match self {
            Self::Score => "Rezultat",
            Self::NTasks => "Število opravljenih nalog",
            Self::WorkTime => "Čas dela [s]",
            Self::QueueTime => "Čas čakanja v čakalni vrsti [s]",
            Self::TravelTime => "Čas premikanja [s]",
            Self::ChargingTime => "Čas polnjenja [s]",
            Self::WaitTime => "Čas mirovanja [s]",
            Self::DischargeTime => "Čas spraznjenosti [s]",
            Self::TravelDistance => "Prepotovana pot [m]",
            Self::EnergyCharged => "Energija napolnjena [J]",
            Self::EnergyDischarged => "Energija porabljena [J]",
        }
    }
}

/// What layout is table
#[derive(Debug)]
enum TableLayout {
    Normal, // One row per combination
    Matrix, // Matrix form (charging x station)
}

/// Write file that represents latex table that
/// shows combination in specified layout
fn write_combinations_table_latex(
    base_file_path: &str,
    combinations: &[Combination],
    layout: TableLayout,
) -> Result<(), Box<dyn std::error::Error>> {
    use std::collections::{HashMap, HashSet};
    let file_path = &format!("{base_file_path}_{layout:?}.txt").to_lowercase();
    let mut file = File::create(file_path)?;
    let caption = "Kombinacije strategij";
    let label = "tab:kombinacije_strategij";

    writeln!(file, "\\begin{{table}}[H]")?;
    writeln!(file, "\\caption{{{caption}}}")?;
    writeln!(file, "\\label{{{label}}}")?;
    writeln!(file, "\\centering")?;

    match layout {
        TableLayout::Normal => {
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
        }

        TableLayout::Matrix => {
            // Collect unique strategies
            let mut charging_strats: Vec<String> = combinations
                .iter()
                .map(|c| format!("{}", c.charging_strategy))
                .collect::<HashSet<_>>()
                .into_iter()
                .collect();
            charging_strats.sort();

            let mut station_strats: Vec<String> = combinations
                .iter()
                .map(|c| format!("{}", c.station_strategy))
                .collect::<HashSet<_>>()
                .into_iter()
                .collect();
            station_strats.sort();

            // Map (charging, station) -> label
            let mut combo_map: HashMap<(String, String), String> = HashMap::new();
            for c in combinations {
                combo_map.insert(
                    (
                        format!("{}", c.charging_strategy),
                        format!("{}", c.station_strategy),
                    ),
                    c.label.clone(),
                );
            }

            // Table with matrix layout
            writeln!(
                file,
                "\\begin{{tabular}}{{|c|{}}}",
                "c|".repeat(station_strats.len())
            )?;
            writeln!(file, "\\hline")?;

            // Header row
            write!(file, "\\rotatebox{{90}}{{*}}")?;
            for station in &station_strats {
                write!(file, " & \\rotatebox{{90}}{{{station}}}")?;
            }
            writeln!(file, " \\\\ \\hline")?;

            // Rows = charging strategies
            for charging in &charging_strats {
                write!(file, "{charging}")?;
                for station in &station_strats {
                    let label = combo_map
                        .get(&(charging.clone(), station.clone()))
                        .cloned()
                        .unwrap_or_else(|| "-".to_string());
                    write!(file, " & {label}")?;
                }
                writeln!(file, " \\\\ \\hline")?;
            }

            writeln!(file, "\\end{{tabular}}")?;
        }
    }

    writeln!(file, "\\end{{table}}")?;
    Ok(())
}

/// Write file that represents latex table that
/// shows best combination for each agent count
fn write_best_combinations_table_latex(
    base_file_path: &str,
    best_combinations: &HashMap<u32, Option<Combination>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let file_path = &format!("{base_file_path}.txt").to_lowercase();
    let mut file = File::create(file_path)?;
    let caption = "Optimalne kombinacije strategij";
    let label = "tab:optimalne_kombinacije_strategij";

    writeln!(file, "\\begin{{table}}[H]")?;
    writeln!(file, "\\caption{{{caption}}}")?;
    writeln!(file, "\\label{{{label}}}")?;
    writeln!(file, "\\centering")?;
    writeln!(file, "\\begin{{tabular}}{{|c|c|c|c|}}")?;
    writeln!(file, "\\hline")?;
    writeln!(
        file,
        "Število agentov & Oznaka & Strategija polnjenja & Strategija izbire postaje \\\\"
    )?;
    writeln!(file, "\\hline")?;

    let mut keys: Vec<_> = best_combinations.keys().cloned().collect();
    keys.sort(); // ascending order

    for n_agents in keys {
        match best_combinations.get(&n_agents) {
            Some(Some(c)) => {
                // Normal case: we have a valid combination
                let label = &c.label;
                let charging_str = format!("{}", c.charging_strategy);
                let station_str = format!("{}", c.station_strategy);

                writeln!(
                    file,
                    "{n_agents} & {label} & {charging_str} & {station_str} \\\\"
                )?;
                writeln!(file, "\\hline")?;
            }
            Some(None) => {
                // Case: best combination was None (e.g., score == DISCHARGE_SCORE)
                writeln!(file, "{n_agents} & / & / & / \\\\")?;
                writeln!(file, "\\hline")?;
            }
            None => {
                // Shouldn’t happen, but handle gracefully
                writeln!(file, "{n_agents} & // missing entry \\\\")?;
                writeln!(file, "\\hline")?;
            }
        }
    }

    writeln!(file, "\\end{{tabular}}")?;
    writeln!(file, "\\end{{table}}")?;
    Ok(())
}

/// Get best combination for each agent count
fn select_best_combination(results: &[AnalyzeEnvResult]) -> HashMap<u32, Option<Combination>> {
    let mut best_per_agents: HashMap<u32, Option<Combination>> = HashMap::new();

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
            if score(best) == DISCHARGED_SCORE {
                best_per_agents.insert(n_agents, None);
            } else {
                best_per_agents.insert(n_agents, Some(best.combination.clone()));
            }
        }
    }

    best_per_agents
}

/// Calculates the overall score for a given simulation result.
fn score(r: &AnalyzeEnvResult) -> f32 {
    // Weights: positive = maximize, negative = minimize
    const W_TASKS_COMPLETED: f32 = 5.0; // prioritize actual productivity
    const W_WORK_TIME: f32 = 2.0; // reward time spent working
    const W_TRAVEL_TIME: f32 = -0.8; // penalize unnecessary travel
    const W_QUEUE_TIME: f32 = -1.5; // penalize station queues
    const W_CHARGING_TIME: f32 = -1.0; // penalize spending too much time charging

    let work_time = r.agent_averaged_stats.work_time.to_base_unit();
    let travel_time = r.agent_averaged_stats.travel_time.to_base_unit();
    let queue_time = r.agent_averaged_stats.queue_time.to_base_unit();
    let charging_time = r.agent_averaged_stats.charging_time.to_base_unit();
    let discharged_time = r.agent_totaled_stats.discharged_time.to_base_unit();
    let tasks_completed = r.n_completed_tasks.avg;

    // Heavy penalty if any agent discharged
    if discharged_time > 0.0 {
        return DISCHARGED_SCORE;
    }

    // Weighted sum of contributions
    let mut total = 0.0;
    total += W_TASKS_COMPLETED * tasks_completed;
    total += W_WORK_TIME * work_time;
    total += W_TRAVEL_TIME * travel_time;
    total += W_QUEUE_TIME * queue_time;
    total += W_CHARGING_TIME * charging_time;

    total
}

/// Plots graph where x-axis represent all combinations,
/// y-axis represent selected value,
/// each line represents each agent count
fn plot_metric<F>(
    png_path: &str,
    combinations: &[Combination],
    results: &[AnalyzeEnvResult],
    displayed_value: &DisplayValue,
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

    let y_min = plot_data
        .iter()
        .map(|p| p.value)
        .fold(f32::MAX, f32::min)
        .max(0.0);
    let y_max = plot_data
        .iter()
        .map(|p| p.value)
        .fold(f32::MIN, f32::max)
        .max(10.0);

    let format_val = |v: f32| format!("{v:.0}");
    let max_label_len = format_val(y_max.abs().max(y_min.abs())).len();
    let y_label_area_size = (max_label_len as u32) * 8 + 30;

    let mut chart = ChartBuilder::on(&root)
        .margin(10)
        .margin_left(30)
        .x_label_area_size(40)
        .y_label_area_size(y_label_area_size)
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
        .x_label_style(TextStyle::from(("sans-serif", 12)).transform(FontTransform::Rotate90))
        .x_desc("Kombinacija strategij")
        .axis_desc_style(("sans-serif", 15))
        .y_desc(displayed_value.to_slovene_plot_label())
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
            // .label(format!("{n_agents} agents"))
            // .legend(move |(x, y)| Circle::new((x + 10, y), 3, color.filled()));
        }
    }

    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.5))
        .border_style(BLACK)
        .position(SeriesLabelPosition::LowerRight)
        .draw()?;

    Ok(())
}

/// Plots 2 graph for `plot_metric`
/// one for total combined value, other for average value
fn plot_metric_pair<F1, F2>(
    combinations: &[Combination],
    results: &[AnalyzeEnvResult],
    displayed_value: DisplayValue,
    avg_metric: F1,
    total_metric: F2,
) -> Result<(), Box<dyn std::error::Error>>
where
    F1: Fn(&AnalyzeEnvResult) -> f32,
    F2: Fn(&AnalyzeEnvResult) -> f32,
{
    let base_name = displayed_value.to_path_str();
    plot_metric(
        &format!("analyze/plots/{base_name}_avg.png"),
        combinations,
        results,
        &displayed_value,
        avg_metric,
    )?;
    plot_metric(
        &format!("analyze/plots/{base_name}_total.png"),
        combinations,
        results,
        &displayed_value,
        total_metric,
    )?;

    Ok(())
}

/// Mode specifies whether to use total time or normalized time
#[derive(Debug)]
enum TimePlotMode {
    Absolute,   // total time in seconds
    Percentage, // normalized to 100%
}

/// Plots histogram where x-axis represents n_agents,
/// y axis represents (time/percentage of time),
/// each bar is segmented on time chunks (work, queue, travel, charge, wait, discharge)
fn plot_best_combinations(
    base_png_path: &str,
    best_combinations: HashMap<u32, Option<Combination>>,
    results: &[AnalyzeEnvResult],
    mode: TimePlotMode,
) -> Result<(), Box<dyn std::error::Error>> {
    let png_path = &format!("{base_png_path}_{mode:?}.png").to_lowercase();
    let axis_desc_size = 25;
    let axis_label_size = 18;
    let combo_label_size = 18;
    let legend_size = 18;
    let root = BitMapBackend::new(png_path, (900, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    // Sort agent counts
    let mut agent_counts: Vec<u32> = best_combinations.keys().copied().collect();
    agent_counts.sort();

    // Compute max_total_time (only needed for Absolute mode)
    let mut max_total_time = 0.0;
    if let TimePlotMode::Absolute = mode {
        for n_agents in &agent_counts {
            if let Some(Some(best_combo)) = best_combinations.get(n_agents)
                && let Some(res) = results
                    .iter()
                    .find(|r| r.n_agents == *n_agents && r.combination.label == best_combo.label)
            {
                let total = res.agent_averaged_stats.work_time.to_base_unit()
                    + res.agent_averaged_stats.travel_time.to_base_unit()
                    + res.agent_averaged_stats.charging_time.to_base_unit()
                    + res.agent_averaged_stats.queue_time.to_base_unit()
                    + res.agent_averaged_stats.discharged_time.to_base_unit()
                    + res.agent_averaged_stats.idle_time.to_base_unit();

                if total > max_total_time {
                    max_total_time = total;
                }
            }
        }
    }

    // Chart range
    let y_range = match mode {
        TimePlotMode::Absolute => 0f32..(max_total_time * 1.1),
        TimePlotMode::Percentage => 0f32..100.0,
    };

    // Build chart
    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .x_label_area_size(50)
        .y_label_area_size(70)
        .build_cartesian_2d(-0.5f32..(agent_counts.len() as f32 - 0.5 + 1.0), y_range)?;

    chart
        .configure_mesh()
        .disable_mesh()
        .x_labels(agent_counts.len() + 1)
        .label_style(("sans-serif", axis_label_size))
        .x_label_formatter(&|idx: &f32| {
            let i = idx.round() as usize;
            agent_counts
                .get(i)
                .map(|v| v.to_string())
                .unwrap_or_default()
        })
        .y_desc(match mode {
            TimePlotMode::Absolute => "Poraba časa [s]",
            TimePlotMode::Percentage => "Poraba časa [%]",
        })
        .x_desc("Število agentov")
        .axis_desc_style(("sans-serif", axis_desc_size))
        .draw()?;

    let bar_width = 0.5;
    let mut added_labels = std::collections::HashSet::new();

    // Draw stacked bars
    for (idx, n_agents) in agent_counts.iter().enumerate() {
        let idx = idx as f32;

        if let Some(Some(best_combo)) = best_combinations.get(n_agents) {
            if let Some(res) = results
                .iter()
                .find(|r| r.n_agents == *n_agents && r.combination.label == best_combo.label)
            {
                // Compute total time (needed for normalization)
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
                        "Delo",
                        res.agent_averaged_stats.work_time.to_base_unit(),
                        &BLUE,
                    ),
                    (
                        "Premikanje",
                        res.agent_averaged_stats.travel_time.to_base_unit(),
                        &GREEN,
                    ),
                    (
                        "Polnjenje",
                        res.agent_averaged_stats.charging_time.to_base_unit(),
                        &RED,
                    ),
                    (
                        "Čakanje v vrsti",
                        res.agent_averaged_stats.queue_time.to_base_unit(),
                        &YELLOW,
                    ),
                    (
                        "Mirovanje",
                        res.agent_averaged_stats.idle_time.to_base_unit(),
                        &MAGENTA,
                    ),
                    (
                        "Izpraznjenost",
                        res.agent_averaged_stats.discharged_time.to_base_unit(),
                        &CYAN,
                    ),
                ];

                let mut start = 0.0;
                for (label, value, color) in segments {
                    let val = match mode {
                        TimePlotMode::Absolute => value,
                        TimePlotMode::Percentage => value / total_time * 100.0,
                    };

                    let rect = Rectangle::new(
                        [
                            (idx - bar_width / 2.0, start),
                            (idx + bar_width / 2.0, start + val),
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

                    start += val;
                }

                // Label above bar
                let combo_label = &best_combo.label;
                chart.draw_series(std::iter::once(Text::new(
                    combo_label.clone(),
                    (
                        idx,
                        match mode {
                            TimePlotMode::Absolute => max_total_time * 1.05,
                            TimePlotMode::Percentage => 105.0,
                        },
                    ),
                    ("sans-serif", combo_label_size)
                        .into_font()
                        .color(&BLACK)
                        .pos(Pos::new(HPos::Center, VPos::Bottom)),
                )))?;
            }
        } else {
            // No best combination
        }
    }

    // Add legend
    chart
        .configure_series_labels()
        .label_font(("sans-serif", legend_size))
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .draw()?;

    Ok(())
}

/// Metric specifying what is better
enum MetricDirection {
    HigherIsBetter,
    LowerIsBetter,
}

trait ToSlovene {
    fn to_slovene(&self) -> String;
}
impl ToSlovene for ChooseStationStrategy {
    fn to_slovene(&self) -> String {
        match self {
            Self::Manhattan(f) => format!("Manhattan({f})"),
            Self::Path(f) => format!("A*({f})"),
        }
    }
}
impl ToSlovene for ChargingStrategy {
    fn to_slovene(&self) -> String {
        match self {
            Self::CriticalOnly(t) => format!("SamoKritično({t})"),
            Self::ThresholdWithLimit(t1, t2) => format!("PragZLimito({t1},{t2})"),
        }
    }
}

/// Plots matrix where x-axis represent station_strategies,
/// y-axis represents charging_strategies,
/// color of rectangles value that is specified in params
fn plot_matrix_value<F>(
    n_agents: u32,
    results: &[AnalyzeEnvResult],
    value_fn: F,
    display_value: DisplayValue,
    direction: MetricDirection,
) -> Result<(), Box<dyn std::error::Error>>
where
    F: Fn(&AnalyzeEnvResult) -> f32,
{
    let size = (900, 600);
    let matrix_percentage = 0.92;
    let png_path = format!(
        "analyze/matrices/{n_agents}_agents/matrix_{n_agents}_{}.png",
        display_value.to_path_str()
    );
    println!("{png_path}");
    let slovene_text: String = display_value.to_slovene_plot_label().to_string();
    let matrix_axis_desc_size = 25;
    let matrix_axis_label_size = 18;
    let matrix_value_size = 18;
    let colorbar_axis_desc_size = 25;
    let colorbar_axis_label_size = 16;

    // Filter results by n_agents
    let results: Vec<_> = results.iter().filter(|r| r.n_agents == n_agents).collect();
    if results.is_empty() {
        return Err("No results found for the given n_agents".into());
    }

    // Unique strategies sorted for consistent ordering
    let mut charging_strats: Vec<_> = results
        .iter()
        .map(|r| r.combination.charging_strategy.clone())
        .collect();
    charging_strats.sort();
    charging_strats.dedup();
    let charging_strats: Vec<String> = charging_strats
        .into_iter()
        .map(|c| c.to_slovene())
        .collect();

    let mut station_strats: Vec<_> = results
        .iter()
        .map(|r| r.combination.station_strategy.clone())
        .collect();
    station_strats.sort();
    station_strats.dedup();
    let station_strats: Vec<String> = station_strats.into_iter().map(|s| s.to_slovene()).collect();

    let n_rows = charging_strats.len();
    let n_cols = station_strats.len();

    // Build score matrix
    let mut matrix = vec![vec![0.0f32; n_cols]; n_rows];
    for (i, charging) in charging_strats.iter().enumerate() {
        for (j, station) in station_strats.iter().enumerate() {
            if let Some(r) = results.iter().find(|r| {
                r.combination.charging_strategy.to_slovene() == *charging
                    && r.combination.station_strategy.to_slovene() == *station
            }) {
                matrix[i][j] = value_fn(r);
            }
        }
    }

    let mut min_score: Option<f32> = None;
    let mut max_score: Option<f32> = None;
    let mut all_discharged = true;

    for row in &matrix {
        for &val in row {
            // Track max ignoring discharged values if possible
            if val > DISCHARGED_SCORE {
                min_score = Some(match min_score {
                    Some(m) => m.min(val),
                    None => val,
                });
                all_discharged = false;
            }

            // Track max for all values
            max_score = Some(match max_score {
                Some(m) => m.max(val),
                None => val,
            });
        }
    }

    // If all values were discharged, min_score = DISCHARGED_SCORE
    let min_score = if all_discharged {
        max_score.unwrap() // all values are discharged, pick any value
    } else {
        min_score.unwrap()
    };

    // max_score should ignore discharged values if possible
    let max_score = if all_discharged {
        max_score.unwrap()
    } else {
        matrix
            .iter()
            .flat_map(|row| row.iter())
            .filter(|&&v| v > DISCHARGED_SCORE)
            .cloned()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap()
    };

    let root = BitMapBackend::new(&png_path, size).into_drawing_area();
    root.fill(&WHITE)?;

    // Split area: left = matrix, right = colorbar
    let left_size = (size.0 as f32 * matrix_percentage) as u32;
    let (left, right) = root.split_horizontally(left_size);

    // ---- Matrix ----
    let label_font = ("sans-serif", matrix_axis_label_size).into_font();
    let desc_font = ("sans-serif", matrix_axis_desc_size).into_font();
    let max_label_width = charging_strats
        .iter()
        .map(|s| label_font.box_size(s).unwrap().0) // .0 = width in pixels
        .max()
        .unwrap_or(0);
    let y_label_area_size = max_label_width + 40;

    let mut chart = ChartBuilder::on(&left)
        .margin(10)
        .x_label_area_size(60)
        .y_label_area_size(y_label_area_size)
        .build_cartesian_2d(0f32..n_cols as f32, 0f32..n_rows as f32)?;

    // compute the pixel distance in drawing area
    let x_range = station_strats.len() as f32;
    let y_range = charging_strats.len() as f32;
    let x_pixel_height = chart.plotting_area().dim_in_pixel().0 as f32;
    let y_pixel_height = chart.plotting_area().dim_in_pixel().1 as f32;
    let x_unit_per_pixel = x_range / x_pixel_height;
    let y_unit_per_pixel = y_range / y_pixel_height;
    // desired offset in "label units"
    let offset_units = 0.5;
    // convert to pixels
    let x_offset_pixels = (offset_units / x_unit_per_pixel) as i32;
    let y_offset_pixels = (offset_units / y_unit_per_pixel) as i32;

    chart
        .configure_mesh()
        .x_labels(n_cols * 2) // because of f32
        .y_labels(n_rows * 2)
        .x_label_formatter(&|x| {
            let idx = *x as usize;
            if idx < station_strats.len() {
                station_strats[idx].clone()
            } else {
                "".to_string()
            }
        })
        .y_label_formatter(&|y| {
            let idx = *y as usize;
            if idx < charging_strats.len() {
                charging_strats[idx].clone()
            } else {
                "".to_string()
            }
        })
        .x_desc("Strategija izbire postaje")
        .y_desc("Strategija polnjenja")
        .label_style(label_font)
        .axis_desc_style(desc_font)
        .x_label_offset(x_offset_pixels)
        .y_label_offset(-y_offset_pixels)
        .draw()?;

    for (y, row) in matrix.iter().enumerate() {
        for (x, &val) in row.iter().enumerate() {
            let color = if val == DISCHARGED_SCORE {
                RGBColor(170, 100, 214).filled() // special case
            } else {
                let frac = ((val - min_score) / (max_score - min_score)).clamp(0.0, 1.0);

                // For green→red scale: 0.33 (green) to 0.0 (red)
                let hue = match direction {
                    MetricDirection::HigherIsBetter => 0.0 + (0.33 - 0.0) * frac as f64, // red → green
                    MetricDirection::LowerIsBetter => 0.33 - (0.33 - 0.0) * frac as f64, // green → red
                };

                HSLColor(hue, 0.7, 0.5).filled()
            };

            let x0_bg = x as f32;
            let y0_bg = y as f32;
            let x1_bg = x as f32 + 1.0;
            let y1_bg = y as f32 + 1.0;

            match direction {
                MetricDirection::HigherIsBetter => {
                    if val == max_score {
                        chart.draw_series(std::iter::once(Rectangle::new(
                            [(x0_bg, y0_bg), (x1_bg, y1_bg)],
                            RGBColor(0, 0, 0).filled(),
                        )))?;
                    }
                }
                MetricDirection::LowerIsBetter => {
                    if val == min_score {
                        chart.draw_series(std::iter::once(Rectangle::new(
                            [(x0_bg, y0_bg), (x1_bg, y1_bg)],
                            RGBColor(0, 0, 0).filled(),
                        )))?;
                    }
                }
            }

            // draw smaller colored rectangle on top (gap effect)
            let gap = 0.03;
            let x0 = x0_bg + gap;
            let y0 = y0_bg + gap;
            let x1 = x1_bg - gap / 2.0;
            let y1 = y1_bg - gap;

            chart.draw_series(std::iter::once(Rectangle::new([(x0, y0), (x1, y1)], color)))?;
            // text in the center
            let cx = (x0 + x1) / 2.0;
            let cy = (y0 + y1) / 2.0;

            let pos = Pos::new(HPos::Center, VPos::Center);
            let text_style =
                TextStyle::from(("sans-serif", matrix_value_size).into_font()).pos(pos);
            chart.draw_series(std::iter::once(Text::new(
                format!("{val:.1}"),
                (cx, cy),
                text_style,
            )))?;
        }
    }

    // ---- Colorbar ----
    let label_font = ("sans-serif", colorbar_axis_label_size).into_font();
    let desc_font = ("sans-serif", colorbar_axis_desc_size).into_font();
    let y_range = match direction {
        MetricDirection::HigherIsBetter => min_score..max_score,
        MetricDirection::LowerIsBetter => max_score..min_score,
    };
    let mut cb = ChartBuilder::on(&right)
        .margin_left(20)
        .margin_right(10)
        .margin_top(10)
        .margin_bottom(63)
        .y_label_area_size(25)
        .build_cartesian_2d(0f32..1f32, y_range)?;

    cb.configure_mesh()
        .disable_x_mesh()
        .y_labels(0) // disable auto ticks
        .y_desc(&slovene_text)
        .label_style(label_font.clone())
        .axis_desc_style(desc_font)
        .draw()?;

    // Fill with vertical gradient
    let steps = 200;
    cb.draw_series((0..steps).map(|i| {
        let frac0 = i as f32 / steps as f32;
        let frac1 = (i + 1) as f32 / steps as f32;

        let val0 = min_score + frac0 * (max_score - min_score);
        let val1 = min_score + frac1 * (max_score - min_score);

        let color = {
            let hue = match direction {
                MetricDirection::HigherIsBetter => 0.0 + (0.33 - 0.0) * frac0 as f64,
                MetricDirection::LowerIsBetter => 0.33 - (0.33 - 0.0) * frac0 as f64,
            };
            HSLColor(hue, 0.7, 0.5).filled()
        };

        Rectangle::new([(0f32, val0), (1f32, val1)], color)
    }))?;

    // manually draw min/max ticks
    if min_score != max_score {
        let ticks = match direction {
            MetricDirection::HigherIsBetter => [(min_score, VPos::Bottom), (max_score, VPos::Top)],
            MetricDirection::LowerIsBetter => [(min_score, VPos::Top), (max_score, VPos::Bottom)],
        };

        for (val, vpos) in ticks {
            let pos = Pos::new(HPos::Right, vpos);
            let text_style = TextStyle::from(label_font.clone()).pos(pos);
            cb.draw_series(std::iter::once(Text::new(
                format!("{val:.0}"),
                (0.0, val),
                &text_style,
            )))?;
        }
    }

    Ok(())
}
