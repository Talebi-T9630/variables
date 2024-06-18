const READY_AMOUNT: i32 = 2;
const STARTING_MISSILES: i32 = 8;

fn main() {
    // let mut missiles: i32 = STARTING_MISSILES ; 
    // let ready: i32 = READY_AMOUNT ; 

    let (mut missiles,ready,go): (i32,i32,i32) = (STARTING_MISSILES,READY_AMOUNT,0);
    println!("Firing {} of my {} missiles...", ready, missiles);
    // missiles = missiles - ready;
    println!("{} missiles left", (missiles - ready));
}
