use uuid::Uuid;

/*
    BBO = Best bid and Ask
*/

enum Direction {
    Buy,
    Sell,
}

enum Symbol {
    FB,
    AAPL,
    AMZN,
    NFLX,
    GOOG,
}

struct Order {
    id: Uuid,
    direction: Direction,
    symbol: Symbol,
    shares: usize,
    limit: f32,
    entry_time: f64,
    event_time: f64,
    next: Box<Option<Order>>,
    previous: Box<Option<Order>>,
    parrent_limit: Box<Option<Limit>>,
}

struct Limit {}

fn main() {
    println!("Hello, world!");
}
