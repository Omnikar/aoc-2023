use std::collections::{HashMap, VecDeque};

#[derive(Clone, Copy)]
struct Message<'a> {
    sender: &'a str,
    high: bool,
}
impl<'a> Message<'a> {
    fn new(sender: &'a str, high: bool) -> Self {
        Self { sender, high }
    }
}

trait Module: std::fmt::Debug {
    fn recv(&mut self, msg: Message) -> Option<bool>;
}

#[derive(Clone, Copy, Debug)]
struct Broadcaster;
impl Module for Broadcaster {
    fn recv(&mut self, msg: Message) -> Option<bool> {
        Some(msg.high)
    }
}

#[derive(Clone, Copy, Debug)]
struct FlipFlop {
    state: bool,
}
impl Module for FlipFlop {
    fn recv(&mut self, msg: Message) -> Option<bool> {
        (!msg.high).then(|| {
            self.state ^= true;
            self.state
        })
    }
}

#[derive(Clone, Debug)]
struct Conjunction<'a> {
    memory: HashMap<&'a str, bool>,
}
impl<'a> Module for Conjunction<'a> {
    fn recv(&mut self, msg: Message) -> Option<bool> {
        *self.memory.get_mut(msg.sender).unwrap() = msg.high;
        self.memory
            .values()
            .copied()
            .reduce(|a, b| a && b)
            .map(|a| !a)
    }
}

type ParseOut<'a> = HashMap<&'a str, (Box<dyn Module + 'a>, Box<[&'a str]>)>;
fn parse<'a>(input: &'a str) -> ParseOut<'a> {
    let mut modules = Vec::new();

    for line in input.lines() {
        let (sender_s, receivers_s) = line.split_once(" -> ").unwrap();
        let sender_t = sender_s.chars().next().unwrap();
        let sender = &sender_s[1..];

        let receivers: Box<[&'a str]> = receivers_s.split(", ").collect();

        modules.push((sender, (sender_t, receivers)));
    }

    let lookup: Vec<_> = modules.iter().map(|(k, v)| (*k, v.1.clone())).collect();

    modules
        .into_iter()
        .map(|(s, (t, r))| {
            let module: Box<dyn Module> = match t {
                'b' => Box::new(Broadcaster),
                '%' => Box::new(FlipFlop { state: false }),
                '&' => {
                    Box::new(Conjunction {
                        memory: lookup
                            .iter()
                            .filter(|(_, r)| r.contains(&s))
                            .map(|v| (v.0, false))
                            .collect(),
                    })
                }
                _ => unreachable!(),
            };
            (s, (module, r))
        })
        .collect()
}

fn calc(input: &str, part2: bool) -> u32 {
    let mut modules = parse(input);

    let mut counts = [0, 0];

    for press in 0.. {
        if !part2 && press >= 1000 {
            break;
        }
        if part2 && press % 1000 == 0 {
            println!("checking {press}");
        }
        let mut msg_queue = VecDeque::new();
        msg_queue.push_back(("roadcaster", Message::new("button", false)));
        while let Some((r, msg)) = msg_queue.pop_front() {
            if part2 && r == "rx" && !msg.high {
                return press + 1;
            }
            // println!(
            //     "{} -{}-> {r}",
            //     msg.sender,
            //     ["low", "high"][msg.high as usize],
            // );
            counts[msg.high as usize] += 1;
            let Some((module, receivers)) = modules.get_mut(&r) else {
                continue;
            };
            if let Some(new_high) = module.recv(msg) {
                let new_msg = Message::new(r, new_high);
                msg_queue.extend(receivers.iter().map(|&r| (r, new_msg)));
            }
        }
    }

    counts[0] * counts[1]
}

fn part1(input: &str) -> u32 {
    calc(input, false)
}

// Brute force, too slow to actually work
fn part2(input: &str) -> u32 {
    calc(input, true)
}

crate::parts!(part1 part2);
