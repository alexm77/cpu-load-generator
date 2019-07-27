extern crate num_cpus;

use rand::distributions::{Distribution, Uniform};
use std::env;
use std::thread;
use std::time::{Duration, Instant};

// wider range supposedly converges faster
const RADIUS: f64 = 10.0;
const RANGE_START: f64 = -RADIUS;
const RANGE_END: f64 = RADIUS;
// maybe we want this as parameter?
const ITERATIONS: u64 = 1_000_000_000;

// program params
const PHYSICAL_CORES_PARAM: &str = "-physical";
const LOGICAL_CORES_PARAM: &str = "-logical";
const USER_DEF_THREADS_PARAM: &str = "-threads:";

type ComputeResult = (String, f64, Duration);

fn main() {
    println!(
        "CPUs: {} physical, {} logical",
        num_cpus::get_physical(),
        num_cpus::get()
    );

    let workers_no = parse_args();
    let start = Instant::now();
    let workers = start_workers(workers_no);
    let mut results = vec![];

    for worker in workers {
        results.push(worker.join().unwrap());
    }
    // exclude printing time
    let end = start.elapsed();

    for (thread_name, pi, time) in results {
        println!("{}: π ~ {} [{:#?}]", thread_name, pi, time);
    }

    println!("All done in {:?}", end)
}

fn parse_args() -> usize {
    let args: Vec<String> = env::args().collect();
    let mut thread_no = 0;
    for i in 1..args.len() {
        let arg = args[i].as_str();

        if PHYSICAL_CORES_PARAM.eq(arg) {
            thread_no = num_cpus::get_physical();
        } else if LOGICAL_CORES_PARAM.eq(arg) {
            thread_no = num_cpus::get();
        } else if arg.starts_with(USER_DEF_THREADS_PARAM) {
            let thread_count_str = &arg[USER_DEF_THREADS_PARAM.len()..];
            let thread_count = thread_count_str.parse::<usize>();
            match thread_count {
                Ok(v) => thread_no = v,
                _ => (),
            }
        }
    }
    if thread_no <= 0 {
        print_help_and_exit(&args[0]);
    }

    thread_no
}

fn start_workers(count: usize) -> Vec<thread::JoinHandle<ComputeResult>> {
    let mut workers = vec![];

    for i in 0..count {
        let worker_name = format!("{}{}", "worker", i).to_string();
        let worker = thread::Builder::new()
            .name(worker_name.clone())
            .spawn(move || {
                // monte carlo pi
                let start = Instant::now();
                let range = Uniform::new(RANGE_START, RANGE_END);
                let mut rng = rand::thread_rng();

                let mut inside = 0;

                for _ in 0..ITERATIONS {
                    let x = range.sample(&mut rng);
                    let y = range.sample(&mut rng);
                    if x * x + y * y <= RADIUS * RADIUS {
                        inside += 1
                    }
                }

                (
                    String::from(thread::current().name().unwrap()),
                    4. * (inside as f64) / (ITERATIONS as f64),
                    start.elapsed(),
                )
            });
        match worker {
            Ok(w) => workers.push(w),
            Err(error) => eprintln!("Error spawning worker {}: {}", worker_name, error),
        }
    }

    workers
}

fn print_help_and_exit(prog_name: &str) {
    let prog_path = std::path::Path::new(prog_name);
    let prog_parent = prog_path.parent();
    let executable_name = match prog_parent {
        Some(v) => &prog_name[(v.to_string_lossy().len() + 1)..],
        None => prog_name,
    };
    println!(
        "Usage: {} <{}>|<{}>|<{}thread_count>
\twhere:
\t\t{} means use as many threads as physical cores
\t\t{} means use as many threads as logical cores
\t\t{} means use as many threads as requsted (>0)",
        executable_name,
        PHYSICAL_CORES_PARAM,
        LOGICAL_CORES_PARAM,
        USER_DEF_THREADS_PARAM,
        PHYSICAL_CORES_PARAM,
        LOGICAL_CORES_PARAM,
        USER_DEF_THREADS_PARAM
    );

    ::std::process::exit(1)
}
