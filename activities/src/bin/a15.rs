// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

enum Ticket {
    Backstage(f64, String),
    Vip(f64, String),
    Standard(f64),
}

fn main() {
    let tickets: Vec<Ticket> = vec![
        Ticket::Backstage(50.0, "A".to_owned()),
        Ticket::Vip(15.0, String::from("B")),
        Ticket::Standard(30.5),
    ];

    for ticket in tickets {
        match ticket {
            Ticket::Backstage(price, holder) => {
                println!("Backstage ticket Holder: {:?}, price: {:?}", holder, price)
            }
            Ticket::Standard(price) => {
                println!("Standard ticket Price: {:?}", price)
            }
            Ticket::Vip(price, holder) => {
                println!("VIP ticket Holder: {:?}, price: {:?}", holder, price)
            }
        }
    }
}
