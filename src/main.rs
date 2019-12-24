mod day1;
mod day2;
mod day3;

struct DayInfo
{
    part1_func: fn( &str ) -> String, // The function pointer to run Part 1 of that day
    part2_func: fn( &str ) -> String, // The function pointer to run Part 2 of that day
    input_file: &'static str          // The file to read the input from
}

const NUM_DAYS : usize = 25;

static DAYS: [Option<DayInfo>; NUM_DAYS] =
[
    Some( DayInfo { part1_func: day1::part1, part2_func: day1::part2, input_file: "input/1.txt" } ),
    Some( DayInfo { part1_func: day2::part1, part2_func: day2::part2, input_file: "input/2.txt" } ),
    Some( DayInfo { part1_func: day3::part1, part2_func: day3::part2, input_file: "input/3.txt" } ),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None
];

fn main() -> Result<(), i32>
{
    // Read in which day to run
    println!( "Run which day's code?" );
    let mut which_day_str = String::new();
    let which_day = match std::io::stdin().read_line( &mut which_day_str )
    {
        Ok( _ ) =>
        {
            // Try to parse the integer
            match which_day_str.trim().parse()
            {
                Ok( which_day ) => which_day,
                Err( _ ) => 0
            }
        },
        Err( _ ) => 0
    };

    if which_day < 1 || which_day > NUM_DAYS
    {
        eprintln!( "Invalid day specified" );
        return Err( -1 );
    }
    
    // Index the day info
    let day = &DAYS[which_day - 1];

    // Read in the input
    match day
    {
        Some( day_info ) =>
        {
            let input_str = match std::fs::read_to_string( day_info.input_file )
            {
                Ok( input_str ) => input_str,
                Err( _ )   =>
                {
                    eprintln!( "Unable to read input file" );
                    return Err( -3 );
                }
            };
            
            // Execute
            println!( "Executing Day {}", which_day );
            println!( "Part 1 Result: {}", (day_info.part1_func)( &input_str ) );
            println!( "Part 2 Result: {}", (day_info.part2_func)( &input_str ) );

            return Ok( () );
        },
        None =>
        {
            eprintln!( "Day {} undefined", which_day );
            return Err( -2 );
        }
    }
}
