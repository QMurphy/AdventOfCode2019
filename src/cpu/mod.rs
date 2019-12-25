const ADD_OP : i32 = 1;
const MULT_OP : i32 = 2;
const READ_OP : i32 = 3;
const WRITE_OP : i32 = 4;
const JUMP_TRUE_OP : i32 = 5;
const JUMP_FALSE_OP : i32 = 6;
const LESS_THAN_OP : i32 = 7;
const EQUALS_OP : i32 = 8;
const TERM_OP : i32 = 99;

const POSITION_MODE : i32 = 0;
const IMMEDIATE_MODE : i32 = 1;

pub fn execute
    (
    code     : &Vec<i32>,
    read_fn  : Option<&dyn Fn( &mut i32 )>,
    write_fn : Option<&dyn Fn( i32 )>
    ) -> i32
{
    let mut code = code.clone(); // Make a mutable copy of the program

    let mut pc = 0;
    loop
    {
        let op = code[pc] % 100;
        let mode1 = ( code[pc] / 100 ) % 10;
        let mode2 = ( code[pc] / 1000 ) % 10;
        let mode3 = ( code[pc] / 10000 ) % 10;
        
        if op == TERM_OP
        {
            break;
        }

        let advance_pc_by = match op
        {
            ADD_OP => alu_op( &mut code, pc, mode1, mode2, mode3, &add ),
            MULT_OP => alu_op( &mut code, pc, mode1, mode2, mode3, &mult ),
            READ_OP => read_op( &mut code, pc, mode1, read_fn.unwrap() ),
            WRITE_OP => write_op( &code, pc, mode1, write_fn.unwrap() ),
            JUMP_TRUE_OP => jump_op( &code, pc, mode1, mode2, &is_non_zero ),
            JUMP_FALSE_OP => jump_op( &code, pc, mode1, mode2, &is_zero ),
            LESS_THAN_OP => alu_op( &mut code, pc, mode1, mode2, mode3, &less_than ),
            EQUALS_OP => alu_op( &mut code, pc, mode1, mode2, mode3, &equal ),
            _ =>
            {
                eprintln!( "Unexpected opcode ({})", code[pc] );
                break;
            }
        };

        if advance_pc_by > 0
        {
            pc += advance_pc_by as usize;
        }
        else
        {
            pc -= advance_pc_by.abs() as usize;
        }
    }

    return code[0];
}

pub fn str_to_code
    (
    input : &str
    ) -> Vec<i32>
{
    let strs : Vec<String> = String::from( input ).split(",").map( String::from ).collect();
    let mut ints = Vec::new();
    for int_str in strs.iter()
    {
        match int_str.trim().parse()
        {
            Ok( integer ) => ints.push( integer ),
            Err( _ ) => println!( "Unable to parse integer ({})", int_str )
        }
    }

    return ints;
}

#[allow(dead_code)]
pub fn code_to_str
    (
    input : &Vec<i32>
    ) -> String
{
    let mut ret = String::new();

    for( pos, i ) in input.iter().enumerate()
    {
        ret.push_str( &i.to_string() );
        if pos as usize + 1 < input.len()
        {
            ret.push_str( "," );
        }
    }

    return ret;
}

fn alu_op
    (
    code  : &mut Vec<i32>,
    pc    : usize,
    mode1 : i32,
    mode2 : i32,
    mode3 : i32,
    func  : &dyn Fn( i32, i32 ) -> i32
    ) -> i32
{
    if mode3 != POSITION_MODE
    {
        eprintln!( "Write index must be in position mode" );
    }
    else
    {
        let read_value1 = match mode1
        {
            POSITION_MODE =>
            {
                let read_idx = code[pc + 1] as usize;
                if read_idx >= code.len()
                {
                    eprintln!( "Read index ({}) out of bounds", read_idx );
                    0
                }
                else
                {
                    code[read_idx]
                }
            },
            IMMEDIATE_MODE => code[pc + 1],
            _ =>
            {
                eprintln!( "Invalid mode specified" );
                0
            }
        };

        let read_value2 = match mode2
        {
            POSITION_MODE =>
            {
                let read_idx = code[pc + 2] as usize;
                if read_idx >= code.len()
                {
                    eprintln!( "Read index ({}) out of bounds", read_idx );
                    0
                }
                else
                {
                    code[read_idx]
                }
            },
            IMMEDIATE_MODE => code[pc + 2],
            _ =>
            {
                eprintln!( "Invalid mode specified" );
                0
            }
        };
        
        let write_idx = code[pc + 3] as usize;
        if write_idx >= code.len()
        {
            eprintln!( "Write index ({}) out of bounds", write_idx );
        }
        else
        {
            code[write_idx] = func( read_value1, read_value2 );
        }
    }

    return 4;
}

fn read_op
    (
    code : &mut Vec<i32>,
    pc   : usize,
    mode : i32,
    func : &dyn Fn( &mut i32 )
    ) -> i32
{
    if mode != POSITION_MODE
    {
        eprintln!( "Index must be in position mode" );
    }
    else
    {
        let read_idx = code[pc + 1] as usize;
        if read_idx >= code.len()
        {
            eprintln!( "Index ({}) out of bounds", read_idx );
        }
        else
        {
            func( &mut code[read_idx] );
        }
    }

    return 2;
}

fn write_op
    (
    code : &Vec<i32>,
    pc   : usize,
    mode : i32,
    func : &dyn Fn( i32 )
    ) -> i32
{
    let write_value = match mode
    {
        POSITION_MODE =>
        {
            let write_idx = code[pc + 1] as usize;
            if write_idx >= code.len()
            {
                eprintln!( "Write index ({}) out of bounds", write_idx );
                0
            }
            else
            {
                code[write_idx]
            }
        },
        IMMEDIATE_MODE => code[pc + 1],
        _ =>
        {
            eprintln!( "Invalid mode specified" );
            0
        }
    };

    func( write_value );

    return 2;
}

fn jump_op
    (
    code  : &Vec<i32>,
    pc    : usize,
    mode1 : i32,
    mode2 : i32,
    func  : &dyn Fn( i32 ) -> bool
    ) -> i32
{
    let jump_cond = match mode1
    {
        POSITION_MODE =>
        {
            let cond_idx = code[pc + 1] as usize;
            if cond_idx >= code.len()
            {
                eprintln!( "Jump index ({}) out of bounds", cond_idx );
                0
            }
            else
            {
                code[cond_idx]
            }
        },
        IMMEDIATE_MODE => code[pc + 1],
        _ =>
        {
            eprintln!( "Invalid mode specified" );
            0
        }
    };

    // If the jump condition is true, compute jump amount
    if func( jump_cond )
    {
        let jump_pos = match mode2
        {
            POSITION_MODE =>
            {
                let jump_idx = code[pc + 2] as usize;
                if jump_idx >= code.len()
                {
                    eprintln!( "Jump index ({}) out of bounds", jump_idx );
                    0
                }
                else
                {
                    code[jump_idx]
                }
            },
            IMMEDIATE_MODE => code[pc + 2],
            _ =>
            {
                eprintln!( "Invalid mode specified" );
                0
            }
        };

        return jump_pos - pc as i32;
    }
    // Otherwise just go past the jump
    else
    {
        return 3;
    }
}

fn is_zero
    (
    a : i32
    ) -> bool
{
    return a == 0;
}

fn is_non_zero
    (
    a : i32
    ) -> bool
{
    return a != 0;
}

fn add
    (
    a : i32,
    b : i32
    ) -> i32
{
    return a + b;
}

fn mult
    (
    a : i32,
    b : i32
    ) -> i32
{
    return a * b;
}

fn less_than
    (
    a : i32,
    b : i32
    ) -> i32
{
    return match a < b
    {
        true => 1,
        false => 0
    };
}

fn equal
    (
    a : i32,
    b : i32
    ) -> i32
{
    return match a == b
    {
        true => 1,
        false => 0
    };
}
