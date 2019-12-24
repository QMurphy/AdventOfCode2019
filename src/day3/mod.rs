use ndarray::Array2;

const GRID_SIZE : usize = 25001;
const TOTAL_WIRE_COUNT : u32 = 2;

enum RouteDir
{
    UP,
    DOWN,
    LEFT,
    RIGHT,
    UNK
}

struct WireRoute
{
    dir    : RouteDir,
    length : u16
}

struct Point2d
{
    x : i32,
    y : i32
}

impl Point2d
{
    fn new( x : i32, y : i32 ) -> Point2d
    {
        Point2d{ x: x, y: y }
    }
}

struct Intersection
{
    point     : Point2d,
    wire1_len : u32,
    wire2_len : u32,
}

impl Intersection
{
    fn new( point : Point2d, len1 : u32, len2 : u32 ) -> Intersection
    {
        Intersection{ point: point, wire1_len: len1, wire2_len: len2 }
    }
}

pub fn part1
    (
    input : &str
    ) -> String
{
    // Get the wire routes from the input string
    let wire_routes = str_to_wire_routes( input );

    // Find intersections
    let intersections = find_intersections( wire_routes );
    
    // Compute Manhattan Distances to each point
    let mut distances : Vec<i32> = intersections.iter().map( |intersection|
    {
        ( intersection.point.x - ( GRID_SIZE / 2 ) as i32 ).abs() + ( intersection.point.y - ( GRID_SIZE / 2 ) as i32 ).abs()
    } ).collect();

    // Sort
    distances.sort();

    // Find smallest distance (not counting origin) as string
    return distances[1].to_string();
}

pub fn part2
    (
    input : &str
    ) -> String
{
    // Get the wire routes from the input string
    let wire_routes = str_to_wire_routes( input );

    // Find intersections
    let intersections = find_intersections( wire_routes );
    
    let mut combined_lengths : Vec<u32> = intersections.iter().map( |intersection|
    {
        intersection.wire1_len + intersection.wire2_len - 2
    } ).collect();

    // Sort
    combined_lengths.sort();
    
    // Find smallest distance (not counting origin) as string
    return combined_lengths[1].to_string();
}

fn find_intersections
    (
    wire_routes : Vec<Vec<WireRoute>>
    ) -> Vec<Intersection>
{
    let mut grid = Array2::<u8>::zeros( ( GRID_SIZE, GRID_SIZE ) );
    let mut length1 = Array2::<u32>::zeros( ( GRID_SIZE, GRID_SIZE ) );
    let mut length2 = Array2::<u32>::zeros( ( GRID_SIZE, GRID_SIZE ) );
    let mut num_wires = 0;
    let all_wires_mask : u8 = 2u8.pow( TOTAL_WIRE_COUNT ) - 1;
    let mut intersections : Vec<Intersection> = Vec::new();

    // For each wire
    for wire_route in wire_routes.iter()
    {
        let mut total_length : u32 = 0;
        let mut cur_x : i32 = ( GRID_SIZE / 2 ) as i32;
        let mut cur_y : i32 = ( GRID_SIZE / 2 ) as i32;
        let wire_mask = 1 << num_wires;
        num_wires += 1;

        // Draw wire on grid
        for route in wire_route.iter()
        {
            let increment : Point2d;
            match route.dir
            {
                RouteDir::UP => increment = Point2d::new( 0, 1 ),
                RouteDir::DOWN => increment = Point2d::new( 0, -1 ),
                RouteDir::LEFT => increment = Point2d::new( -1, 0 ),
                RouteDir::RIGHT => increment = Point2d::new( 1, 0 ),
                _ => increment = Point2d::new( 0, 0 )
            }

            for _ in 0..route.length
            {
                total_length += 1;
                grid[[cur_y as usize, cur_x as usize]] |= wire_mask;

                if num_wires == 1 &&
                   length1[[cur_y as usize, cur_x as usize]] == 0
                {
                    length1[[cur_y as usize, cur_x as usize]] = total_length;
                }
                else if num_wires == 2 &&
                        length2[[cur_y as usize, cur_x as usize]] == 0
                {
                    length2[[cur_y as usize, cur_x as usize]] = total_length;
                }

                // See if the wires intersected
                if grid[[cur_y as usize, cur_x as usize]] & all_wires_mask == all_wires_mask
                {
                    // Add to intersection Vec
                    intersections.push( Intersection::new(
                                            Point2d::new( cur_x, cur_y ),
                                            length1[[cur_y as usize, cur_x as usize]],
                                            length2[[cur_y as usize, cur_x as usize]] ) );
                }

                cur_x += increment.x;
                cur_y += increment.y;
            }
        }
    }

    return intersections;
}

fn str_to_wire_routes
    (
    input : &str
    ) -> Vec<Vec<WireRoute>>
{
    let mut wire_strs : Vec<String> = String::from( input ).split("\n").map( String::from ).collect();
    while wire_strs.len() > TOTAL_WIRE_COUNT as usize
    {
        wire_strs.pop();
    }
    let wire_routes : Vec<Vec<WireRoute>> = wire_strs.iter().map( |wire_str|
    {
        str_to_wire_route( wire_str )
    } ).collect();

    return wire_routes;
}

fn str_to_wire_route
    (
    input : &str
    ) -> Vec<WireRoute>
{
    let wire_route_strs : Vec<String> = String::from( input ).split(",").map( String::from ).collect();
    let mut wire_routes = Vec::new();
    for wire_route_str in wire_route_strs.iter()
    {
        let dir = match wire_route_str.as_bytes()[0] as char
        {
            'U' => RouteDir::UP,
            'D' => RouteDir::DOWN,
            'L' => RouteDir::LEFT,
            'R' => RouteDir::RIGHT,
            _ =>
            {
                eprintln!( "Unable to parse route direction" );
                RouteDir::UNK
            }
        };

        let length = match wire_route_str[1..].parse()
        {
            Ok( value ) => value,
            Err( _ ) =>
            {
                eprintln!( "Unable to parse route length" );
                0
            }
        };

        wire_routes.push( WireRoute { dir, length } );
    }

    return wire_routes;
}
