fn main() {
    let mut counter = 0;

    let ret = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(ret, 20);
}