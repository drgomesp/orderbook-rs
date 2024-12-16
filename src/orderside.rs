use crate::orderbook::OrderPtr;
use std::collections::{BTreeMap, HashMap, VecDeque};
use std::sync::{Arc, Mutex};

pub type OrderQueuePtr = Arc<Mutex<OrderQueue>>;

#[derive(Debug)]
pub struct OrderQueue {
    orders: VecDeque<OrderPtr>,
    pub volume: f64,
}

impl OrderQueue {
    fn new() -> OrderQueue {
        OrderQueue {
            orders: VecDeque::new(),
            volume: 0.0,
        }
    }

    fn add(&mut self, order: &OrderPtr) {
        self.orders.push_back(order.clone());
        self.volume += order.lock().unwrap().volume;
    }
}

#[derive(Debug)]
pub struct OrderSide {
    #[doc = r"`price_tree` is an ordered map (binary tree) where the key
is the price and the value is the order list for that price."]
    pub price_tree: BTreeMap<String, OrderQueuePtr>,

    /// `price_map` is a hash map where the key is the price
    /// and the value is the order list for that price.
    price_map: HashMap<String, OrderQueuePtr>,

    length: usize,
    depth: usize,
    volume: f64,
}

impl OrderSide {
    /// `add_order` adds an order to the order side, keeping track of the order in two different
    /// places, `price_tree` and `price_map`, where orders are kept in a queue for each price level.
    pub fn add_order(&mut self, order: &OrderPtr) {
        let (price, volume);

        {
            let order = order.lock().expect("order lock failed");
            price = order.price;
            volume = order.volume;
        }

        if let Some(queue) = self.price_map.get_mut(price.to_string().as_str()) {
            let mut queue = queue.lock().expect("order queue lock failed");
            queue.add(order);
        } else {
            let mut queue = OrderQueue::new();
            queue.add(order);

            let queue = Arc::new(Mutex::new(queue));

            self.price_map.insert(price.to_string(), queue.clone());
            self.price_tree.insert(price.to_string(), queue.clone());

            self.depth += 1;
        }

        self.length += 1;
        self.volume += volume;
    }

    pub fn get_max_price(&mut self) -> Option<f64> {
        if let Some(entry) = self.price_tree.last_entry() {
            let price = entry.key().parse::<f64>().expect("failed to parse price");
            Some(price)
        } else {
            None
        }
    }

    pub fn get_min_price(&mut self) -> Option<f64> {
        if let Some(entry) = self.price_tree.first_entry() {
            let price = entry.key().parse::<f64>().expect("failed to parse price");
            Some(price)
        } else {
            None
        }
    }
}

impl OrderSide {
    pub fn new() -> OrderSide {
        OrderSide {
            price_tree: BTreeMap::new(),
            price_map: HashMap::new(),
            length: 0,
            depth: 0,
            volume: 0.0,
        }
    }
}
