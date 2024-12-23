use std::{collections::HashSet, env, fs};

fn parse(input: &str) -> HashSet<(&str, &str)> {
    let mut connections: HashSet<(&str, &str)> = HashSet::new();

    for line in input.lines() {
        let pair = line.trim().split("-").map(|p| p).collect::<Vec<_>>();
        connections.insert((pair[0], pair[1]));
        connections.insert((pair[1], pair[0]));
    }

    connections
}

fn solve(connections: &HashSet<(&str, &str)>) -> u32 {
    let computers: HashSet<&str> = connections
        .iter()
        .flat_map(|(c1, c2)| vec![*c1, *c2])
        .collect();

    let mut lan_party: HashSet<Vec<&str>> = HashSet::new();

    for &(c1, c2) in connections.iter() {
        if !c1.starts_with("t") {
            continue;
        }

        for &c3 in computers.iter() {
            if c3 == c1 || c3 == c2 {
                continue;
            }

            if connections.contains(&(c1, c3)) && connections.contains(&(c2, c3)) {
                let mut party = vec![c1, c2, c3];
                party.sort();
                lan_party.insert(party);
            }
        }
    }

    lan_party.len() as u32
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = args
        .get(1)
        .unwrap_or(&"src/bin/input.txt".to_string())
        .to_string();
    let input = fs::read_to_string(path).expect("to read file");
    let connections = parse(&input);
    let result = solve(&connections);

    println!("Lan Parties: {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves() {
        let input = "kh-tc
        qp-kh
        de-cg
        ka-co
        yn-aq
        qp-ub
        cg-tb
        vc-aq
        tb-ka
        wh-tc
        yn-cg
        kh-ub
        ta-co
        de-co
        tc-td
        tb-wq
        wh-td
        ta-ka
        td-qp
        aq-cg
        wq-ub
        ub-vc
        de-ta
        wq-aq
        wq-vc
        wh-yn
        ka-de
        kh-ta
        co-tc
        wh-qp
        tb-vc
        td-yn"
            .to_string();
        let conns = parse(&input);

        assert_eq!(solve(&conns), 7);
    }
}
