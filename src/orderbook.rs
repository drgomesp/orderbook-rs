use crate::order::{Order, OrderKind};
use crate::orderside::{OrderQueue, OrderSide};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::sync::{Arc, Mutex};

const DISPLAY_NUM_COLS: usize = 30;

type Trade = ();

#[derive(Debug)]
pub enum OrderBookError {
    OrderAlreadyExists(String),
    OrderVolumeZero,
    OrderPriceZero,
}

impl fmt::Display for OrderBookError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderBookError::OrderAlreadyExists(id) => {
                write!(f, "order with id {} already exists", id)
            }
            OrderBookError::OrderVolumeZero => {
                write!(f, "order volume must be greater than zero")
            }
            OrderBookError::OrderPriceZero => {
                write!(f, "order price must be greater than zero")
            }
        }
    }
}

impl Error for OrderBookError {}

#[derive(Debug)]
pub struct OrderBook {
    /// `orders` is a hash map of orders where the key is the order id.
    orders: HashMap<String, Arc<Mutex<Order>>>,

    /// The buy orders side.
    bids: OrderSide,

    /// The sell orders side.
    asks: OrderSide,
}

impl OrderBook {
    pub fn new() -> OrderBook {
        OrderBook {
            orders: HashMap::new(),
            bids: OrderSide::new(),
            asks: OrderSide::new(),
        }
    }

    pub fn add_order(&mut self, order: Order) -> Result<Vec<Trade>, OrderBookError> {
        let (id, kind) = (order.id.clone(), order.kind.clone());

        if self.orders.contains_key(&id) {
            return Err(OrderBookError::OrderAlreadyExists(id));
        }

        if order.volume == 0.0 {
            return Err(OrderBookError::OrderVolumeZero);
        }

        if order.price == 0.0 {
            return Err(OrderBookError::OrderPriceZero);
        }

        let order = Arc::new(Mutex::new(order));

        match kind {
            OrderKind::Buy => self.bids.add_order(order.clone()),
            OrderKind::Sell => self.asks.add_order(order.clone()),
        }

        self.orders.insert(id, order);

        self.match_orders()
    }

    pub fn get_best_ask(&mut self) -> Option<String> {
        self.asks.get_min_price()
    }

    pub fn get_best_bid(&mut self) -> Option<String> {
        self.bids.get_max_price()
    }

    fn match_orders(&self) -> Result<Vec<Trade>, OrderBookError> {
        let trades = Vec::new();

        Ok(trades)
    }

    fn display_orderside(&self, order_side: &OrderSide, rev: bool) {
        let iter: Box<dyn Iterator<Item = (&String, &Arc<Mutex<OrderQueue>>)>> = match rev {
            true => Box::new(order_side.price_tree.iter().rev()),
            false => Box::new(order_side.price_tree.iter()),
        };

        for (k, v) in iter {
            let queue = v.lock().expect("order queue lock failed");

            let mut col_count = 0;
            for _ in (1..(queue.volume / 2f64) as u64) {
                col_count += 1;
            }

            for i in 0..DISPLAY_NUM_COLS - 5 {
                if i >= col_count {
                    print!(" ");
                } else {
                }
            }

            print!("{} ", queue.volume);
            for _ in (0..(queue.volume / 2f64) as u64) {
                print!("█");
            }

            println!(" {}", k);
        }
    }
}

impl Display for OrderBook {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        print!("Asks ");
        for i in 0..DISPLAY_NUM_COLS {
            print!("-")
        }
        println!();

        self.display_orderside(&self.asks, true);

        print!("Bids ");
        for i in 0..DISPLAY_NUM_COLS {
            print!("-")
        }
        println!();

        Ok(self.display_orderside(&self.bids, false))
    }
}
