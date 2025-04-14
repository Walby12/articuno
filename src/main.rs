use std::usize;

static STACK_CAPACITY: usize = 1024;
static PROGRAM_CAPACITY: usize = 1024;

enum Trap {
    StackOverFlow,
    StackUnderFlow,
    DivByZero,
    IllegalInstAcces,
    Ok,
}

fn trap_as_str(trap: Trap) -> String {
    match trap {
        Trap::StackUnderFlow => {
            return String::from("Stack underflow");
        },
        Trap::StackOverFlow => {
            return String::from("Stack overflow");
        },
        Trap::DivByZero => {
            return String::from("Division by zero");
        },
        Trap::IllegalInstAcces => {
            return String::from("Illegal instruction acces");
        },
        Trap::Ok => {
            return String::from("Ok");
        },
    }
}

type Word = usize;

struct Articuno {
    stack: Vec<Word>,
    stack_size: usize,
    program: Vec<Inst>,
    program_size: usize,
    ip: Word,
    halt: bool,
}

#[derive(Clone)]
enum InstType {
    InstPush,
    InstPlus,
    InstMinus,
    InstMult,
    InstDiv,
    InstJmp,
    InstHalt,
    InstDump,
}

#[derive(Clone)]
struct Inst {
    ins_t: InstType,
    operand: Word,
}

fn inst_push(operand: Word) -> Inst {
    let inst = Inst {
        ins_t: InstType::InstPush,
        operand,
    };
    return inst;
}

fn inst_plus() -> Inst {
    let inst = Inst {
        ins_t: InstType::InstPlus,
        operand: 0,
    };
    return inst;
}

fn inst_minus() -> Inst {
    let inst = Inst {
        ins_t: InstType::InstMinus,
        operand: 0,
    };
    return inst;
}

fn inst_mul() -> Inst {
    let inst = Inst {
        ins_t: InstType::InstMult,
        operand: 0,
    };
    return inst;
}

fn inst_div() -> Inst {
    let inst = Inst {
        ins_t: InstType::InstDiv,
        operand: 0,
    };
    return inst;
}

fn inst_dump() -> Inst {
    let inst = Inst {
        ins_t: InstType::InstDump,
        operand: 0,
    };
    return inst;
}

fn inst_jump(operand: Word) -> Inst {
    let inst = Inst {
        ins_t: InstType::InstJmp,
        operand,
    };
    return inst;
}

fn inst_halt() -> Inst {
    let inst = Inst {
        ins_t: InstType::InstHalt,
        operand: 0,
    };
    return inst;
}

fn art_exec_inst(art: &mut Articuno,  inst: &Inst) -> Trap {
    match inst.ins_t {
        InstType::InstPlus => {
            if art.stack_size < 2 {
                return Trap::StackUnderFlow;
            }
            let a = art.stack.pop().unwrap();
            let b = art.stack.pop().unwrap();
            art.stack.push(a + b);
            art.stack_size -= 1;
            
        },
        InstType::InstMinus => {
            if art.stack_size < 2 {
                return Trap::StackUnderFlow;
            }
            let a = art.stack.pop().unwrap();
            let b = art.stack.pop().unwrap();
            art.stack.push(b - a);
            art.stack_size -= 1;
        },
        InstType::InstMult => {
            if art.stack_size < 2 {
                return Trap::StackUnderFlow;
            }
            let a = art.stack.pop().unwrap();
            let b = art.stack.pop().unwrap();
            art.stack.push(a * b);
            art.stack_size -= 1;
        },
        InstType::InstDiv => {
            if art.stack_size < 2 {
                return Trap::StackUnderFlow;
            }
            let a = art.stack.pop().unwrap();
            if a == 0 {
                return Trap::DivByZero;
            }
            let b = art.stack.pop().unwrap();
            art.stack.push(b / a);
            art.stack_size -= 1;
        },
        InstType::InstPush => {
            if art.stack_size >= STACK_CAPACITY {
                return Trap::StackOverFlow;
            }
            art.stack.push(inst.operand);
            art.stack_size += 1;
        },
        InstType::InstDump => {
            art_dump_stack(art);
        },
        InstType::InstJmp => {
            art.ip = inst.operand; 
        },
        InstType::InstHalt => {
            art.halt = true;
        },
    }
    art.ip += 1;
    Trap::Ok
}

fn art_dump_stack(art: &Articuno) {
    if art.stack_size == 0 {
        println!("Stack: [empty]");
        return;
    }
    println!("Stack:");
    for i in art.stack.clone() {
        println!("   {}", i);
    }
}

fn art_push_inst(art: &mut Articuno, inst: Inst) {
    assert!(art.program_size < PROGRAM_CAPACITY);
    art.program.push(inst);
    art.program_size += 1;
}

fn main() {
    let mut art = Articuno {
        stack: Vec::new(),
        stack_size: 0,
        program: Vec::new(),
        program_size: 0,
        ip: 0,
        halt: false,
    };

    art_push_inst(&mut art, inst_push(12));
    art_push_inst(&mut art, inst_push(12));
    art_push_inst(&mut art, inst_plus());
    art_push_inst(&mut art, inst_dump());
    
    while !art.halt && art.ip < art.program_size {
        let inst = art.program[art.ip].clone();
        let mut trap = art_exec_inst(&mut art, &inst);

        if art.ip < 0 || art.ip > art.program_size {
            trap = Trap::IllegalInstAcces;
        }
        
        match trap {
        Trap::Ok => continue,
        _ => {
            println!("Error: {}", trap_as_str(trap));
            art_dump_stack(&art);
            return;
            }
        }
    }
    println!("\nProgram executed successfully");
}
