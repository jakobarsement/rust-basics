// consts can be declared in global scope
const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    // can bind multiple vars in one line (tuples)
    let (missles, ready) = (STARTING_MISSILES, READY_AMOUNT);

    // STARTING_MISSILES = 2; cant change cosnts

    println!("Firing {} of my {} missles...", ready, missles);

    println!("{} missles left", missles - ready);
}
