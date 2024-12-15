use crate::order::{Order, OrderKind};
use crate::orderside::{OrderQueue, OrderQueuePtr, OrderSide};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::sync::{Arc, Mutex};

const DISPLAY_NUM_COLS: usize = 30;

type Trade = ();

pub type OrderPtr = Arc<Mutex<Order>>;

#[derive(Debug)]
pub enum OrderBookError {
    OrderAlreadyExists(String),
    OrderVolumeZero,
    OrderPriceZero,
}

impl Display for OrderBookError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderBookError::OrderAlreadyExists(id) => {
                write!(f, "order with id {id} already exists")
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
    orders: HashMap<String, OrderPtr>,

    /// The buy orders side.
    bids: OrderSide,

    /// The sell orders side.
    asks: OrderSide,
}

impl Default for OrderBook {
    fn default() -> Self {
        Self::new()
    }
}

impl OrderBook {
    #[must_use]
    pub fn new() -> Self {
        OrderBook {
            orders: HashMap::new(),
            bids: OrderSide::new(),
            asks: OrderSide::new(),
        }
    }

    /// # Errors
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
            OrderKind::Buy => self.bids.add_order(&order),
            OrderKind::Sell => self.asks.add_order(&order),
        }

        self.orders.insert(id, order);

        self.match_orders()
    }

    pub fn get_best_ask(&mut self) -> Option<f64> {
        self.asks.get_min_price()
    }

    pub fn get_best_bid(&mut self) -> Option<f64> {
        self.bids.get_max_price()
    }

    /// # Panics
    #[must_use]
    pub fn get_volume_at_ask_price(&self, price: f64) -> Option<f64> {
        match self.asks.price_tree.get(&price.to_string()) {
            Some(queue) => {
                let orders = queue.lock().expect("order queue lock failed");

                Some(orders.volume)
            }
            None => None,
        }
    }

    /// # Panics
    #[must_use]
    pub fn get_volume_at_bid_price(&self, price: f64) -> Option<f64> {
        match self.bids.price_tree.get(&price.to_string()) {
            Some(queue) => {
                let orders = queue.lock().expect("order queue lock failed");

                Some(orders.volume)
            }
            None => None,
        }
    }

    fn match_orders(&self) -> Result<Vec<Trade>, OrderBookError> {
        let trades = Vec::new();

        Ok(trades)
    }
}

impl Display for OrderBook {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Asks ")?;
        for _ in 0..DISPLAY_NUM_COLS {
            write!(f, "-")?;
        }
        writeln!(f)?;

        display_orderside(&self.asks, true);

        write!(f, "Bids ")?;
        for _ in 0..DISPLAY_NUM_COLS {
            write!(f, "-")?;
        }
        writeln!(f, "-")?;

        display_orderside(&self.bids, true);
        Ok(())
    }
}

fn display_orderside(order_side: &OrderSide, rev: bool) {
    let iter: Box<dyn Iterator<Item = (&String, &OrderQueuePtr)>> = if rev {
        Box::new(order_side.price_tree.iter().rev())
    } else {
        Box::new(order_side.price_tree.iter())
    };

    for (k, v) in iter {
        let queue = v.lock().expect("order queue lock failed");

        let mut col_count = 0;
        for _ in 1..(queue.volume as u64 / 2) {
            col_count += 1;
        }

        for i in 0..DISPLAY_NUM_COLS - 5 {
            if i >= col_count {
                print!(" ");
            }
        }

        print!("{} ", queue.volume);
        for _ in 1..(queue.volume as u64) / 2 {
            print!("█");
        }

        println!("{k}");
    }
}
