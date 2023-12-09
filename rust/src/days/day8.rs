use std::collections::HashMap;

fn parse(input: &str) -> (impl Iterator<Item = usize>, HashMap<&str, [&str; 2]>) {
    let (dirs_s, map_s) = input.split_once("\n\n").unwrap();

    let dirs_vec = dirs_s.chars().map(|s| (s == 'R') as usize).collect();
    struct LoopingIter<T: Clone>(Vec<T>, usize);
    impl<T: Clone> Iterator for LoopingIter<T> {
        type Item = T;
        fn next(&mut self) -> Option<Self::Item> {
            let item = self.0[self.1].clone();
            self.1 += 1;
            self.1 %= self.0.len();
            Some(item)
        }
    }
    let dirs = LoopingIter(dirs_vec, 0);

    let map = map_s
        .lines()
        .map(|l| {
            let (key, val_s) = l.split_once(" = ").unwrap();
            let val =
                val_s
                    .strip_prefix('(')
                    .and_then(|s| s.strip_suffix(')'))
                    .and_then(|s| s.split_once(", "))
                    .unwrap()
                    .into();
            (key, val)
        })
        .collect();

    (dirs, map)
}

fn part1(input: &str) -> u32 {
    let (mut dirs, map) = parse(input);

    let mut cur = "AAA";
    let mut steps = 0;
    loop {
        let i = dirs.next().unwrap();
        cur = map[cur][i];
        steps += 1;
        if cur == "ZZZ" {
            break;
        }
    }

    steps
}

// Brute force, too slow to actually work
fn part2(input: &str) -> u32 {
    let (mut dirs, map) = parse(input);

    let mut curs = map
        .keys()
        .copied()
        .filter(|s| s.ends_with('A'))
        .collect::<Vec<_>>();
    let mut steps = 0;
    loop {
        let i = dirs.next().unwrap();
        curs.iter_mut().for_each(|cur| *cur = map[cur][i]);
        steps += 1;
        if curs.iter().all(|cur| cur.ends_with('Z')) {
            break;
        }
    }

    steps
}

crate::parts!(part1 part2);
