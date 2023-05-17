// using namespace of standard library (std) with specific modules (io, fs, cmp, collections) 
// and crate dependencies (num_traits, binary_heap_plus)
use std::io;
use std::io::BufRead;
use std::fs::File;
use std::cmp::Ordering;
use std::collections::VecDeque;
use num_traits::cast::ToPrimitive;
use binary_heap_plus;

// constants declaration, edit these to change the behavior of MLFQ
const BOOSTTIME: i32 = 10; // changes boost time, how long it takes before all processes are boosted to the first level queue
const MLFQPRINTING: bool = true; // true to print MLFQ state, false to disable printing

// this is a struct with 2 trait derivations. Traits can be thought of as interfaces
// I derive Clone here because I want to be able to use the .copy() method to make copies of a process
// I derive Debug here as well, this is mostly for printing functionality
#[derive(Clone, Debug)]
pub struct Process {
    pub arrival:    f32, // all values are 32-bit floating point numbers, also note here that everything has to be declared with pub
    pub duration:   f32, // class members and functions in other files by default are private
    pub first_run:  f32,
    pub completion: f32,
    pub remaining_time: f32,
}

// impl means I am implementing the class itself
impl Process { 
    // this is a constructor method, it takes 4 inputs all with generic type T 
    // (meaning these all have to be of the same type, can't mix and match floats with ints)
    // also notice that this is just a function like any other function, new is not a keyword,
    // it could be named "asdfasdf" and would be valid as a constructor. 
    // what is important is that it explicity states the return type with the "->" operator and returns a Self typed object, or a Process
    // also note the "where" statement before the bracket, this indicates that T must derive the ToPrimitive trait from the num_traits
    // crate I used. All numeric primitives derive this in the crate implementation of num_traits. I use this because I want to be able to
    // take in integers or floats, not just one or another.
    // I then call the method to_f32() to conver the inputs to primitive f32 type. I will explain ampersand and unwrape later.
    pub fn new<T>(arrival: T, duration: T, first_run: T, completion: T) -> Self 
        where T: ToPrimitive{
        Self {
            arrival: ToPrimitive::to_f32(&arrival).unwrap(),
            duration: ToPrimitive::to_f32(&duration).unwrap(),
            first_run: ToPrimitive::to_f32(&first_run).unwrap(),
            completion: ToPrimitive::to_f32(&completion).unwrap(),
            remaining_time: ToPrimitive::to_f32(&duration).unwrap(),
        }
    }
}

//-----------UTILS----------

// This function works similarly to the read_workload function in project 3, it takes in a path, and reads that file into a vector of processes
pub fn read_workload(wkld_path: &String) -> Vec<Process>{
    let mut wkld = Vec::new(); 
    // notice unwrap here. Rust has this functionality where many things return a Result enum. The enum itself has 2 types, Ok(T) or Err(E).
    // These are essentially wrappers around anything that could be returned. I have to call unwrap() to be able to extract that value 
    // from the Result. This is important because it adds the ability to avoid having null types in the language. Read more by googling
    // "null billion dollar mistake", the top results all explain a lot about the problems of having null references. 
    let file = File::open(wkld_path).unwrap(); 
    let lines = io::BufReader::new(file).lines(); // this function reads in a file to a buffered reader and returns an iterator over the file
    for line in lines {
        let l = line.unwrap();
        // there are two things to notice in the next function, the |s| and expect. What this line does essentially is 
        // take a line, split it on whitespaces into an iterator, and applies a map function, common in functional programming
        // to then parse the string into an integer, then collecting the iterator into a vector.
        // |s| indicates a closure, it is a an anonymous function which is Rust's version of a lambda function.
        // expect works similary to unwrap(), except that unwrap calls a macro called panic! which essentially ends execution during runtime
        // expect does not panic and instead can pass errors along.
        let numbers: Vec<i32> = l.split_whitespace().map(|s| s.parse().expect("parse error")).collect();
        let arrival = numbers[0];
        let duration = numbers[1];
        let p = Process::new(arrival,duration,0,0);
        wkld.push(p);
    }
    // sorting a vector using a comparator function. You might first notice here that we have an ampersand around b.arrival, wonder what it is
    // and wonder why a.arrival does not have this either. Ampersand or & indicates that this function takes in the borrowed value of b.arrival
    // There is a system of ownership within rust that only allows one pointer to a piece of data on the heap. But other functions such as partial_cmp()
    // can take in the borrowed value of it to perform calculations without consuming the ownership of the original pointer.
    // a.arrival also is being borrowed here, it is just that the Rust compilier will automatically add that during compile time. 
    // unwrap_or here either returns the result from the partial comparison or returns an Ordering::Equal type to indicate that the 
    // a.arrival and b.arrival are equal.
    wkld.sort_by(|a, b| a.arrival.partial_cmp(&b.arrival).unwrap_or(Ordering::Equal));
    wkld
}

// calculate average turnaround time (completion time - arrival time)
// input: borrowed Vector of Processes, output: f32
pub fn avg_turnaround(processes : &Vec<Process>) -> f32{
    let n = processes.len() as f32; // note here the type declaration, .len() returns an integer, we convert it to f32 with as
    let sum = processes.iter().fold(0.0, |acc, p| acc + p.completion - p.arrival); //similar to a reduce function
    sum / n
}

// calculate average turnaround time (first run time - arrival time)
// input: borrowed Vector of Processes, output: f32
pub fn avg_response(processes : &Vec<Process>) -> f32 {
    let n = processes.len() as f32;
    let sum = processes.iter().fold(0.0, |acc, p| acc + p.first_run - p.arrival);
    sum / n
}

// prints processes
// input: borrowed Vector of Processes, output: f32
pub fn show_processes(processes: &Vec<Process>) {
    let p_iter = processes.iter();
    // note here, this function, println!, the exlamation mark indicates this is a macro. This macro is by default included in the prelude
    // of the program. This is because Rust does not support variable arguments, so println has to be implemented as a macro to use format parameters
    println!("Processes:");
    for p in p_iter {
        println!(
            //format paramaters are illustrated here, very similar to how fstrings work in Python
            // or std::format in C++
            "\tarrival={}, duration={}, first_run={}, completion={}",
            p.arrival, p.duration, p.first_run, p.completion
        );
    }
}

// prints processes and metrics
// input: borrowed Vector of Processes, output: None
pub fn show_metrics(processes : &Vec<Process>){
    let turn = avg_turnaround(processes);
    let resp = avg_response(processes);
    show_processes(processes);
    println!("Average Turnaround Time: {}", turn);
    println!("Average Response Time:   {}", resp);
}


//----------ALGORITHMS-----------


// runs FIFO algorithm
// input: borrowed Vector of Processes, output: Vector of Processes
pub fn fifo(workload: &Vec<Process>) -> Vec<Process> {
    // Note here, that all these variables are declared with mut or mutable, by default values are not mutable. This ensures saftey as well
    // by preventing unecessary changes to references.
    let mut complete : Vec<Process> = Vec::new();
    let wkld_iter = workload.iter();
    let mut curr_time = workload.get(0).unwrap().arrival;

    for process in wkld_iter{
        let p = Process::new(process.arrival, process.duration, curr_time, curr_time+process.duration);
        curr_time += process.duration;
        complete.push(p);
    }
    // note here, we give up ownership of complete to whatever reference points to this function call
    // also note, because this is the last statement in the function, this is returned without an explicit call to return
    // this is the idiomatic way to return, calling return if the thing you are returning is the last statement works but is 
    // considered to be not idiomatic. Notice here that there is no semicolon either, if you added a semicolon, this function would
    // instead return a None type. 
    complete
}

// runs SJF algorithm
// input: borrowed Vector of Processes, output: Vector of Processes
pub fn sjf(workload:  &Vec<Process>) -> Vec<Process> {
    let mut wkld = VecDeque::from(workload.clone());
    let mut complete : Vec<Process> = Vec::new();
    let mut curr_time = wkld.get(0).unwrap().arrival;
    
    //Here I'm calling from the binary_heap_plus crate to use the BinaryHeap. This allows me to pass in a custom comparator.
    // Note as well the vec![] macro, this creates a vector from an array.
    let mut duration = binary_heap_plus::BinaryHeap::from_vec_cmp(vec![], 
        |p1: &Process, p2 :&Process| p2.duration.partial_cmp(&p1.duration).unwrap());

    duration.push(wkld.pop_front().unwrap());

    while !duration.is_empty() {
        let mut p = duration.pop().unwrap();
        p.first_run = curr_time;
        curr_time += p.duration;
        p.completion = curr_time;
        complete.push(p);
        while !wkld.is_empty() && curr_time >= wkld.get(0).unwrap().arrival {
            let p2 = wkld.pop_front().unwrap();
            duration.push(p2);
        }
    }

    complete
}

// runs STCF algorithm
// input: borrowed Vector of Processes, output: Vector of Processes
pub fn stcf(workload: &Vec<Process>) -> Vec<Process> {
    let wkld = workload.clone();
    let mut complete : Vec<Process> = Vec::new();
    let mut todo = binary_heap_plus::BinaryHeap::from_vec_cmp(wkld.to_vec(), 
        |p1: &Process, p2 :&Process| p2.arrival.partial_cmp(&p1.arrival).unwrap());
    let mut in_progress_dur = binary_heap_plus::BinaryHeap::from_vec_cmp(vec![], 
        |p1: &Process, p2 :&Process| p2.duration.partial_cmp(&p1.duration).unwrap());

    let mut curr_time = todo.peek().unwrap().arrival;
    let mut init = todo.peek().unwrap().clone();
    init.first_run = -1.0;
    in_progress_dur.push(init);
    todo.pop();

    while !in_progress_dur.is_empty() {
        while !todo.is_empty() && curr_time == todo.peek().unwrap().arrival {
            let p = todo.pop().unwrap();
            let mut p_clone = p.clone();
            p_clone.first_run = -1.0;
            in_progress_dur.push(p_clone);
        }

        let mut p = in_progress_dur.pop().unwrap();
        if p.first_run == -1.0 {
            p.first_run = curr_time;
        }
        p.duration -= 1.0;
        curr_time += 1.0;

        if p.duration == 0.0 {
            p.completion = curr_time;
            complete.push(p);
        } else {
            in_progress_dur.push(p);
        }
    }

    complete
}

// runs RR algorithm
// input: borrowed Vector of Processes, output: Vector of Processes
pub fn rr(workload: &Vec<Process>) -> Vec<Process> {
    let wkld = workload.clone();
    let mut complete : Vec<Process> = Vec::new();
    let mut todo = binary_heap_plus::BinaryHeap::from_vec_cmp(wkld.to_vec(), 
        |p1: &Process, p2 :&Process| p2.arrival.partial_cmp(&p1.arrival).unwrap());
    let mut in_progress: VecDeque<Process> = VecDeque::new();
    let mut curr_time = todo.peek().unwrap().arrival;
    let mut init = todo.peek().unwrap().clone();
    init.first_run = -1.0;
    in_progress.push_back(init);
    todo.pop();

    // Note, pop_front() returns an Option enum, which can either be Some or None. None is similar to null while avoiding having null
    // this check that that pop_front() pops a Some type and not a None type
    while let Some(mut p) = in_progress.pop_front() {
        while !todo.is_empty() && curr_time == todo.peek().unwrap().arrival {
            let p = todo.pop().unwrap();
            let p = Process {
                first_run: -1.0,
                ..p // Note here, this essentially fills in the rest of the fields with the fields from the original p
            };
            in_progress.push_back(p);
        }

        if p.first_run == -1.0 {
            p.first_run = curr_time;
        }
        p.remaining_time -= 1.0;
        curr_time += 1.0;

        if p.remaining_time == 0.0 {
            p.completion = curr_time;
            complete.push(p);
        } else {
            in_progress.push_back(p);
        }
    }

    complete
}

// runs MLFQ algorithm
// input: borrowed Vector of Processes, output: Vector of Processes
pub fn mlfq(workload: &Vec<Process>) -> Vec<Process> {
    let wkld = workload.clone();
    let mut todo = binary_heap_plus::BinaryHeap::from_vec_cmp(wkld.to_vec(), 
        |p1: &Process, p2 :&Process| p2.arrival.partial_cmp(&p1.arrival).unwrap());

    // creates size 4 array of VectorDeques, four levels in the MLFQ
    let mut mlfq : [VecDeque<Process>; 4]= [VecDeque::new(), VecDeque::new(), VecDeque::new(),VecDeque::new()];
    let mut complete : Vec<Process> = Vec::new();
    let mut curr_time = todo.peek().unwrap().arrival;
    let mut init = todo.pop().unwrap().clone();
    let mut counter = 1;
    init.first_run = -1.0;
    mlfq[0].push_back(init);
    let mut curr_queue = 0;

    // while we still have processes left to finish
    while complete.len() != wkld.len() {

        // boosting mechanism, go through all levels and elevate to first level
        if counter % BOOSTTIME == 0{
            let mut j = 1;
            while j < mlfq.len(){
                while !mlfq[j].is_empty(){
                    let p = mlfq[j].pop_front().unwrap();
                    mlfq[0].push_back(p);
                }
                j += 1;
            }
            curr_queue=0;
        }

        // mechanism to read in processes if the current time matches the arrival time of that process
        while !todo.is_empty() && curr_time == todo.peek().unwrap().arrival {
            let p_add = todo.pop().unwrap();
            let p_add = Process {
                first_run: -1.0,
                ..p_add
            };
            mlfq[0].push_front(p_add);
            curr_queue = 0;
        }
        
        // printing functionality
        if MLFQPRINTING {
            println!("{counter}");
            let mut pr = 0;
            for vd in mlfq.iter(){
                println!("time: {counter} MLFQ Level {pr}: {:?}",vd);
                pr += 1;
            }
        }

        // Putting a process onto the cpu for a time quantum of 1 (maybe think of not as a second or measure of time but as a CPU cycle)
        let mut p = mlfq[curr_queue].pop_front().unwrap();
        if p.first_run == -1.0 {
            p.first_run = curr_time;
        }
        p.remaining_time -= 1.0;
        curr_time += 1.0;

        // mechanism to change the current queue pointer
        let mut changed = false;
        if mlfq[curr_queue].is_empty() && curr_queue+1 < mlfq.len(){
            curr_queue = (curr_queue+1)%mlfq.len();
            changed = true;
        }

        // mechanism to decide where put a process after taking off CPU, if its done put it in complete
        // if not, if we have already changed our current queue pointer, put process on the current queue pointer
        // avoids skipping a level
        // also if we are currently at the last level, dont go further
        // otherwise, put on the next level
        if p.remaining_time == 0.0 {
            p.completion = curr_time;
            complete.push(p);
        } else {
            let z = if changed  || curr_queue+1 >= mlfq.len() {curr_queue} else {curr_queue+1};
            mlfq[z].push_back(p);
        }
        counter += 1;
    }
    complete
}
