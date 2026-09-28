use dw09::{average_dollars, budget_status};

fn main() {
    let total_cents = 750;
    let count = 4;
    let average = average_dollars(total_cents, count);

    println!("total_cents: {}\ncount: {}", total_cents, count);
    println!("Average: {:.2}", average);
    println!("{}", budget_status(average, 1.50));
}
