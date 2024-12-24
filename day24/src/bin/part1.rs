use std::{collections::HashMap, env, fs};

#[derive(Debug, PartialEq)]
enum Logic {
    And,
    Or,
    Xor,
}

#[derive(Debug, PartialEq)]
struct Gate {
    in0: String,
    in1: String,
    logic: Logic,
    out: String,
}

fn parse(input: String) -> (HashMap<String, u8>, Vec<Gate>) {
    let mut wires = HashMap::new();
    let mut gates = Vec::new();

    let sections = input.trim().split("\n\n").collect::<Vec<_>>();

    for wire in sections[0].lines() {
        let wire = wire.trim();
        let mut parts = wire.split(":");

        let net = parts.next().unwrap();
        let net = net.trim().to_string();

        let value = parts.next().unwrap();
        let value = if value.trim() == "1" { 1 } else { 0 };

        wires.entry(net).or_insert(value);
    }

    for line in sections[1].lines() {
        let line = line.trim();
        let mut parts = line.split("->");

        let input = parts.next().unwrap();
        let mut input = input.trim().split_whitespace();

        let in0 = input.next().unwrap();
        let in0 = in0.trim().to_string();

        let logic = input.next().unwrap();
        let logic = match logic.trim() {
            "AND" => Logic::And,
            "OR" => Logic::Or,
            "XOR" => Logic::Xor,
            _ => unreachable!(),
        };

        let in1 = input.next().unwrap();
        let in1 = in1.trim().to_string();

        let out = parts.next().unwrap();
        let out = out.trim().to_string();

        gates.push(Gate {
            in0,
            in1,
            logic,
            out,
        });
    }

    (wires, gates)
}

fn solve(inputs: HashMap<String, u8>, gates: Vec<Gate>) -> u64 {
    let mut wires = inputs.clone();
    let mut complete = false;

    while !complete {
        complete = true;

        for gate in gates.iter() {
            if !wires.contains_key(&gate.in0) || !wires.contains_key(&gate.in1) {
                complete = false;
                continue;
            }

            if wires.contains_key(&gate.out) {
                // We've already seen this one
                continue;
            }

            let &in0 = wires.get(&gate.in0).unwrap();
            let &in1 = wires.get(&gate.in1).unwrap();
            let result: u8 = match gate.logic {
                Logic::And => {
                    if in0 == 1 && in1 == 1 {
                        1
                    } else {
                        0
                    }
                }
                Logic::Or => {
                    if in0 == 1 || in1 == 1 {
                        1
                    } else {
                        0
                    }
                }
                Logic::Xor => {
                    if (in0 == 1 && in1 == 0) || (in0 == 0 && in1 == 1) {
                        1
                    } else {
                        0
                    }
                }
            };

            wires.insert(gate.out.clone(), result);
        }
    }

    let mut output = wires
        .iter()
        .filter_map(|(wire, &val)| {
            if wire.starts_with("z") {
                Some((wire, val))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    output.sort_by(|a, b| a.0.cmp(&b.0));

    output
        .into_iter()
        .enumerate()
        .map(|(i, (_, bit))| (bit as u64) * (1 << i))
        .sum()
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let path = args
        .get(1)
        .unwrap_or(&"src/bin/input.txt".to_string())
        .to_string();
    let input = fs::read_to_string(path).expect("to read file");
    let (wires, gates) = parse(input);
    let result = solve(wires, gates);

    println!("{result}");
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_parses() {
        let input = "x00: 1
        x01: 1
        x02: 1
        y00: 0
        y01: 1
        y02: 0

        x00 AND y00 -> z00
        x01 XOR y01 -> z01
        x02 OR y02 -> z02"
            .to_string();
        let (inputs, connections) = parse(input);

        assert_eq!(
            inputs,
            HashMap::from([
                ("x00".to_string(), 1),
                ("x01".to_string(), 1),
                ("x02".to_string(), 1),
                ("y00".to_string(), 0),
                ("y01".to_string(), 1),
                ("y02".to_string(), 0),
            ])
        );
        assert_eq!(connections.len(), 3);
        assert_eq!(
            connections,
            vec![
                Gate {
                    in0: "x00".to_string(),
                    in1: "y00".to_string(),
                    logic: Logic::And,
                    out: "z00".to_string()
                },
                Gate {
                    in0: "x01".to_string(),
                    in1: "y01".to_string(),
                    logic: Logic::Xor,
                    out: "z01".to_string()
                },
                Gate {
                    in0: "x02".to_string(),
                    in1: "y02".to_string(),
                    logic: Logic::Or,
                    out: "z02".to_string()
                },
            ]
        )
    }

    #[test]
    fn it_solves() {
        let input = "x00: 1
        x01: 1
        x02: 1
        y00: 0
        y01: 1
        y02: 0

        x00 AND y00 -> z00
        x01 XOR y01 -> z01
        x02 OR y02 -> z02"
            .to_string();
        let (inputs, connections) = parse(input);
        let result = solve(inputs, connections);

        assert_eq!(result, 4);

        let input = "x00: 1
        x01: 0
        x02: 1
        x03: 1
        x04: 0
        y00: 1
        y01: 1
        y02: 1
        y03: 1
        y04: 1

        ntg XOR fgs -> mjb
        y02 OR x01 -> tnw
        kwq OR kpj -> z05
        x00 OR x03 -> fst
        tgd XOR rvg -> z01
        vdt OR tnw -> bfw
        bfw AND frj -> z10
        ffh OR nrd -> bqk
        y00 AND y03 -> djm
        y03 OR y00 -> psh
        bqk OR frj -> z08
        tnw OR fst -> frj
        gnj AND tgd -> z11
        bfw XOR mjb -> z00
        x03 OR x00 -> vdt
        gnj AND wpb -> z02
        x04 AND y00 -> kjc
        djm OR pbm -> qhw
        nrd AND vdt -> hwm
        kjc AND fst -> rvg
        y04 OR y02 -> fgs
        y01 AND x02 -> pbm
        ntg OR kjc -> kwq
        psh XOR fgs -> tgd
        qhw XOR tgd -> z09
        pbm OR djm -> kpj
        x03 XOR y03 -> ffh
        x00 XOR y04 -> ntg
        bfw OR bqk -> z06
        nrd XOR fgs -> wpb
        frj XOR qhw -> z04
        bqk OR frj -> z07
        y03 OR x01 -> nrd
        hwm AND bqk -> z03
        tgd XOR rvg -> z12
        tnw OR pbm -> gnj"
            .to_string();
        let (inputs, connections) = parse(input);
        let result = solve(inputs, connections);

        assert_eq!(result, 2024);
    }
}
