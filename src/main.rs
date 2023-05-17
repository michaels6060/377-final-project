// think of as import or include
mod scheduler;

// similar to namespaces
use std::{env};
use scheduler::*;

fn main(){
    let args: Vec<String> = env::args().collect(); // reads command line arguments into a vector, similar to C++ vector STL
    if args.len() != 3 {
        println!("usage: cargo run -- [fifo|sjf|stcf|rr|mlfq] workload_file");
        return;
    }

    let algo: &String = &args[1]; // This is a reference to the second String in the args vector, a borrow of the value
    let wkld_path: &String = &args[2];

    let wkld = read_workload(&wkld_path);

    match algo.as_str() { // switch statement equivalent
        "fifo" => show_metrics(&fifo(&wkld)),
        "sjf" => show_metrics(&sjf(&wkld)),
        "stcf" => show_metrics(&stcf(&wkld)),
        "rr" => show_metrics(&rr(&wkld)),
        "mlfq" => show_metrics(&mlfq(&wkld)),
        _ => {
            println!("Error: Unknown algorithm:");
            println!("usage: cargo run -- [fifo|sjf|stcf|rr|mlfq] workload_file");
        }
    }

    return;
 }