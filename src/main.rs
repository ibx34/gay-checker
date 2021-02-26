fn main() {
    let is_gay = true;
    let check_gay = |gay: bool| {
        if gay{
            println!("IsGay")
        } else {
            println!("Is not gay")
        }
    };
    check_gay(is_gay)
}
