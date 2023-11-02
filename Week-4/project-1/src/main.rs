fn main() {
    let speed_1 = calculate_speed(80, 2);
    let speed_2 = calculate_speed(120, 4);

    println!("The speed of the car if it goes 80 miles in 2 hours is {} kilometers/hour.", speed_1);
    println!("The speed of the car if it goes 120 miles in 4 hours is {} kilometers/hour.", speed_2);
}

fn calculate_speed(distance: u32, time: u32) -> f64 {
    let speed = (distance as f64) / (time as f64);
    let speed_km = speed * 1.60934; // Converting miles to kilometers
    speed_km
}
