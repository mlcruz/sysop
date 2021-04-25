use std::{error::Error, fs::*, path::Path, u128};
use std::{io::Read, path::PathBuf};

#[derive(Debug, Clone, Default)]
struct BenchResult {
    path: PathBuf,
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
    let benches = std::fs::read_dir("bench")?;

    for path in benches {
        let path = path?.path();

        match path.extension() {
            Some(csv) if csv == "csv" => {
                let mut file = File::open(&path)?;
                let mut buf = String::new();
                file.read_to_string(&mut buf).ok();
                let mut bench_result = BenchResult::default();

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
                        "cycles" => bench_result.cycles = val.parse()?,
                        "instructions" => bench_result.instructions = val.parse()?,
                        "page-faults" | "faults" => bench_result.faults = val.parse()?,
                        "major-faults" => bench_result.major_faults = val.parse()?,
                        "minor-faults" => bench_result.minor_faults = val.parse()?,
                        "bus-cycles" => bench_result.bus_cycles = val.parse()?,
                        "dTLB-load-misses" => bench_result.d_tlb_load_misses = val.parse()?,
                        "dTLB-loads" => bench_result.d_tlb_loads = val.parse()?,
                        "LLC-load-misses" => bench_result.llc_load_misses = val.parse()?,
                        "LLC-stores" => bench_result.llc_load_misses = val.parse()?,
                        "LLC-loads" => bench_result.llc_loads = val.parse()?,
                        "branch-misses" => bench_result.branch_misses = val.parse()?,
                        "cs" => bench_result.context_switches = val.parse()?,
                        "dTLB-stores" => bench_result.d_tlb_stores = val.parse()?,
                        "cache-misses" => bench_result.cache_misses = val.parse()?,
                        "cache-references" => bench_result.cache_references = val.parse()?,
                        "L1-dcache-loads" => bench_result.l1_cache_loads = val.parse()?,
                        "branch-instructions" => bench_result.branch_instructions = val.parse()?,
                        "L1-dcache-load-misses" => {
                            bench_result.l1_cache_load_misses = val.parse()?
                        }
                        "L1-dcache-stores" => bench_result.l1_cache_stores = val.parse()?,
                        "dTLB-stores-misses" => bench_result.d_tlb_store_misses = val.parse()?,
                        _ => println!("{}", line),
                    }
                }
            }
            _ => {}
        }
    }
    Ok(())
}
