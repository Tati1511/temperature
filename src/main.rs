fn tell_temperature (temperature:i32)
{

    if temperature >= 30 {
        println!("It's hot!");
    } else {
        println!("It's not hot!");
    }
}


fn main() {
    tell_temperature(10);
    tell_temperature(20);
    tell_temperature(30);
    tell_temperature(40);
}