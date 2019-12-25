use super::cpu;

pub fn part1
    (
    input : &str
    ) -> String
{
    let mut code = cpu::str_to_code( input );
    code[1] = 12;
    code[2] = 2;
    let result = cpu::execute( &code, None, None );
    return result.to_string();
}

pub fn part2
    (
    input : &str
    ) -> String
{
    let mut code = cpu::str_to_code( input );
    let mut ret = String::new();
    for noun in 0..100
    {
        for verb in 0..100
        {
            code[1] = noun;
            code[2] = verb;
            let result = cpu::execute( &code, None, None );
            if result == 19690720
            {
                ret = ( 100 * noun + verb ).to_string();
                break;
            }
        }
    }
    return ret;
}
