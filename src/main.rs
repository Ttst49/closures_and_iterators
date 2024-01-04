use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_value = 10;
    let simulated_random_number = 6;

    generate_exercises(simulated_user_value, simulated_random_number)

}

fn generate_exercises(intensity :u64, random_number :u64) {
    let heavy_calc = |intensity:u64| ->u64{
        println!("Heavy calculating...");
        thread::sleep(Duration::from_secs(2));
        intensity
    };

    if intensity < 25 {
        println!("Today make {} pushups",heavy_calc(intensity));
        println!("then, do {} abs",heavy_calc(intensity));
    }else {
        if random_number == 3 {
            println!("It's your day off today, stay hydrated");
        }else {
            println!("Today, you have to run {} minutes",heavy_calc(intensity));
        }
    }

    //Example of closure, you can only have one return type per closure
    //this result into an error in the compiler
    /*
    let test_closure = |x| x;

    test_closure(12);
    test_closure("12");
     */

}


struct Cache<T>
where T:Fn(u64)->u64, {
    calc:T,
    value: Option<u64>
}

impl<T> Cache<T>
    where T:Fn(u64)->u64{

    fn new(calc:T)->Cache<T>{
        Cache{
            calc,
            value: None
        }
    }

    fn value(&mut self, arg:u64)->u64{
        match self.value{
            Some(V)=>V,
            None=>{
                let v = (self.calc)(arg);
                self.value = Some(v)
            }
        }
    }

}