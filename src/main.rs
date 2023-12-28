use std::thread;
use std::time::Duration;

fn main() {
    simulate_heavy_calc(40);
}


fn simulate_heavy_calc(intensity: u64)->u64{
    println!("Heavy calculating...");
    thread::sleep(Duration::from_secs(2));
    intensity
}