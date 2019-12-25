use super::cpu;
use std::io;
use std::io::Write;

static mut LAST_PRINTED_VALUE : i32 = 0;

pub fn both_parts
    (
    input : &str
    ) -> String
{
    let code = cpu::str_to_code( input );
    cpu::execute( &code, Some( &read_stdin ), Some( &test_output ) );
    unsafe
    {
        return LAST_PRINTED_VALUE.to_string();
    }
}

fn read_stdin
    (
    read_value : &mut i32
    )
{
    print!( "Program Requesting Input: " );
    io::stdout().flush().unwrap();
    let mut input = String::new();
    *read_value = match io::stdin().read_line(&mut input) {
        Ok(_) =>
        {
            let parsed : i32 = match input.trim().parse()
            {
                Ok( num ) => num,
                Err( error ) => 
                {
                    eprintln!( "Unable to parse integer: {}", error );
                    0
                }
            };
            parsed
        }
        Err( error ) =>
        {
            eprintln!( "Unable to read from stdin: {}", error );
            0
        }
    }
}

fn test_output
    (
    write_value : i32
    )
{
    unsafe
    {
        LAST_PRINTED_VALUE = write_value;
    }
}
