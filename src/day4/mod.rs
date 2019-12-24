pub fn part1
    (
    input : &str
    ) -> String
{
    let minMax : Vec<u32> = String::from( input ).split("-").map( |s| { s.trim().parse().unwrap() } ).collect();
    let min = minMax[0];
    let max = minMax[1];
    let mut total = 0;

    for num in min..=max
    {
        if isSixDigits( num ) &&
           hasAdjacentSameDigits( num ) &&
           neverDecreasesDigits( num )
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
    let minMax : Vec<u32> = String::from( input ).split("-").map( |s| { s.trim().parse().unwrap() } ).collect();
    let min = minMax[0];
    let max = minMax[1];
    let mut total = 0;

    for num in min..=max
    {
        if isSixDigits( num ) &&
           hasAdjacentSameDigitsOnlyTwo( num ) &&
           neverDecreasesDigits( num )
        {
            total += 1;
        }
    }

    return total.to_string();
}

fn isSixDigits
    (
    num : u32
    ) -> bool
{
    return num >= 100_000 && num < 1_000_000;
}

fn hasAdjacentSameDigits
    (
    mut num : u32
    ) -> bool
{
    let mut lastDigit = num % 10;
    while num != 0
    {
        num /= 10;
        if num % 10 == lastDigit
        {
            return true;
        }
        lastDigit = num % 10;
    }
    return false;
}

fn hasAdjacentSameDigitsOnlyTwo
    (
    mut num : u32
    ) -> bool
{
    let mut lastDigit = num % 10;
    let mut concurrentLastDigit = 1;
    while num != 0
    {
        num /= 10;
        if num % 10 == lastDigit
        {
            concurrentLastDigit += 1;
        }
        let newLastDigit = num % 10;
        if newLastDigit != lastDigit
        {
            if concurrentLastDigit == 2
            {
                return true;
            }
            concurrentLastDigit = 1;
        }
        lastDigit = newLastDigit;
    }
    return concurrentLastDigit == 2;
}

fn neverDecreasesDigits
    (
    mut num : u32
    ) -> bool
{
    let mut lastDigit = num % 10;
    while num != 0
    {
        num /= 10;
        if num % 10 > lastDigit
        {
            return false;
        }
        lastDigit = num % 10;
    }
    return true;
}
