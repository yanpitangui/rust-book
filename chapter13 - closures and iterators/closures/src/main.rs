use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}



fn main() {
    let simulated_intensity = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_intensity, simulated_random_number);

    let x = 4;
    let equal_to_x = | z | z == x;

    let y = 4;
    assert!(equal_to_x(y));
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut cached_result = Cacher::new(|num: u32| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    let example_closure = |x: &str| {
        println!("Olá, {}", x);
        String::from(x)
    };
    let mut cached = Cacher::new(example_closure);
    cached.value("Olá");
    cached.value("Olá");
    cached.value("Olá");
    cached.value("Olá");
    cached.value("Olá");
    if intensity < 25 {
        println!("Today, do {} pushups!", cached_result.value(intensity));
        println!("Next, do {} situps!", cached_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                cached_result.value(intensity)
            )
        }
    }
}

struct Cacher<TFn, K, V>
 where K: Eq + Hash + Copy, TFn: Fn(K) -> V
{
    calculation: TFn,
    values: Box<HashMap<K, Box<V>>>
}

impl<TFn, K, V> Cacher<TFn, K, V>
where K: Eq + Hash + Copy, TFn: Fn(K) -> V
{
    fn new(calculation: TFn) -> Cacher<TFn, K, V> {
        Cacher {
            calculation,
            values: Box::new(HashMap::new())
        }
    }

    fn value(&mut self, arg: K) -> &V {
        if !self.values.contains_key(&arg) {
            self.values.insert(arg, Box::new((self.calculation)(arg.clone())));
        }
        return self.values.get(&arg).unwrap()

    }
}