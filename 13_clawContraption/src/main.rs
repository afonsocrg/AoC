use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fmt::Debug;
use std::fs;
use std::time::Instant;

struct Machine {
    ai: i64,
    aj: i64,
    bi: i64,
    bj: i64,
    ci: i64,
    cj: i64,
}

impl Debug for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "A: ({}, {}), B: ({}, {}), Prize: ({}, {})",
            self.ai, self.aj, self.bi, self.bj, self.ci, self.cj
        )
    }
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
    println!("{:?}", result);
    println!("Executed in {:?}", elapsed);
}

fn parse_input(input_file: &str) -> Vec<Machine> {
    let a_pattern = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
    let b_pattern = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap();
    let prize_pattern = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
    fs::read_to_string(input_file)
        .expect("Failed to read file")
        .split("\n\n")
        .map(|machine| {
            let mut lines = machine.lines();
            let a = lines.next().unwrap();
            let b = lines.next().unwrap();
            let prize = lines.next().unwrap();
            let a_matches = a_pattern.captures(a).unwrap();
            let b_matches = b_pattern.captures(b).unwrap();
            let prize_matches = prize_pattern.captures(prize).unwrap();
            Machine {
                ai: a_matches[1].parse().unwrap(),
                aj: a_matches[2].parse().unwrap(),
                bi: b_matches[1].parse().unwrap(),
                bj: b_matches[2].parse().unwrap(),
                ci: prize_matches[1].parse().unwrap(),
                cj: prize_matches[2].parse().unwrap(),
            }
        })
        .collect()
}

fn part_1(input: Vec<Machine>) -> u64 {
    input
        .iter()
        .map(|machine| {
            let sol_i = diophantine_solve(machine.ai, machine.bi, machine.ci);
            let sol_j = diophantine_solve(machine.aj, machine.bj, machine.cj);
            if sol_i.is_none() || sol_j.is_none() {
                return 0;
            }
            let sol_i = sol_i.unwrap();
            let sol_j = sol_j.unwrap();

            return if let Some((ki, kj)) = find_k(&sol_i, &sol_j) {
                let xi = sol_i.x + ki * sol_i.v;
                let xj = sol_j.x + kj * sol_j.v;
                let yi = sol_i.y - ki * sol_i.u;
                let yj = sol_j.y - kj * sol_j.u;

                // Assert that the solution is correct
                assert_eq!(xi, xj, "xi != xj for Machine [{:?}]", machine);
                assert_eq!(yi, yj, "yi != yj for Machine [{:?}]", machine);
                assert!(xi > 0, "xi <= 0 for Machine [{:?}]", machine);
                assert!(yi > 0, "yi <= 0 for Machine [{:?}]", machine);
                assert_eq!(
                    xi * machine.ai + yi * machine.bi,
                    machine.ci,
                    "Equation not satisfied for Machine [{:?}]",
                    machine
                );
                assert_eq!(
                    xj * machine.aj + yj * machine.bj,
                    machine.cj,
                    "Equation not satisfied for Machine [{:?}]",
                    machine
                );
                (3 * xi + yi) as u64
            } else {
                0
            };
        })
        .sum()
}

fn part_2(input: Vec<Machine>) -> u64 {
    part_1(input.iter().map(|m| Machine {
        ai: m.ai,
        aj: m.aj,
        bi: m.bi,
        bj: m.bj,
        ci: m.ci + 10000000000000,
        cj: m.cj + 10000000000000,
    }).collect())
}

// Used to find gcd and save intermediate steps
struct QuotientEquation {
    a: i64,
    b: i64,
    q: i64,
    r: i64,
}

impl Debug for QuotientEquation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {} * {} + {}", self.a, self.b, self.q, self.r)
    }
}

struct DiophantineSolution {
    x: i64,
    y: i64,
    u: i64,
    v: i64,
}

impl Debug for DiophantineSolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({} + k * ({}), {} - k * ({}))",
            self.x, self.v, self.y, self.u
        )
    }
}

fn diophantine_solve(a: i64, b: i64, c: i64) -> Option<DiophantineSolution> {
    let mut quotients: Vec<QuotientEquation> = vec![];

    // Find gcd and save intermediate quotients
    let (mut ra, mut rb) = if a > b { (a, b) } else { (b, a) };
    while rb != 0 {
        let q = ra / rb;
        let r = ra % rb;
        quotients.push(QuotientEquation { a: ra, b: rb, q, r });
        ra = rb;
        rb = r;
    }
    let gcd = ra;

    if c % gcd != 0 {
        return None;
    }


    let u = a / gcd;
    let v = b / gcd;

    if gcd == a {
        return Some(DiophantineSolution {
            x: c / gcd,
            y: 0,
            u,
            v,
        });
    }
    if gcd == b {
        return Some(DiophantineSolution {
            x: 0,
            y: c / gcd,
            u,
            v,
        });
    }

    let mut remainder_equations: HashMap<i64, HashMap<i64, i64>> = HashMap::new();
    quotients.iter().for_each(|q| {
        let mut equation = HashMap::new();
        equation.insert(q.a, 1);
        equation.insert(q.b, -q.q);
        remainder_equations.insert(q.r, equation);
    });

    let mut gcd_equation = remainder_equations.get(&gcd).unwrap().clone();
    let mut i = quotients.len() - 1;
    while i > 1 {
        let replaced_term = quotients.get(i as usize).unwrap().a;
        let multiplier = gcd_equation.get(&replaced_term).unwrap().clone();
        let incoming_equation = remainder_equations.get(&replaced_term).unwrap();
        for (k, v) in incoming_equation.iter() {
            let coefficient = gcd_equation.get(&k).unwrap_or(&0);
            gcd_equation.insert(*k, coefficient + multiplier * v);
            gcd_equation.remove(&replaced_term);
        }
        i -= 1;
    }

    Some(DiophantineSolution {
        x: gcd_equation.get(&a).unwrap() * (c / gcd),
        y: gcd_equation.get(&b).unwrap() * (c / gcd),
        u,
        v,
    })
}

fn find_k(sol_i: &DiophantineSolution, sol_j: &DiophantineSolution) -> Option<(i64, i64)> {
    let xi = sol_i.x;
    let yi = sol_i.y;
    let ui = sol_i.u;
    let vi = sol_i.v;

    let xj = sol_j.x;
    let yj = sol_j.y;
    let uj = sol_j.u;
    let vj = sol_j.v;

    let yj_minus_yi = yj - yi;
    let xj_minus_xi = xj - xi;
    let ujvi_minus_uivj = uj * vi - ui * vj;

    let numerator_i = uj * xj_minus_xi + vj * yj_minus_yi;
    let numerator_j = ui * xj_minus_xi + vi * yj_minus_yi;

    if numerator_i % ujvi_minus_uivj != 0 || numerator_j % ujvi_minus_uivj != 0 {
        return None;
    }

    let ki = numerator_i / ujvi_minus_uivj;
    let kj = numerator_j / ujvi_minus_uivj;

    Some((ki, kj))
}
