use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_value = 10;
    let simulated_random_number = 6;

    generate_exercises(simulated_user_value, simulated_random_number)

}

fn generate_exercises(intensity :u64, random_number :u64) {
    let mut heavy_calc = Cache::new(|intensity:u64| ->u64{
        println!("Heavy calculating...");
        thread::sleep(Duration::from_secs(2));
        intensity
    });

    if intensity < 25 {
        println!("Today make {} pushups",heavy_calc.value(intensity));
        println!("then, do {} abs",heavy_calc.value(intensity));
    }else {
        if random_number == 3 {
            println!("It's your day off today, stay hydrated");
        }else {
            println!("Today, you have to run {} minutes",heavy_calc.value(intensity));
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
            Some(v)=>v,
            None=>{
                let v = (self.calc)(arg);
                self.value = Some(v);
                v
            }
        }
    }

}



//make an error because cache supposes that we use the same value
/*
#[test]
fn call_cache_with_different_value() {
    let mut c = Cache::new(|a| a);

    let v1 = c.valeur(1);
    let v2 = c.valeur(2);

    assert_eq!(v2, 2);
}
 */