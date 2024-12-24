use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::fs;
use std::time::Instant;

#[derive(Clone, Eq, PartialEq, Hash, Copy, Debug)]
enum Operator {
    And,
    Or,
    Xor,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Initialization {
    name: String,
    value: bool,
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Gate {
    left: String,
    operator: Operator,
    right: String,
    output: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = args.get(1).expect("Input file is required");
    let input = parse_input(input_file);

    let now = Instant::now();
    let result = if args.contains(&"--part-2".to_string()) {
        part_2(input)
    } else {
        part_1(input)
    };
    let elapsed = now.elapsed();
    println!("Answer: {}", result);
    println!("Executed in {:?}", elapsed);
}

fn parse_input(input_file: &str) -> (Vec<Initialization>, Vec<Gate>) {
    let binding = fs::read_to_string(input_file).expect("Failed to read file");
    let (initializations, gates) = binding.split_once("\n\n").unwrap();

    let initialization_regex = Regex::new(r"^(\w+): (\d+)$").unwrap();
    let initializations = initializations
        .lines()
        .map(|l| {
            let captures = initialization_regex.captures(l).unwrap();
            let name = captures.get(1).unwrap().as_str().to_string();
            let value = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
            Initialization {
                name,
                value: if value == 1 { true } else { false },
            }
        })
        .collect();

    let gate_regex = Regex::new(r"^(\w+) (AND|OR|XOR) (\w+) -> (\w+)$").unwrap();
    let gates = gates
        .lines()
        .map(|l| {
            let captures = gate_regex.captures(l).unwrap();
            let left = captures.get(1).unwrap().as_str().to_string();
            let operator = match captures.get(2).unwrap().as_str() {
                "AND" => Operator::And,
                "OR" => Operator::Or,
                "XOR" => Operator::Xor,
                _ => panic!("Invalid operator: {}", captures.get(2).unwrap().as_str()),
            };
            let right = captures.get(3).unwrap().as_str().to_string();
            let output = captures.get(4).unwrap().as_str().to_string();
            Gate {
                left,
                operator,
                right,
                output,
            }
        })
        .collect();

    (initializations, gates)
}

fn part_1((initializations, gates): (Vec<Initialization>, Vec<Gate>)) -> String {
    let mut values: HashMap<String, bool> = HashMap::new();
    for initialization in initializations {
        values.insert(initialization.name, initialization.value);
    }

    let mut gates_to_evaluate = VecDeque::from(gates);
    while let Some(gate) = gates_to_evaluate.pop_front() {
        let left_value = values.get(&gate.left);
        let right_value = values.get(&gate.right);

        if left_value.is_none() || right_value.is_none() {
            gates_to_evaluate.push_back(gate);
            continue;
        }

        let left_value = *left_value.unwrap();
        let right_value = *right_value.unwrap();

        let result_value = match gate.operator {
            Operator::And => left_value && right_value,
            Operator::Or => left_value || right_value,
            Operator::Xor => left_value ^ right_value,
        };
        values.insert(gate.output, result_value);
    }

    let mut z: i64 = 0;
    for (key, value) in values {
        if key.starts_with("z") && value {
            z += 1 << key[1..].parse::<i64>().unwrap();
        }
    }
    z.to_string()
}

fn part_2((_, gates): (Vec<Initialization>, Vec<Gate>)) -> String {
    let mut usage: HashMap<(&String, Operator), &Gate> = HashMap::new();
    let mut originated_from: HashMap<&String, &Gate> = HashMap::new();
    gates.iter().for_each(|g| {
        usage.insert((&g.left, g.operator), g);
        usage.insert((&g.right, g.operator), g);
        originated_from.insert(&g.output, g);
    });

    // First sum iteration does not have carry-in
    // Checking if x0 ^ y0 output is z0
    let mut misplaced: HashSet<String> = HashSet::new();
    let x0_name = get_wire_name("x", 0);
    let y0_name = get_wire_name("y", 0);
    let z0_name = get_wire_name("z", 0);

    let gate_x_xor = usage
        .get(&(&x0_name, Operator::Xor))
        .expect("Did not find gate XOR for x0");
    let gate_y_xor = usage
        .get(&(&y0_name, Operator::Xor))
        .expect("Did not find gate XOR for y0");

    assert!(gate_x_xor == gate_y_xor);
    if gate_x_xor.output != z0_name {
        misplaced.insert(gate_x_xor.output.clone());
        misplaced.insert(z0_name.clone());
    }

    // Get x0 and y0 carry-out
    let gate_x_and = usage
        .get(&(&x0_name, Operator::And))
        .expect("Did not find gate AND for x0");
    let gate_y_and = usage
        .get(&(&y0_name, Operator::And))
        .expect("Did not find gate AND for y0");

    assert!(gate_x_and == gate_y_and);
    let mut d_prev = Some(gate_x_and.output.clone());

    for i in 1..=44 {
        println!("i: {}", i);
        let x_i = get_wire_name("x", i);
        let y_i = get_wire_name("y", i);
        let z_i = get_wire_name("z", i);

        // get a_i
        let gate_x_xor = usage
            .get(&(&x_i, Operator::Xor))
            .expect("Did not find gate XOR for x");
        let gate_y_xor = usage
            .get(&(&y_i, Operator::Xor))
            .expect("Did not find gate XOR for y");
        assert!(gate_x_xor == gate_y_xor);
        let xi_xor_yi = gate_x_xor;
        let possible_ai = &xi_xor_yi.output;

        // get b_i
        let gate_x_and = usage
            .get(&(&x_i, Operator::And))
            .expect("Did not find gate AND for x");
        let gate_y_and = usage
            .get(&(&y_i, Operator::And))
            .expect("Did not find gate AND for y");
        assert!(gate_x_and == gate_y_and);
        let xi_and_yi = gate_x_and;
        let possible_bi = &xi_and_yi.output;

        let bi_or = usage.get(&(possible_bi, Operator::Or));
        if bi_or.is_none() {
            misplaced.insert(possible_bi.clone());
        }

        let mut possible_ci: Option<String> = None;

        let ai_xor = usage.get(&(possible_ai, Operator::Xor));
        let ai_and = usage.get(&(possible_ai, Operator::And));
        if ai_xor.is_none() || ai_and.is_none() {
            misplaced.insert(possible_ai.clone());
        } else {
            let ai_xor = ai_xor.unwrap();
            let ai_and = ai_and.unwrap();

            if let Some(d_prev) = d_prev {
                let dprev_xor = usage.get(&(&d_prev, Operator::Xor));
                let dprev_and = usage.get(&(&d_prev, Operator::And));
                if dprev_xor.is_none() || dprev_and.is_none() {
                    misplaced.insert(d_prev.clone());
                } else {
                    let dprev_xor = dprev_xor.unwrap();
                    let dprev_and = dprev_and.unwrap();

                    if dprev_xor != ai_xor {
                        let z_comes_from = originated_from
                            .get(&z_i)
                            .expect("Did not find gate that originated z_i");
                        if possible_ai != &z_comes_from.left && possible_ai != &z_comes_from.right {
                            misplaced.insert(possible_ai.clone());
                        }
                        if d_prev != z_comes_from.left && d_prev != z_comes_from.right {
                            misplaced.insert(d_prev.clone());
                        }
                    } else {
                        if dprev_xor.output != z_i {
                            misplaced.insert(z_i.clone());
                            misplaced.insert(dprev_xor.output.clone());
                        }
                    }

                    if dprev_and != ai_and {
                        println!("dprev_and != ai_and");
                    } else {
                        possible_ci = Some(dprev_and.output.clone());
                    }
                }
            }
        }

        let bi_or = usage.get(&(&possible_bi, Operator::Or));

        if bi_or.is_none() {
            if possible_ci.is_none() {
                d_prev = None;
            } else {
                let ci = possible_ci.expect("ci is None");
                let ci_or = usage.get(&(&ci, Operator::Or));
                d_prev = Some(ci_or.expect("bi_or and ci_or are None").output.clone());
            }
        } else {
            d_prev = Some(bi_or.expect("bi_or and ci_or are None").output.clone());
        }
    }

    let mut result = Vec::from_iter(misplaced);
    result.sort();
    result.join(",")
}

fn get_wire_name(prefix: &str, index: i32) -> String {
    format!("{}{:02}", prefix, index)
}
