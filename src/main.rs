use std::io::{self, Read};
use std::num::Wrapping;

enum Op {
    IncPointer,
    DecPointer,
    IncVal,
    DecVal,
    Print,
    Read,
    While { ops: Vec<Op> },
}

const HEAP_SIZE: usize = 4092;

struct State {
    curr_ptr: usize,
    data: [u8; HEAP_SIZE],
    output: Vec<u8>,
}

fn read() -> u8 {
    let mut buffer = [0];
    io::stdin().read_exact(&mut buffer).ok();
    buffer[0]
}

fn eval_while(state: &mut State, ops: &[Op]) {
    while state.data[state.curr_ptr] != 0 {
        eval_vec(state, ops);
    }
}

fn eval_vec(state: &mut State, ops: &[Op]) {
    for op in ops {
        eval(state, op);
    }
}

fn eval(state: &mut State, op: &Op) {
    match *op {
        Op::IncPointer => state.curr_ptr = (Wrapping(state.curr_ptr) + Wrapping(1)).0 % HEAP_SIZE,
        Op::DecPointer => state.curr_ptr = (Wrapping(state.curr_ptr) - Wrapping(1)).0 % HEAP_SIZE,
        Op::While { ref ops } => {
            eval_while(state, ops);
        }
        Op::IncVal => {
            state.data[state.curr_ptr] = (Wrapping(state.data[state.curr_ptr]) + Wrapping(1)).0
        }
        Op::DecVal => {
            state.data[state.curr_ptr] = (Wrapping(state.data[state.curr_ptr]) - Wrapping(1)).0
        }
        Op::Print => state.output.push(state.data[state.curr_ptr]),
        Op::Read => state.data[state.curr_ptr] = read(),
    }
}


fn get_ast(code: &[char]) -> (Vec<Op>, usize) {
    let mut ops = Vec::new();
    let mut i = 0;
    while i < code.len() {
        let ch = code[i];
        let op = match ch {
            '>' => Some(Op::IncPointer),
            '<' => Some(Op::DecPointer),
            '+' => Some(Op::IncVal),
            '-' => Some(Op::DecVal),
            '.' => Some(Op::Print),
            ',' => Some(Op::Read),
            '[' => {
                let (ops, size) = get_ast(&code[(i + 1..code.len())]);
                i += size + 1;
                match code[i] {
                    ']' => Some(Op::While { ops: ops }),
                    x => panic!("while loop needs to end with ']' but was with '{:?}'", x),
                }
            }
            ']' => return (ops, i),  
            _ => None,
        };
        if let Some(op) = op {
            ops.push(op);
        }
        i += 1;
    }
    (ops, i)
}

fn run_brainfuck(code: &str) -> String {
    let mut state = State {
        curr_ptr: 0,
        data: [0; HEAP_SIZE],
        output: Vec::new(),
    };

    let chars: Vec<char> = code.chars().collect();
    let (ast, _) = get_ast(&chars);
    eval_vec(&mut state, &ast);
    String::from_utf8_lossy(&state.output.as_slice()).into_owned()
}

fn main() {
    println!(
        "{}",
        run_brainfuck(
            "++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++.. +++.>++.<<+++++++++++++++.>.+++.------.--------.>+.",
        )
    );
}
