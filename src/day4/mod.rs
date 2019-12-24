pub fn part1
    (
    input : &str
    ) -> String
{
    let min_max : Vec<u32> = String::from( input ).split("-").map( |s| { s.trim().parse().unwrap() } ).collect();
    let min = min_max[0];
    let max = min_max[1];
    let mut total = 0;

    for num in min..=max
    {
        if is_six_digits( num ) &&
           has_adjacent_same_digits( num ) &&
           never_decreases_digits( num )
        {
            total += 1;
        }
    }

    return total.to_string();
}

pub fn part2
    (
    input : &str
    ) -> String
{
    let min_max : Vec<u32> = String::from( input ).split("-").map( |s| { s.trim().parse().unwrap() } ).collect();
    let min = min_max[0];
    let max = min_max[1];
    let mut total = 0;

    for num in min..=max
    {
        if is_six_digits( num ) &&
           has_adjacent_same_digits_only_two( num ) &&
           never_decreases_digits( num )
        {
            total += 1;
        }
    }

    return total.to_string();
}

fn is_six_digits
    (
    num : u32
    ) -> bool
{
    return num >= 100_000 && num < 1_000_000;
}

fn has_adjacent_same_digits
    (
    mut num : u32
    ) -> bool
{
    let mut last_digit = num % 10;
    while num != 0
    {
        num /= 10;
        if num % 10 == last_digit
        {
            return true;
        }
        last_digit = num % 10;
    }
    return false;
}

fn has_adjacent_same_digits_only_two
    (
    mut num : u32
    ) -> bool
{
    let mut last_digit = num % 10;
    let mut concurrent_last_digit = 1;
    while num != 0
    {
        num /= 10;
        if num % 10 == last_digit
        {
            concurrent_last_digit += 1;
        }
        let new_last_digit = num % 10;
        if new_last_digit != last_digit
        {
            if concurrent_last_digit == 2
            {
                return true;
            }
            concurrent_last_digit = 1;
        }
        last_digit = new_last_digit;
    }
    return concurrent_last_digit == 2;
}

fn never_decreases_digits
    (
    mut num : u32
    ) -> bool
{
    let mut last_digit = num % 10;
    while num != 0
    {
        num /= 10;
        if num % 10 > last_digit
        {
            return false;
        }
        last_digit = num % 10;
    }
    return true;
}
