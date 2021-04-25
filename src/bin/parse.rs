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

fn main() -> Result<(), Box<dyn Error>> {
    let benches = std::fs::read_dir("bench")?;

    for path in benches {
        let path = path?.path();
        let mut file = File::open(&path)?;
        let mut bench_result = BenchResult::default();

        match path.extension() {
            Some(csv) if csv == "csv" => {
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
            }
            _ => {}
        }
    }
    Ok(())
}
