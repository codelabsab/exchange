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

struct Limit {
    price: f64,
    size: u32,   // is this needed?
    volume: u32, // is this needed?
    parrent: Box<Option<Limit>>,
    left_child: Box<Option<Limit>>,
    right_child: Box<Option<Limit>>,
    head_order: Box<Option<Order>>,
    tail_order: Box<Option<Order>>,
}

struct Book {
    // not implemented
}

fn main() {
    println!("Hello, world!");
}
