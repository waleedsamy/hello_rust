#![allow(dead_code)]
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

impl Shoe {
    pub fn filter(shoes: Vec<Shoe>, size: u32) -> Vec<Shoe> {
        shoes.into_iter().filter(|x| x.size == size).collect()
    }
}

#[derive(Debug)]
struct Counter {
    current: usize,
    max: usize,
}

impl Counter {
    pub fn new(max: usize) -> Counter {
        Counter {
            current: 0,
            max: max,
        }
    }
}

impl Iterator for Counter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.max {
            self.current += 1;
            Some(self.current)
        } else {
            None
        }
    }
}

fn main() {
    let v1 = vec![1, 2, 3, 4];
    let mut v1_itr = v1.iter();
    assert_eq!(Some(&1), v1_itr.next());
    assert_eq!(Some(&2), v1_itr.next());
    assert_eq!(Some(&3), v1_itr.next());
    assert_eq!(Some(&4), v1_itr.next());
    assert_eq!(None, v1_itr.next());
    println!("v1 {:?}", v1);

    let v2 = vec![1, 2, 3, 4];
    for i in v2 {
        println!("v2 {:}", i);
    }
    // println!("v2 {:?}", v2); // already moved into the for loop (use v2.iter())

    let v1_sum = v1.iter().sum::<i32>();
    println!("v1 sum: {}", v1_sum);

    let v1_by_10_sum = v1.iter().map(|e| e * 10).collect::<Vec<i32>>();
    println!("v1_by_10_sum: {:?}", v1_by_10_sum);

    let v1_filtered: Vec<&i32> = v1.iter().filter(|x| **x > 2).collect();
    println!("v1_filtered: {:?}", v1_filtered);

    let counter_to_10 = Counter::new(10);
    println!(
        "counter_to_10: {:?}",
        counter_to_10.into_iter().collect::<Vec<usize>>()
    );

    let _sum = Counter::new(10)
        .zip(Counter::new(5))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum::<usize>();
    println!("_sum {}", _sum);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn filter_shoes() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: "a".to_string(),
            },
            Shoe {
                size: 20,
                style: "b".to_string(),
            },
            Shoe {
                size: 30,
                style: "c".to_string(),
            },
        ];

        assert_eq!(
            vec![Shoe {
                size: 10,
                style: "a".to_string(),
            }],
            Shoe::filter(shoes, 10)
        )
    }

    #[test]
    fn test_counter() {
        let mut c1 = Counter::new(5);
        assert_eq!(Some(1), c1.next());
        assert_eq!(Some(2), c1.next());
        assert_eq!(Some(3), c1.next());
        assert_eq!(Some(4), c1.next());
        assert_eq!(Some(5), c1.next());
        assert_eq!(None, c1.next());
    }
}
