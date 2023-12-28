use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_value = 10;
    let simulated_random_number = 6;

    generate_exercises(simulated_user_value, simulated_random_number)

}

fn generate_exercises(intensity :u64, random_number :u64) {
    if intensity < 25 {
        println!("Today make {} pushups",
                 simulate_heavy_calc(intensity));
        println!("then, do {} abs",
            simulate_heavy_calc(intensity));
    }else {
        if random_number == 3 {
            println!("It's your day off today, stay hydrated");
        }else {
            println!("Today, you have to run {} minutes",
                simulate_heavy_calc(intensity));
        }
    }
}


fn simulate_heavy_calc(intensity: u64)->u64{
    println!("Heavy calculating...");
    thread::sleep(Duration::from_secs(2));
    intensity
}