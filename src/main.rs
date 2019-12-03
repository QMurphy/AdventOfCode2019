mod day1;

struct DayInfo
{
    part1_func: Option<fn( &str ) -> String>,
    part2_func: Option<fn( &str ) -> String>,
    input_file: Option<&'static str>
}

const NUM_DAYS : usize = 25;

static DAYS: [DayInfo; NUM_DAYS] =
[
    DayInfo { part1_func: Some( day1::part1 ), part2_func: Some( day1::part2 ), input_file: Some( "input/1.txt" ) },
    DayInfo { part1_func: None,                part2_func: None,                input_file: None                  },
    DayInfo { part1_func: None,                part2_func: None,                input_file: None                  },
    DayInfo { part1_func: None,                part2_func: None,                input_file: None                  },
    DayInfo { part1_func: None,                part2_func: None,                input_file: None                  },
    DayInfo { part1_func: None,                part2_func: None,                input_file: None                  },
    DayInfo { part1_func: None,                part2_func: None,                input_file: None                  },
    DayInfo { part1_func: None,                part2_func: None,                input_file: None                  },
    DayInfo { part1_func: None,                part2_func: None,                input_file: None                  },
    DayInfo { part1_func: None,                part2_func: None,                input_file: None                  },
    DayInfo { part1_func: None,                part2_func: None,                input_file: None                  },
    DayInfo { part1_func: None,                part2_func: None,                input_file: None                  },
    DayInfo { part1_func: None,                part2_func: None,                input_file: None                  },
    DayInfo { part1_func: None,                part2_func: None,                input_file: None                  },
    DayInfo { part1_func: None,                part2_func: None,                input_file: None                  },
    DayInfo { part1_func: None,                part2_func: None,                input_file: None                  },
    DayInfo { part1_func: None,                part2_func: None,                input_file: None                  },
    DayInfo { part1_func: None,                part2_func: None,                input_file: None                  },
    DayInfo { part1_func: None,                part2_func: None,                input_file: None                  },
    DayInfo { part1_func: None,                part2_func: None,                input_file: None                  },
    DayInfo { part1_func: None,                part2_func: None,                input_file: None                  },
    DayInfo { part1_func: None,                part2_func: None,                input_file: None                  },
    DayInfo { part1_func: None,                part2_func: None,                input_file: None                  },
    DayInfo { part1_func: None,                part2_func: None,                input_file: None                  },
    DayInfo { part1_func: None,                part2_func: None,                input_file: None                  }
];

fn main() -> Result<(), i32>
{
    // Determine which day to run
    println!( "Run which day's code?" );
    let mut which_day_str = String::new();
    let which_day = match std::io::stdin().read_line( &mut which_day_str )
    {
        Ok( _ ) =>
        {
            match which_day_str.trim().parse()
            {
                Ok( which_day ) => which_day,
                Err( _ ) => 0
            }
        },
        Err( _ ) => 0
    };

    if which_day < 1 || which_day > 25
    {
        println!( "Invalid day specified" );
        return Err( -1 );
    }
    
    // Index the day info
    let day_info = &DAYS[which_day - 1];

    // Read in the input
    match day_info.input_file
    {
        Some( input_file ) =>
        {
            let input_str = match std::fs::read_to_string( input_file )
            {
                Ok( input_str ) => input_str,
                Err( _ )   =>
                {
                    println!( "Unable to read input file" );
                    return Err( -3 );
                }
            };
            
            println!( "Executing Day {}", which_day );
            match day_info.part1_func
            {
                Some( f ) => println!( "Part 1 Result: {}", f( &input_str ) ),
                None => println!( "No function for part 1" )
            };

            match day_info.part2_func
            {
                Some( f ) => println!( "Part 2 Result: {}", f( &input_str ) ),
                None => println!( "No function for part 2" )
            };

            return Ok( () );
        },
        None =>
        {
            println!( "No input defined for day {}", which_day );
            return Err( -2 );
        }
    }
}
