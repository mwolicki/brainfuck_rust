use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::num::Wrapping;

#[derive(Debug,Clone)]
enum Op {
    IncPointer,
    DecPointer,
    IncVal,
    DecVal,
    Print,
    Read,
    While { ops: Vec<Op> }
}

#[derive(Debug,Clone)]
struct State {
    curr_ptr :i32,
    data : HashMap<i32, u8>
}

fn read() -> u8 {
    let mut buffer = [0];
    io::stdin().read(&mut buffer).ok();
    return buffer[0];
}

fn print(ch:u8){
    let buf = &[ch];
    let _ = io::stdout().write(buf);
}


fn eval_while(s: State, ops: Vec<Op>) -> State {
    let mut state = s;
    while *(state.data.entry(state.curr_ptr).or_insert(0)) != 0 {
        state = eval_vec(state, ops.clone());
    }
    return state;
}

fn eval_vec(s: State, ops: Vec<Op>) -> State {
    let mut state = s;
        for op in ops.iter() {
            let op_code = op.clone();
            state = eval (state, op_code);
        }
    return state;
}

fn eval (mut s:State, op:Op) -> State {
        {
            let state = s.clone();
            
            let val = s.data.entry(s.curr_ptr).or_insert(0);
            let wrapped_val = Wrapping(*val);
            let wrapped_one = Wrapping(1);
            
            match op {
                Op::IncPointer => s.curr_ptr += 1,
                Op::DecPointer => s.curr_ptr -= 1,
                Op::IncVal => *val = (wrapped_val + wrapped_one).0,
                Op::DecVal => *val = (wrapped_val - wrapped_one).0,
                Op::Print => print(*val),
                Op::Read => *val = read(),
                Op::While { ops } => { 
                    return eval_while(state, ops); 
                }
            }
        }
        s
}

fn get_ast (code:&str) -> (Vec<Op>, usize) {
    let mut ops = Vec::new();
    let mut i = 0;
    while i < code.len() {
        let ch = code[(i..i+1)].as_ref();
        let op = match ch {
            ">" => Some(Op::IncPointer),
            "<" => Some(Op::DecPointer),
            "+" => Some(Op::IncVal),
            "-" => Some(Op::DecVal),
            "." => Some(Op::Print),
            "," => Some(Op::Read),
            "[" =>{
                let (ops, size) = get_ast(code[(i+1..code.len())].as_ref());
                i=i+size+1;
                match code[(i..i+1)].as_ref() {
                    "]" => Some(Op::While{ops : ops}),
                    x => panic!("blah {:?}", x)}}
            "]" => { return (ops, i) } 
            _ => None,
        };
        if let Some(op) = op {
            ops.push (op);
        }
        i=i+1;        
    }
    return (ops, i)
}



fn main() {
    let x = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
    //println!(">>{:?}", get_ast(x));

    let init_state = State { curr_ptr : 0, data : HashMap::new() };
    let (ast, _) = get_ast(x);
    eval_vec(init_state, ast);
}
