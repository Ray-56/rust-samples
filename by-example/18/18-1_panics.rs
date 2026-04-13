fn give_princess(gift: &str) {
    // The princess hates snakes, so if the princess expresses disgust we have to stop!
    if gift == "snake" { panic!("AAAAaaaaa!!!!!"); }

    println!("I love {}s!!!!", gift);
}

fn main() {
    give_princess("teddy bear");
    give_princess("snake");
}