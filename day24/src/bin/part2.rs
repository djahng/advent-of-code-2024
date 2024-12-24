use std::{collections::HashMap, env, fs};

#[derive(Debug, PartialEq, Clone, Copy)]
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

fn solve(_inputs: HashMap<String, u8>, gates: Vec<Gate>) -> String {
    // We need to make this thing look like a full adder
    let mut wrong_wires: Vec<String> = Vec::new();
    let mut wire_map: HashMap<String, Vec<(Logic, String)>> = HashMap::new();

    for gate in gates.iter() {
        wire_map
            .entry(gate.in0.clone())
            .or_insert_with(Vec::new)
            .push((gate.logic, gate.out.clone()));
        wire_map
            .entry(gate.in1.clone())
            .or_insert_with(Vec::new)
            .push((gate.logic, gate.out.clone()));
    }

    let mut outputs = gates
        .iter()
        .filter(|g| g.out.starts_with("z"))
        .collect::<Vec<_>>();
    outputs.sort_by(|a, b| b.out.cmp(&a.out));
    let output_msb = outputs.first().unwrap().out.clone();

    for gate in gates.iter() {
        let chained_ops = wire_map.get(&gate.out);
        let chained_ops_contain =
            |op| chained_ops.is_some_and(|v| v.iter().find(|a| a.0 == op).is_some());
        let has_chained_xor = chained_ops_contain(Logic::Xor);
        let has_chained_and = chained_ops_contain(Logic::And);
        let has_chained_or = chained_ops_contain(Logic::Or);
        let takes_first_input = gate.in0.ends_with("00") && gate.in1.ends_with("00");
        let takes_input_bit = (gate.in0.starts_with("x") && gate.in1.starts_with("y"))
            || (gate.in0.starts_with("y") && gate.in1.starts_with("x"));
        let outputs_bit = gate.out.starts_with("z");
        let outputs_last_bit = gate.out == output_msb;

        let valid = match gate.logic {
            Logic::And => {
                if has_chained_or {
                    true
                } else if takes_first_input {
                    true
                } else {
                    false
                }
            }
            Logic::Or => {
                if outputs_last_bit || (has_chained_and && has_chained_xor) {
                    true
                } else {
                    false
                }
            }
            Logic::Xor => {
                if !takes_input_bit && outputs_bit {
                    true
                } else if takes_input_bit && has_chained_xor {
                    true
                } else if takes_first_input && outputs_bit {
                    true
                } else {
                    false
                }
            }
        };
        if !valid {
            wrong_wires.push(gate.out.clone());
        }
    }
    wrong_wires.sort();
    wrong_wires.join(",").to_string()
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
}
