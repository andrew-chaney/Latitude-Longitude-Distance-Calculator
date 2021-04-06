// Take two positions, lat/long, and return the kms/miles between them
use std::io::stdin;

// Header to welcome user to the program
pub fn header() {
    println!("Welcome to the latitude and longitude distance finder.");
    println!("This tool takes two lats/longs as input, and returns distance in miles or kilometers.");
    println!();
}

// Ask user for miles or kilometers
pub fn get_units() -> String {
    // Enter loop to verify valid input
    loop{
        // String to store input
        let mut units = String::new();
        println!("Please select kilometers or miles (k/m): ");
        // Read user input
        stdin().read_line(&mut units).expect("ERROR: unable to read line");
        let units = units.trim().to_lowercase();
        // If the input is 'k' or 'm' it's valid
        if units == "k" || units == "m" {
            println!();
            // Return will break the loop and exit the function
            return units.to_string();
        }
        // Otherwise, throw an "error" message and ask for proper input.
        // Continue loop
        println!("Inproper input, please select 'k' for kilometers or 'm' for miles only.");
    }
}

// Get a set of coordinates from user, lat/long in degrees minutes-decimal format
pub fn get_coordinates() -> Vec<Vec<f32>> {
    // Getting coordinate
    println!("For the coordinates please ensure if the coordinate degree is North or East to leave the degree as a positive number. If the coordinate is South or West please enter the degree as a negative number.");
    // Vector to store complete coordinate
    let mut coord: Vec<Vec<f32>> = vec![vec![0.0, 0.0], vec![0.0, 0.0]];
    // Get latitude of coordinate
    println!("We'll start with latitude.");
    // Get degree of latitude
    // Enter loop to verify valid input
    loop {
        // String to store user input
        let mut lat_degrees: String = String::new();
        println!("Please enter the degree of your latitude without the degree symbol: ");
        // Read user input
        stdin().read_line(&mut lat_degrees).expect("ERROR: unable to read line");
        let check_deg = lat_degrees.trim().parse::<f32>().unwrap().abs();
        // If the input is between -90.0 and 90.0, the input is valid
        if check_deg < 90.0 && check_deg > 0.0  {
            // Save lat degree
            coord[0][0] = lat_degrees.trim().parse::<f32>().unwrap();
            // Break loop
            break;
        }
        // Otherwise, input is invalid, print error message and get proper input
        else {
            println!("ERROR: invalid input");
            println!("\t Your latitude degree input should be between -90 and 90 degrees.");
            println!("\t Any input besides a negative sign and numbers will result in errors.");
        }
    }
    // Get rest of latitude in minute-decimal format (ex: xx.xx)
    // Enter loop to verify valid input
    loop {
        let mut lat_md: String = String::new();
        println!("Please enter the minutes and seconds of your latitude in minute-decimal format (EX: 14.15 is 14 minutes and 9 seconds):  ");
        // Read user input
        stdin().read_line(&mut lat_md).expect("ERROR: unable to read line");
        let check_md = lat_md.trim().parse::<f32>().unwrap();
        // If input is between 0.00 and 60.00, the input is valid
        if check_md >= 0.0 && check_md <= 60.0 {
            // Save lat minutes
            coord[0][1] = lat_md.trim().parse::<f32>().unwrap();
            // Break loop
            break;
        }
        // Otherwise, input is invalid, print error message and get proper input
        else {
            println!("ERROR: invalid input");
            println!("\t Your latitude minutes and seconds input should be between 00.00 and 60.00 degrees.");
            println!("\t Any input besides numbers will result in errors.");
        }
    }

    // Get longitude of coordinate
    println!("Now, we'll move onto the longitude.");
    // Get degree of longitude
    // This is exactly the same as for latitude, please refer to comments written above
    loop {
        let mut long_degrees: String = String::new();
        println!("Please enter the degree of your longitude without the degree symbol: ");
        stdin().read_line(&mut long_degrees).expect("ERROR: unable to read line");
        let check_deg = long_degrees.trim().parse::<f32>().unwrap().abs();
        if check_deg < 180.0 && check_deg > 0.0  {
            coord[1][0] = long_degrees.trim().parse::<f32>().unwrap();
            break;
        }
        else {
            println!("ERROR: invalid input");
            println!("\t Your degree of longitude input should be between -180 and 180 degrees.");
            println!("\t Any input besides a negative sign and numbers will result in errors.");
        }
    }
    // Get rest of longitude in minute-decimal format (ex: xx.xx)
    loop {
        let mut long_md: String = String::new();
        println!("Please enter the minutes and seconds of your latitude in minute-decimal format (EX: 14.15 is 14 minutes and 9 seconds):  ");
        stdin().read_line(&mut long_md).expect("ERROR: unable to read line");
        let check_md = long_md.trim().parse::<f32>().unwrap();
        if check_md >= 0.0 && check_md <= 60.0 {
            coord[1][1] = long_md.trim().parse::<f32>().unwrap();
            break;
        }
        else {
            println!("ERROR: invalid input");
            println!("\t Your latitude minutes and seconds input should be between 00.00 and 60.00 degrees.");
            println!("\t Any input besides numbers will result in errors.");
        }
    }
    return coord;
}

// Function to print a more readable lat/long for the user to verify accuracy
pub fn beautify(coord: &Vec<Vec<f32>>) {
    // If degree of latitude is positive, the degree is North.
    if coord[0][0] > 0.0 {
        // If the degree of longitude is positive, the degree is East
        if coord[1][0] > 0.0 {
            println!("{}N-{}'  {}E-{}'", coord[0][0].round().abs(), coord[0][1], coord[1][0].round().abs(), coord[1][1]);
        }
        // If the degree of latitude is negative, the degree is West
        else {
            println!("{}N-{}'  {}W-{}'", coord[0][0].round().abs(), coord[0][1], coord[1][0].round().abs(), coord[1][1]);
        }
    }
    // If the degree of latitude is negatice, the degree is South
    else {
        // If the degree of longitude is positive, the degree is East
        if coord[1][0] > 0.0 {
            println!("{}S-{}'  {}E-{}'", coord[0][0].round().abs(), coord[0][1], coord[1][0].round().abs(), coord[1][1]);
        }
        // If the degree of latitude is negative, the degree is West
        else {
            println!("{}S-{}'  {}W-{}'", coord[0][0].round().abs(), coord[0][1], coord[1][0].round().abs(), coord[1][1]);
        }
    }
}

// Reformat lat/long from degrees-minutes-decimal to just degree format for easier computation
fn reformat(coord: Vec<Vec<f32>>) -> (f32, f32) {
    let lat: f32;
    let lon: f32;
    if coord[0][0] < 0.0 {
        lat = coord[0][0] - (coord[0][1] / 60.0);
    } else {
        lat = coord[0][0] + (coord[0][1] / 60.0);
    }
    if coord[1][0] < 0.0 {
        lon = coord[1][0] - (coord[1][1] / 60.0);
    } else {
        lon = coord[1][0] + (coord[1][1] / 60.0);
    }

    return (lat, lon);
}

// Find the central angle between the two points (think a triangle)
fn angle(coord1: Vec<Vec<f32>>, coord2: Vec<Vec<f32>>) -> f32 {
    // Reformat coordinates to degree format
    let (lat1_deg, lon1_deg) = reformat(coord1);
    let (lat2_deg, lon2_deg) = reformat(coord2);
    // Convert from degrees to radians
    let lat1: f32 = lat1_deg.to_radians();
    let lon1: f32 = lon1_deg.to_radians();
    let lat2: f32 = lat2_deg.to_radians();
    let lon2: f32 = lon2_deg.to_radians();
    // Find the deltas/differences between the two lats and longs
    let lat_delta: f32 = lat2 - lat1;
    let lon_delta: f32 = lon2 - lon1;
    // Plug values into Haversine Formula to get result
    let angle: f32 = (lat_delta / 2.0).sin().powf(2.0) + lat1.cos() * lat2.cos() * (lon_delta / 2.0).sin().powf(2.0);
    let c_angle: f32 = 2.0 * angle.sqrt().atan2((1.0 - angle).sqrt());
    // Return central angle
    return c_angle;
}

// Calculate the miles between two points
pub fn calculate_m(coord1: Vec<Vec<f32>>, coord2: Vec<Vec<f32>>) -> f32 {
    // Radius of the Earth in miles
    let earth_rad: f32 = 3958.8;
    // Get central angle
    let angle: f32 = angle(coord1, coord2);
    // Get the distance by multiplying the central angle by the radius of the Earth
    let distance: f32 = earth_rad * angle;
    // Return the distance in miles
    return distance;
}

// Calculate the kilometers between two points
pub fn calculate_km(coord1: Vec<Vec<f32>>, coord2: Vec<Vec<f32>>) -> f32 {
    // Radius of the Earth in kilometers
    let earth_rad: f32 = 6371.0;
    // Get central angle
    let angle: f32 = angle(coord1, coord2);
    // Get the distance by multiplying the central angle by the radius of the Earth
    let distance: f32 = earth_rad * angle;
    // Return the distance in kilometers
    return distance;
}