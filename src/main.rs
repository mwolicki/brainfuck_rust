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
    io::stdin().read_exact(&mut buffer).ok();
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
                Op::While { ops } => { return eval_while(state, ops); }
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
                    x => panic!("while loop needs to end with ']' but was with '{:?}'", x)}}
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
    let hello_word = " 
[ This program prints 'Hello World!' and a newline to the screen, its
  length is 106 active command characters. [It is not the shortest.]

  This loop is an \"initial comment loop\", a simple way of adding a comment
  to a BF program such that you don't have to worry about any command
  characters. Any .,+-<> characters are simply
  ignored, the '[' and ']' characters just have to be balanced. This
  loop and the commands it contains are ignored because the current cell
  defaults to a value of 0; the 0 value causes this loop to be skipped.
]
++++++++               Set Cell #0 to 8
[
    >++++               Add 4 to Cell #1; this will always set Cell #1 to 4
    [                   as the cell will be cleared by the loop
        >++             Add 2 to Cell #2
        >+++            Add 3 to Cell #3
        >+++            Add 3 to Cell #4
        >+              Add 1 to Cell #5
        <<<<-           Decrement the loop counter in Cell #1
    ]                   Loop till Cell #1 is zero; number of iterations is 4
    >+                  Add 1 to Cell #2
    >+                  Add 1 to Cell #3
    >-                  Subtract 1 from Cell #4
    >>+                 Add 1 to Cell #6
    [<]                 Move back to the first zero cell you find; this will
                        be Cell #1 which was cleared by the previous loop
    <-                  Decrement the loop Counter in Cell #0
]                       Loop till Cell #0 is zero; number of iterations is 8

The result of this is:
Cell No :   0   1   2   3   4   5   6
Contents:   0   0  72 104  88  32   8
Pointer :   ^

>>.                     Cell #2 has value 72 which is 'H'
>---.                   Subtract 3 from Cell #3 to get 101 which is 'e'
+++++++..+++.           Likewise for 'llo' from Cell #3
>>.                     Cell #5 is 32 for the space
<-.                     Subtract 1 from Cell #4 for 87 to give a 'W'
<.                      Cell #3 was set to 'o' from the end of 'Hello'
+++.------.--------.    Cell #3 for 'rl' and 'd'
>>+.                    Add 1 to Cell #5 gives us an exclamation point
>++.                    And finally a newline from Cell #6";
    let init_state = State { curr_ptr : 0, data : HashMap::new() };
    let (ast, _) = get_ast(hello_word);
    eval_vec(init_state, ast);
}
