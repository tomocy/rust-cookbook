use std::collections::HashMap;
use std::hash;
use std::thread;
use std::time;

fn main() {
    let intensity = 10;
    let random = 4;

    generate_work(intensity, random);
}

fn generate_work(intensity: u32, random: u32) {
    let mut calculate_target_level = Cacher::new(simulated_expensive_calculation);

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            calculate_target_level.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            calculate_target_level.value(intensity)
        );

        return;
    }

    if random == 3 {
        println!("Take a break today! Remember to stay hydrated!");
        return;
    }

    println!(
        "Today, run for {} minutes!",
        calculate_target_level.value(intensity)
    );
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(time::Duration::from_secs(2));
    intensity
}

struct Cacher<T, U>
where
    T: Fn(U) -> U,
    U: Eq + Copy + hash::Hash,
{
    calculate: T,
    values: HashMap<U, U>,
}

impl<T, U> Cacher<T, U>
where
    T: Fn(U) -> U,
    U: Eq + Copy + hash::Hash,
{
    fn new(calculate: T) -> Cacher<T, U> {
        Cacher {
            calculate: calculate,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, x: U) -> U {
        match self.values.get(&x) {
            Some(v) => *v,
            None => {
                let v = (self.calculate)(x);
                self.values.insert(x, v);
                v
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cache_different_values() {
        let mut cacher = Cacher::new(|x| x);
        cacher.value("1");
        cacher.value("2");
        assert_eq!("1", cacher.value("1"));
        assert_eq!("2", cacher.value("2"));
    }
}
