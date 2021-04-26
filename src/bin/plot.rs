use plotters::prelude::*;
use std::error::Error;
use sysop::MergedBenchResult;

fn main() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path("benchmarks.csv")?;

    let mut data = vec![];
    for result in rdr.deserialize() {
        let record: MergedBenchResult = result?;
        data.push(record);
    }

    let root_area = BitMapBackend::new("IPC.png", (600, 400)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let chart_data = chart_by_msg_size("8", &data);

    let measurements = chart_data.iter().map(|m| m.connections);

    let ipc = chart_data
        .iter()
        .map(|m| (m.connections, m.instructions as f64 / m.cycles as f64));

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("IPC", ("sans-serif", 40))
        .build_cartesian_2d(0..measurements.max().unwrap(), 0.0..1.5)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(LineSeries::new(ipc, &RED))
        .unwrap()
        .label("IPC")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    ctx.configure_series_labels()
        .border_style(&BLACK)
        .background_style(&WHITE.mix(0.8))
        .draw()
        .unwrap();

    Ok(())
}

fn chart_by_msg_size(msg_size: &str, merged: &Vec<MergedBenchResult>) -> Vec<MergedBenchResult> {
    merged
        .iter()
        .filter(|m| m.msg_size == msg_size)
        .cloned()
        .collect()
}
