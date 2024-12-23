use std::{
    collections::{HashMap, HashSet},
    env, fs,
};

fn parse(input: &str) -> HashSet<(&str, &str)> {
    let mut connections: HashSet<(&str, &str)> = HashSet::new();

    for line in input.lines() {
        let pair = line.trim().split("-").map(|p| p).collect::<Vec<_>>();
        connections.insert((pair[0], pair[1]));
        connections.insert((pair[1], pair[0]));
    }

    connections
}

fn solve(connections: &HashSet<(&str, &str)>) -> String {
    let mut largest_party: Vec<&str> = Vec::new();

    // Create a lookup table of connections
    let mut lookup: HashMap<&str, Vec<&str>> = HashMap::new();
    for &(c1, c2) in connections.iter() {
        lookup.entry(c1).or_default().push(c2);
        lookup.entry(c2).or_default().push(c1);
    }

    for &first in lookup.keys() {
        let mut party = vec![first];

        for &next in lookup.get(first).unwrap() {
            if party.iter().all(|c| lookup.get(next).unwrap().contains(c)) {
                party.push(next);
            }
        }

        if party.len() > largest_party.len() {
            largest_party = party.clone();
        }
    }

    largest_party.sort();
    largest_party
        .into_iter()
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .join(",")
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

        assert_eq!(solve(&conns), "co,de,ka,ta".to_string());
    }
}
