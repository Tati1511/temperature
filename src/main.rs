fn tell_temperature(temp: i32) {
    if temp <= 10 {
        println!("It's cold!");
    } else {
        if temp <= 25 {
            println!("It's nice");
        } else {
            if temp <= 30 {
                println!("It's warm");
            } else {
                println!("It's hot!")
            }
        }
    }
}



fn tell_temperature1(temp: i32) {
    if temp <= 10 {
        println!("It's cold!");
    } else if temp <= 25 {
        println!("It's nice");
    } else if temp <= 30 {
        println!("It's warm");
    } else {
        println!("It's hot!")
    }
}

fn tell_temperature3 (temp: i32) {
    let message = if temp <= 10 {
        "It's cold!"
    } else if temp <= 25 {
        "It's nice"
    } else if temp <= 30 {
        "It's warm"
    } else {
        "It's hot!"
    };

    println!("{}", message);
}

fn tell_temperature4(temp: i32) {
    let message = if temp <= 10 {
        "It's cold!"
    } else if temp <= 25 {
        "It's nice"
    } else if temp <= 30 {
        "It's warm"
    } else {
        "It's hot!"
    };

    println!("{}", message);
}

fn main() {
    tell_temperature(31);
    tell_temperature1(15);
    tell_temperature3(10);
    tell_temperature4(10);


}
