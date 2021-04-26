use serde::{Deserialize, Serialize};
use std::{error::Error, fs::*, u128};
use std::{io::Read, path::PathBuf};

#[derive(Debug, Clone, Default)]
struct BenchResult {
    path: PathBuf,
    cycles: (u128, u128),
    instructions: (u128, u128),
    faults: (u128, u128),
    minor_faults: (u128, u128),
    major_faults: (u128, u128),
    bus_cycles: (u128, u128),
    d_tlb_loads: (u128, u128),
    d_tlb_load_misses: (u128, u128),
    llc_loads: (u128, u128),
    llc_load_misses: (u128, u128),
    l1_cache_loads: (u128, u128),
    l1_cache_load_misses: (u128, u128),
    l1_cache_stores: (u128, u128),
    llc_stores: (u128, u128),
    cache_references: (u128, u128),
    cache_misses: (u128, u128),
    branch_instructions: (u128, u128),
    d_tlb_stores: (u128, u128),
    d_tlb_store_misses: (u128, u128),
    branch_misses: (u128, u128),
    context_switches: (u128, u128),
}

#[derive(Debug, Clone)]
struct MsgCount {
    counts: Vec<u128>,
    path: PathBuf,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
struct MergedBenchResult {
    msg_size: String,
    connections: u128,
    kind: String,
    counts: u128,
    cycles: u128,
    instructions: u128,
    faults: u128,
    minor_faults: u128,
    major_faults: u128,
    bus_cycles: u128,
    d_tlb_loads: u128,
    d_tlb_load_misses: u128,
    llc_loads: u128,
    llc_load_misses: u128,
    l1_cache_loads: u128,
    l1_cache_load_misses: u128,
    l1_cache_stores: u128,
    llc_stores: u128,
    cache_references: u128,
    cache_misses: u128,
    branch_instructions: u128,
    d_tlb_stores: u128,
    d_tlb_store_misses: u128,
    branch_misses: u128,
    context_switches: u128,
}

fn main() -> Result<(), Box<dyn Error>> {
    let perf_benches = perf_benches().unwrap();
    let total_msgs = total_msgs().unwrap();
    let mut merged = merge_bench_result(perf_benches, total_msgs).unwrap();
    merged.sort();

    let mut writer = csv::Writer::from_path("benchmarks.csv")?;

    for item in merged {
        writer.serialize(item)?;
        writer.flush()?;
    }

    Ok(())
}

fn total_msgs() -> Result<Vec<MsgCount>, Box<dyn Error>> {
    let benches = std::fs::read_dir("bench")?;
    let mut total_msgs = vec![];

    for path in benches {
        let path = path?.path();
        let mut file = File::open(&path)?;
        let mut msg_count = MsgCount {
            counts: vec![],
            path: path.clone(),
        };

        match path.extension() {
            Some(txt) if txt == "txt" => {
                let mut buf = String::new();
                file.read_to_string(&mut buf).ok();

                for line in buf.lines() {
                    let split = line.split(|c| c == ':' || c == ',').collect::<Vec<_>>();
                    if line == "" || split.len() < 2 {
                        continue;
                    }

                    if split[0] == "TaskTotalMsgs" || "ThreadsTotalMsgs" == split[0] {
                        let msgs: u128 = split[1].trim().parse()?;
                        msg_count.counts.push(msgs);
                    }
                }

                total_msgs.push(msg_count);
            }
            _ => {}
        }
    }

    Ok(total_msgs)
}

fn perf_benches() -> Result<Vec<BenchResult>, Box<dyn Error>> {
    let benches = std::fs::read_dir("bench")?;
    let mut bench_results = vec![];

    for path in benches {
        let path = path?.path();
        let mut file = File::open(&path)?;

        match path.extension() {
            Some(csv) if csv == "csv" => {
                let mut bench_result = BenchResult::default();
                bench_result.path = path;
                let mut buf = String::new();
                file.read_to_string(&mut buf).ok();

                for line in buf.lines() {
                    if line.starts_with("#") {
                        continue;
                    }
                    if line == "" {
                        continue;
                    }
                    let fields = line.split(",").collect::<Vec<_>>();

                    if fields[0].contains("not supported") {
                        continue;
                    }

                    let val = fields[0];
                    let key = fields[2];
                    if val.contains("not counted") {
                        continue;
                    }
                    match key {
                        "cycles" => bench_result.cycles = (val.parse()?, bench_result.cycles.1 + 1),
                        "instructions" => {
                            bench_result.instructions =
                                (val.parse()?, bench_result.instructions.1 + 1)
                        }
                        "page-faults" | "faults" => {
                            bench_result.faults = (val.parse()?, bench_result.faults.1 + 1)
                        }
                        "major-faults" => {
                            bench_result.major_faults =
                                (val.parse()?, bench_result.major_faults.1 + 1)
                        }
                        "minor-faults" => {
                            bench_result.minor_faults =
                                (val.parse()?, bench_result.minor_faults.1 + 1)
                        }
                        "bus-cycles" => {
                            bench_result.bus_cycles = (val.parse()?, bench_result.bus_cycles.1 + 1)
                        }
                        "dTLB-load-misses" => {
                            bench_result.d_tlb_load_misses =
                                (val.parse()?, bench_result.d_tlb_load_misses.1 + 1)
                        }
                        "dTLB-loads" => {
                            bench_result.d_tlb_loads =
                                (val.parse()?, bench_result.d_tlb_loads.1 + 1)
                        }
                        "LLC-load-misses" => {
                            bench_result.llc_load_misses =
                                (val.parse()?, bench_result.llc_load_misses.1 + 1)
                        }
                        "LLC-stores" => {
                            bench_result.llc_load_misses =
                                (val.parse()?, bench_result.llc_stores.1 + 1)
                        }
                        "LLC-loads" => {
                            bench_result.llc_loads = (val.parse()?, bench_result.llc_loads.1 + 1)
                        }
                        "branch-misses" => {
                            bench_result.branch_misses =
                                (val.parse()?, bench_result.branch_misses.1 + 1)
                        }
                        "cs" => {
                            bench_result.context_switches =
                                (val.parse()?, bench_result.context_switches.1 + 1)
                        }
                        "dTLB-stores" => {
                            bench_result.d_tlb_stores =
                                (val.parse()?, bench_result.d_tlb_stores.1 + 1)
                        }
                        "cache-misses" => {
                            bench_result.cache_misses =
                                (val.parse()?, bench_result.cache_misses.1 + 1)
                        }
                        "cache-references" => {
                            bench_result.cache_references =
                                (val.parse()?, bench_result.cache_references.1 + 1)
                        }
                        "L1-dcache-loads" => {
                            bench_result.l1_cache_loads =
                                (val.parse()?, bench_result.l1_cache_loads.1 + 1)
                        }
                        "branch-instructions" => {
                            bench_result.branch_instructions =
                                (val.parse()?, bench_result.branch_instructions.1 + 1)
                        }
                        "L1-dcache-load-misses" => {
                            bench_result.l1_cache_load_misses =
                                (val.parse()?, bench_result.l1_cache_load_misses.1 + 1)
                        }
                        "L1-dcache-stores" => {
                            bench_result.l1_cache_stores =
                                (val.parse()?, bench_result.l1_cache_stores.1 + 1)
                        }
                        "dTLB-stores-misses" => {
                            bench_result.d_tlb_store_misses =
                                (val.parse()?, bench_result.d_tlb_store_misses.1 + 1)
                        }
                        _ => println!("{}", line),
                    }
                }

                bench_results.push(bench_result);
            }
            _ => {}
        }
    }

    Ok(bench_results)
}

fn merge_bench_result(
    bench_result: Vec<BenchResult>,
    msg_count: Vec<MsgCount>,
) -> Result<Vec<MergedBenchResult>, Box<dyn Error>> {
    macro_rules! avg {
        ($tuple: expr) => {{
            let val = $tuple.0;
            let count = $tuple.1;

            if (count == 0) {
                0
            } else {
                val / count
            }
        }};
    }

    let mut merged: Vec<MergedBenchResult> = vec![];

    let bench_result_values: Vec<_> = bench_result
        .iter()
        .map(|b| {
            let path_string = b.path.as_os_str().to_str().unwrap();
            let split = path_string.split("_").collect::<Vec<_>>();

            let conns = split[2];
            let msg_size = split[3];
            let kind = split[4].replace(".csv", "");

            (conns, msg_size, kind, b.clone())
        })
        .collect();

    let msg_result_values: Vec<_> = msg_count
        .iter()
        .map(|b| {
            let path_string = b.path.as_os_str().to_str().unwrap();
            let split = path_string.split("_").collect::<Vec<_>>();

            let kind = split[2];
            let conn = split[3];
            let msg_size = split[4];
            (conn, msg_size, kind, b.clone())
        })
        .collect();

    for bench in bench_result_values {
        let msg_count = msg_result_values
            .iter()
            .find(|i| i.0 == bench.0 && i.1 == bench.1 && i.2 == bench.2);

        let bench_result = bench.3.clone();

        let mut counts = msg_count.unwrap().3.counts.clone();

        counts.sort();
        counts.remove(0);
        let count_avg: u128 = counts.iter().sum::<u128>() / counts.len() as u128;
        let merged_bench_result = MergedBenchResult {
            branch_instructions: avg!(bench_result.branch_instructions),
            branch_misses: avg!(bench_result.branch_misses),
            bus_cycles: avg!(bench_result.bus_cycles),
            cache_misses: avg!(bench_result.cache_misses),
            cache_references: avg!(bench_result.cache_references),
            context_switches: avg!(bench_result.context_switches),
            cycles: avg!(bench_result.cycles),
            d_tlb_load_misses: avg!(bench_result.d_tlb_load_misses),
            d_tlb_loads: avg!(bench_result.d_tlb_loads),
            d_tlb_store_misses: avg!(bench_result.d_tlb_store_misses),
            d_tlb_stores: avg!(bench_result.d_tlb_stores),
            faults: avg!(bench_result.faults),
            instructions: avg!(bench_result.instructions),
            l1_cache_load_misses: avg!(bench_result.l1_cache_load_misses),
            l1_cache_loads: avg!(bench_result.l1_cache_loads),
            l1_cache_stores: avg!(bench_result.l1_cache_stores),
            llc_load_misses: avg!(bench_result.llc_load_misses),
            llc_loads: avg!(bench_result.llc_loads),
            llc_stores: avg!(bench_result.llc_stores),
            major_faults: avg!(bench_result.major_faults),
            minor_faults: avg!(bench_result.minor_faults),
            connections: bench.0.to_owned().parse()?,
            msg_size: bench.1.to_owned(),
            kind: bench.2.to_owned(),
            counts: count_avg,
        };

        merged.push(merged_bench_result);
    }

    Ok(merged)
}
