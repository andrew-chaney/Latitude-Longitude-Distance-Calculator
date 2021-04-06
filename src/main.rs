mod latlong_functions;

use std::io::stdin;

fn main() {
    // Print program welcome header
    latlong_functions::header();

    // Get Unit of Measurement from user
    let measurement = latlong_functions::get_units();
    if measurement == "k" {
        println!("Units Selected: kilometers");
    } else {
        println!("Units Selected: miles");
    }

    println!();
    // Get the first location from the user
    println!("First location...");
    let coord1: Vec<Vec<f32>>; // to maintain scope
    loop {
        // Get lat and long for first location
        let coord = latlong_functions::get_coordinates();
        println!();
        // Print lat/long in easy to read format
        println!("The location you entered is: ");
        latlong_functions::beautify(&coord);
        let mut correct: String = String::new();
        // Verify accuracy with the user
        println!("Is this location correct (y/n): ");
        stdin().read_line(&mut correct).expect("ERROR: unable to read line");
        // If it is accurate, continue
        let correct = correct.trim().to_lowercase();
        if correct == "y" {
            coord1 = coord;
            println!();
            break;
        }
        // If not accurate, have user re-enter coordinates
        else {
            println!("Let's redo it then.");
            println!();
        }
    }

    println!("Second Location...");
    // Get the second location from the user
    let coord2: Vec<Vec<f32>>; // to maintain scope
    loop {
        // Get lat and long for second location
        let coord = latlong_functions::get_coordinates();
        println!();
        // Print lat/long in easy to read format
        println!("The location you entered is: ");
        latlong_functions::beautify(&coord);
        let mut correct: String = String::new();
        // Verify accuracy with the user
        println!("Is this location correct (y/n): ");
        stdin().read_line(&mut correct).expect("ERROR: unable to read line");
        // If it is accurate, continue
        let correct = correct.trim().to_lowercase();
        if correct == "y" {
            coord2 = coord;
            break;
        }
        // If not accurate, have user re-enter coordinates
        else {
            println!("Let's redo it then.");
            println!();
        }
    }

    // Calculate the distance between the two points
    let distance: f32;
    // In miles
    if measurement == "m" {
        distance = latlong_functions::calculate_m(coord1, coord2);
        println!("Distance: {:.2} miles", distance);
    } 
    // Or in kilometers
    else {
        distance = latlong_functions::calculate_km(coord1, coord2);
        println!("Distance: {:.2} kilometers", distance);
    }
}
