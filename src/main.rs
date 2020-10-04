use std::error::Error;
use std::time::{Duration, Instant};
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

/*
    Orders
*/
struct Order<'a> {
    id: Uuid,
    direction: Direction,
    symbol: Symbol,
    shares: usize,
    limit: f32,
    entry_time: Option<std::time::Instant>,
    event_time: Option<std::time::Duration>,
    next: Option<Box<&'a Order<'a>>>,
    previous: Option<Box<&'a Order<'a>>>,
}

/*
    A Limit represent a price point
    example: 100.0 USD

    We will create a binary tree of limits

    Each Limit is a FIFO queue of orders for that price point.Limit

    We store head and tail of order to get O(1) constant time lookup
    and insertion for head or tail
*/
struct Limit<'a> {
    price: &'a f64,
    left: Option<&'a Limit<'a>>,
    right: Option<&'a Limit<'a>>,
    head: Option<Box<&'a Order<'a>>>,
    tail: Option<Box<&'a Order<'a>>>,
}

impl<'a> Limit<'a> {
    pub fn insert(&self, order: &f32) -> Result<(), Box<dyn Error>> {
        println!("Not implemented!");
        Ok(())
    }
}

/*
    A Book is a representation of the order book which consist of buy orders and sell orders
    that are implemented as binary trees
*/
struct Book<'a> {
    buy: Option<&'a Limit<'a>>,
    sell: Option<&'a Limit<'a>>,
    lowest: Option<&'a Order<'a>>,
    highest: Option<&'a Order<'a>>,
}

/*
    Initiate new empty Book
*/
impl<'a> Book<'a> {
    pub fn new() -> Book<'a> {
        Book {
            buy: None,
            sell: None,
            lowest: None,
            highest: None,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let book = Book::new();
    Ok(())
}
