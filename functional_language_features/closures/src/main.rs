use std::thread;
use std::time::Duration;

fn simulate_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly....");
    thread::sleep(Duration::from_secs(2));

    intensity
}

fn main() {
    let simulated_intensity = 10;
    let simulated_rand_number = 7;

    println!("Running with ineffieint implementation");
    // Running with ineffieint implementation
    generate_workout_inefficient(simulated_intensity, simulated_rand_number);
    // calculating slowly....
    // Today, do 10 pushups!
    // calculating slowly....
    // Next, do 10 situps!

    println!("Running with ineffieint implementation2");
    // Running with ineffieint implementation2
    generate_workout_inefficient2(simulated_intensity, simulated_rand_number);
    // calculating slowly....
    // Today, do 10 pushups!
    // Next, do 10 situps!
    // Implementation using closure

    println!("Running with inefficient implementation with closure");
    // Running with inefficient implementation with closure
    generate_workout_closure_inefficient(simulated_intensity, simulated_rand_number);
    // calculating slowly....
    // Today, do 10 pushups!
    // calculating slowly....
    // Next, do 10 situps!

    println!("Running with efficient impelementation with closure");
    // Running with efficient impelementation with closure
    generate_workout_closure_with_cacher(simulated_intensity, simulated_rand_number);
    // calculating slowly....
    // Today, do 10 pushups!
    // Next, do 10 situps!

    // It only calcullates it once
}

// Here we are doing the expenisve task again and again even though we need to calculate it once
fn generate_workout_inefficient(intensity: u32, rand_number: u32) {
    if intensity < 25 {
        // Specially here we are calculating it twice even though it returns the same value
        println!(
            "Today, do {} pushups!",
            simulate_expensive_calculation(intensity)
        );
        println!(
            "Next, do {} situps!",
            simulate_expensive_calculation(intensity)
        );
    } else {
        if rand_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                simulate_expensive_calculation(intensity)
            );
        }
    }
}

// You may argue to make the simulated result once and then we use it again and again.
fn generate_workout_inefficient2(intensity: u32, rand_number: u32) {
    let expensive_result = simulate_expensive_calculation(intensity);
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result);
        println!("Next, do {} situps!", expensive_result);
    } else {
        // But here we do not need expensive result in this case but we are calculating any way
        if rand_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result);
        }
    }
}

// Inefficinet Solution to with closure with the implementation 1 problem
fn generate_workout_closure_inefficient(intensity: u32, rand_number: u32) {
    let expensive_closure = |num| {
        println!("calculating slowly....");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        // But here we do not need expensive result in this case but we are calculating any way
        if rand_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure(intensity));
        }
    }
}

// Solution with Struct and closure
// In case where our intesity varies, we need to maintain a hash function, but in this situation
// the intensity is fixed for the generate_workout period so we only need to cache it once
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);

                v
            }
        }
    }
}
fn generate_workout_closure_with_cacher(intensity: u32, rand_number: u32) {
    let mut cached_result = Cacher::new(|num| {
        println!("calculating slowly....");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", cached_result.value(intensity));
        println!("Next, do {} situps!", cached_result.value(intensity));
    } else {
        // But here we do not need expensive result in this case but we are calculating any way
        if rand_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", cached_result.value(intensity));
        }
    }
}
