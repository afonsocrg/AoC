use std::env;
use std::fs;
use std::time::Instant;
use z3::ast::Ast;
use z3::*;



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

fn parse_input(input_file: &str) -> ComputerState {
    let input = fs::read_to_string(input_file).expect("Failed to read file");
    let mut input = input.split("\n\n");
    let memory = input.next().expect("Failed to read memory");
    let program = input.next().expect("Failed to read program");

    let memory: Vec<i64> = memory
        .lines()
        .map(|l| {
            l.split_once(": ")
                .expect("Failed to split memory line")
                .1
                .parse::<i64>()
                .expect("Failed to parse memory value")
        })
        .collect();

    assert_eq!(memory.len(), 3);

    let program: Vec<i64> = program
        .split_once(": ")
        .expect("Failed to split program line")
        .1
        .split(',')
        .map(|n| n.parse::<i64>().expect("Failed to parse program value"))
        .collect();

    ComputerState {
        program,
        ip: 0,
        reg_a: memory[0],
        reg_b: memory[1],
        reg_c: memory[2],
        output: vec![],
    }
}

fn part_1(mut state: ComputerState) -> String {
    state.execute();
    state
        .output
        .iter()
        .map(|o| o.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn part_2(state: ComputerState) -> String {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    let opt = Optimize::new(&ctx);

    let a = ast::BV::new_const(&ctx, "a", 64);
    opt.minimize(&a);

    let mut z3_state = ComputerStateZ3 {
        program: state.program,
        ip: 0,
        reg_a: a.clone(),
        reg_b: ast::BV::from_i64(&ctx, state.reg_b, 64),
        reg_c: ast::BV::from_i64(&ctx, state.reg_c, 64),
        ctx: &ctx,
        assumptions: vec![],
        n_out_commands: 0,
    };

    z3_state.execute();

    if let SatResult::Sat = opt.check(&z3_state.assumptions) {
        let model = opt.get_model().unwrap();
        let a_val = model.eval(&a, false).unwrap();
        return a_val.as_i64().unwrap().to_string();
    }

    "".to_string()
}

#[derive(Debug)]
struct ComputerState {
    program: Vec<i64>,
    ip: usize,
    reg_a: i64,
    reg_b: i64,
    reg_c: i64,
    output: Vec<i64>,
}

impl ComputerState {
    fn execute(&mut self) {
        while self.can_step() {
            self.step();
        }
    }

    fn step(&mut self) {
        let instruction = self.program[self.ip];
        let operand = self.program[self.ip + 1];

        match instruction {
            0 => self.adv(operand),
            1 => self.bxl(operand),
            2 => self.bst(operand),
            3 => self.jnz(operand),
            4 => self.bxc(operand),
            5 => self.out(operand),
            6 => self.bdv(operand),
            7 => self.cdv(operand),
            _ => panic!("Invalid instruction"),
        }
    }

    fn can_step(&self) -> bool {
        self.ip < self.program.len()
    }

    fn get_combo_op(&self, arg: i64) -> i64 {
        match arg {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => panic!("Invalid combo op"),
        }
    }

    fn adv(&mut self, arg: i64) {
        // Dividing by 2^arg is the same as shifting right by arg
        let num = self.reg_a;
        let arg = self.get_combo_op(arg);
        let res = num >> arg;
        self.reg_a = res;
        self.ip += 2;
        // println!("adv: {} >> {} = {} => reg_a", num, arg, res);
    }

    fn bdv(&mut self, arg: i64) {
        // Dividing by 2^arg is the same as shifting right by arg
        let num = self.reg_a;
        let arg = self.get_combo_op(arg);
        let res = num >> arg;
        self.reg_b = res;
        self.ip += 2;
        // println!("bdv: {} >> {} = {} => reg_a", num, arg, res);
    }

    fn cdv(&mut self, arg: i64) {
        // Dividing by 2^arg is the same as shifting right by arg
        let num = self.reg_a;
        let arg = self.get_combo_op(arg);
        let res = num >> arg;
        self.reg_c = res;
        self.ip += 2;
        // println!("cdv: {} >> {} = {} => reg_a", num, arg, res);
    }

    fn bxl(&mut self, arg: i64) {
        let res = self.reg_b ^ arg;
        self.reg_b = res;
        self.ip += 2;
        // println!("bxl: {} XOR {} = {} => reg_b", self.reg_b, arg, res);
    }

    fn bst(&mut self, arg: i64) {
        let arg = self.get_combo_op(arg);
        let res = arg % 8;
        self.reg_b = res;
        self.ip += 2;
        // println!("bst: {} % 8 = {} => reg_b", arg, res);
    }

    fn jnz(&mut self, arg: i64) {
        if self.reg_a != 0 {
            self.ip = arg as usize;
            // println!("jnz: jumping to {}", arg);
        } else {
            self.ip += 2;
            // println!("jnz: skipped");
        }
    }

    fn bxc(&mut self, _: i64) {
        let res = self.reg_b ^ self.reg_c;
        self.reg_b = res;
        self.ip += 2;
        // println!("bxc: {} XOR {} = {} => reg_b", self.reg_b, self.reg_c, res);
    }

    fn out(&mut self, arg: i64) {
        let arg = self.get_combo_op(arg);
        let res = arg & 7;
        // println!("out: {} % 8 = {} => output", arg, res);
        self.output.push(res);
        self.ip += 2;
    }
}

#[derive(Debug)]
struct ComputerStateZ3<'ctx> {
    program: Vec<i64>,
    ip: usize,
    reg_a: ast::BV<'ctx>,
    reg_b: ast::BV<'ctx>,
    reg_c: ast::BV<'ctx>,

    // z3 part
    assumptions: Vec<ast::Bool<'ctx>>,
    ctx: &'ctx Context,
    n_out_commands: usize,
}

impl<'ctx> ComputerStateZ3<'ctx> {
    fn execute(&mut self) {
        while self.can_step() {
            self.step();
        }
    }

    fn step(&mut self) {
        let instruction = self.program[self.ip];
        let operand = self.program[self.ip + 1];

        // println!("Executing {} {}", instruction, operand);

        match instruction {
            0 => self.adv(operand),
            1 => self.bxl(operand),
            2 => self.bst(operand),
            3 => {
                // HACK: We know the program will only jump from the end to the beginning
                // so we can tell it when it should jump
                let should_jump = self.n_out_commands < self.program.len();
                self.jnz(operand, should_jump);
            }
            4 => self.bxc(operand),
            5 => {
                let expected = self.program[self.n_out_commands];
                self.out(operand, expected);
                self.n_out_commands += 1;
            }
            6 => self.bdv(operand),
            7 => self.cdv(operand),
            _ => panic!("Invalid instruction"),
        }
    }

    fn can_step(&self) -> bool {
        self.ip < self.program.len()
    }

    fn get_combo_op(&self, arg: i64) -> ast::BV<'ctx> {
        match arg {
            0 => ast::BV::from_i64(&self.ctx, 0, 64),
            1 => ast::BV::from_i64(&self.ctx, 1, 64),
            2 => ast::BV::from_i64(&self.ctx, 2, 64),
            3 => ast::BV::from_i64(&self.ctx, 3, 64),
            4 => self.reg_a.clone(),
            5 => self.reg_b.clone(),
            6 => self.reg_c.clone(),
            _ => panic!("Invalid combo op"),
        }
    }

    fn adv(&mut self, arg: i64) {
        self.reg_a = self.reg_a.bvlshr(&self.get_combo_op(arg));
        self.ip += 2;
    }

    fn bdv(&mut self, arg: i64) {
        self.reg_b = self.reg_a.bvlshr(&self.get_combo_op(arg));
        self.ip += 2;
    }

    fn cdv(&mut self, arg: i64) {
        self.reg_c = self.reg_a.bvlshr(&self.get_combo_op(arg));
        self.ip += 2;
    }

    fn bxl(&mut self, arg: i64) {
        self.reg_b = self.reg_b.bvxor(&ast::BV::from_i64(&self.ctx, arg, 64));
        self.ip += 2;
    }

    fn bst(&mut self, arg: i64) {
        self.reg_b = self
            .get_combo_op(arg)
            .bvand(&ast::BV::from_i64(&self.ctx, 7, 64));
        self.ip += 2;
    }

    fn jnz(&mut self, arg: i64, should_jump: bool) {
        if should_jump {
            let constraint = self.reg_a._eq(&ast::BV::from_i64(&self.ctx, 0, 64)).not();
            self.assumptions.push(constraint);
            self.ip = arg as usize;
        } else {
            let constraint = self.reg_a._eq(&ast::BV::from_i64(&self.ctx, 0, 64));
            self.assumptions.push(constraint);
            self.ip += 2;
        }
    }

    fn bxc(&mut self, _: i64) {
        self.reg_b = self.reg_b.bvxor(&self.reg_c);
        self.ip += 2;
    }

    fn out(&mut self, arg: i64, expected: i64) {
        let constraint = self
            .get_combo_op(arg)
            .bvand(&ast::BV::from_i64(&self.ctx, 7, 64))
            ._eq(&ast::BV::from_i64(&self.ctx, expected, 64));

        self.assumptions.push(constraint);
        self.ip += 2;
    }
}
