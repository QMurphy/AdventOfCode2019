pub fn part1
    (
    input : &str
    ) -> String
{
    let total_fuel = get_module_fuel( input, false );

    return total_fuel.to_string();
}

pub fn part2
    (
    input : &str
    ) -> String
{
    let total_fuel = get_module_fuel( input, true );

    return total_fuel.to_string();
}

fn get_module_fuel
    (
    input : &str,
    fuel_has_weight : bool
    ) -> u32
{
    let mut total_fuel = 0;
    for line_result in input.split_whitespace()
    {
        let mass = match line_result.parse()
        {
            Ok( mass ) => mass,
            Err( _ ) =>
            {
                println!( "Error parsing mass" );
                0
            }
        };

        let mut this_fuel = module_mass_to_fuel_mass( mass );
        total_fuel += this_fuel;
        if fuel_has_weight
        {
            loop
            {
                let add_fuel = ( this_fuel / 3 ) as i32 - 2;

                if add_fuel <= 0
                {
                    break;
                }
                else
                {
                    total_fuel += add_fuel;
                    this_fuel = add_fuel;
                }
            }
        }
    }

    return total_fuel as u32;
}

fn module_mass_to_fuel_mass
    (
    module_mass : u32
    ) -> i32
{
    return ( module_mass / 3 ) as i32 - 2;
}
