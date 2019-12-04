const ADD_OP : usize = 1;
const MULT_OP : usize = 2;
const TERM_OP : usize = 99;

pub fn part1
    (
    input : &str
    ) -> String
{
    let code = str_to_vec_usize( input );
    let result = execute_program( &code, 12, 2 );
    return result.to_string();
}

pub fn part2
    (
    input : &str
    ) -> String
{
    let code = str_to_vec_usize( input );
    let mut ret = String::new();
    for noun in 0..100
    {
        for verb in 0..100
        {
            let result = execute_program( &code, noun, verb );
            if result == 19690720
            {
                ret = ( 100 * noun + verb ).to_string();
                break;
            }
        }
    }
    return ret;
}

fn str_to_vec_usize
    (
    input : &str
    ) -> Vec<usize>
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
fn vec_usize_to_str
    (
    input : &Vec<usize>
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

fn execute_program
    (
    code : &Vec<usize>,
    noun : usize,
    verb : usize
    ) -> usize
{
    let mut code = code.clone(); // Make a mutable copy of the program
    code[1] = noun;
    code[2] = verb;

    let mut pc = 0;
    while code[pc] != TERM_OP
    {
        match code[pc]
        {
            ADD_OP => op2( &mut code, pc, &add ),
            MULT_OP => op2( &mut code, pc, &mult ),
            _ =>
            {
                eprintln!( "Unexpected opcode ({})", code[pc] );
                break;
            }
        }

        pc += 4;
    }

    return code[0];
}

fn op2
    (
    code : &mut Vec<usize>,
    pc   : usize,
    func : &Fn( usize, usize ) -> usize
    )
{
    let read_idx1 = code[pc + 1];
    let read_idx2 = code[pc + 2];
    let write_idx = code[pc + 3];
    if read_idx1 >= code.len()
    {
        eprintln!( "Read index #1 ({}) out of bounds", read_idx1 );
    }
    else if read_idx2 >= code.len()
    {
        eprintln!( "Read index #2 ({}) out of bounds", read_idx2 );
    }
    else if write_idx >= code.len()
    {
        eprintln!( "Write index ({}) out of bounds", write_idx );
    }
    else
    {
        code[write_idx] = func( code[read_idx1], code[read_idx2] );
    }
}

fn add
    (
    a : usize,
    b : usize
    ) -> usize
{
    return a + b;
}

fn mult
    (
    a : usize,
    b : usize
    ) -> usize
{
    return a * b;
}
