fn main() {
    // range is NOT inclusive (so this goes from 0 to 4, not 5)
    for i in 0..5 {
        println!("Current number: {i}");
    }

    for x in 0..11 {
        if x == 5 {
            println!("We're on 5!");
        } else {
            println!("Not 5  :(")
        }
    }
}
