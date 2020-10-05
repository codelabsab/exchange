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
struct Order {
    id: Uuid,
    direction: Direction,
    symbol: Symbol,
    shares: u32,
    limit: u32,
    entry_time: Option<std::time::Instant>,
    event_time: Option<std::time::Instant>,
    duration: Option<std::time::Duration>,
    next: Option<Box<Order>>,
    previous: Option<Box<Order>>,
}

impl Order {
    /*
        Add new Order and attach to correct Limit. If Limit doesn't exist we need to create it.
    */
    fn new(direction: Direction, symbol: Symbol, shares: u32, limit: u32) -> Order {
        Order {
            id: Uuid::new_v4(),
            direction,
            symbol,
            shares,
            limit,
            entry_time: None,
            event_time: Some(Instant::now()),
            duration: None,
            next: None,
            previous: None,
        }
    }
}

/*
    A Limit represent a price point
    example: 100.0 USD

    We will create a binary tree of limits

    Each Limit is a FIFO queue of orders for that price point.Limit

    We store head and tail of order to get O(1) constant time lookup
    and insertion for head or tail
*/
struct Limit {
    price: u32,
    left: Option<Box<Limit>>,
    right: Option<Box<Limit>>,
    head: Option<Box<Order>>,
    tail: Option<Box<Order>>,
}

impl Limit {
    fn new(price: u32) -> Limit {
        Limit {
            price: price,
            left: None,
            right: None,
            head: None,
            tail: None,
        }
    }

    fn insert(&mut self, price: u32) {
        let new_node = Some(Box::new(Limit::new(price)));
        if price < self.price {
            match self.left.as_mut() {
                None => self.left = new_node,
                Some(left) => left.insert(price),
            }
        } else {
            match self.right.as_mut() {
                None => self.right = new_node,
                Some(right) => right.insert(price),
            }
        }
    }

    fn search(&self, target: u32) -> Option<u32> {
        match self.price {
            value if target == value => Some(value),
            value if target < value => self.left.as_ref()?.search(target),
            value if target > value => self.right.as_ref()?.search(target),
            _ => None,
        }
    }
}

/*
    A Book is a representation of the order book which consist of buy orders and sell orders
    that are implemented as binary trees
*/
struct Book<'a> {
    symbol: Symbol,
    buy: Option<&'a Limit>,
    sell: Option<&'a Limit>,
    lowest: Option<&'a Order>,
    highest: Option<&'a Order>,
}

/*
    Initiate new empty Book
*/
impl<'a> Book<'a> {
    pub fn new(symbol: Symbol) -> Book<'a> {
        Book {
            symbol,
            buy: None,
            sell: None,
            lowest: None,
            highest: None,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Limit Order Book for AAPL
    let mut book1 = Book::new(Symbol::AAPL);
    book1.symbol = Symbol::AAPL;

    // Order1
    let order1 = Order::new(Direction::Buy, Symbol::AAPL, 100, 116);
    let mut limit1 = Limit::new(116);
    limit1.insert(order1.limit);

    // add Limit to Book
    book1.buy = Some(&limit1);

    // Order 2
    let order2 = Order::new(Direction::Buy, Symbol::AAPL, 50, 116);
    let mut limit2 = Limit::new(order2.limit);
    limit2.insert(order1.limit);
    Ok(())
}
