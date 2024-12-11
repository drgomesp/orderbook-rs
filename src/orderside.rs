use crate::order::Order;
use std::collections::{BTreeMap, HashMap, LinkedList};
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct OrderQueue {
    orders: LinkedList<Arc<Mutex<Order>>>,
    pub volume: f64,
}

impl OrderQueue {
    fn new() -> OrderQueue {
        OrderQueue {
            orders: LinkedList::new(),
            volume: 0.0,
        }
    }

    fn add(&mut self, order: Arc<Mutex<Order>>) {
        let order = order.lock().unwrap();

        self.volume += order.volume;
        self.orders.push_back(Arc::new(Mutex::new(order.clone())));
    }
}

#[derive(Debug)]
pub struct OrderSide {
    /// `price_tree` is an ordered map (binary tree) where the key
    /// is the price and the value is the order list for that price.
    pub price_tree: BTreeMap<String, Arc<Mutex<OrderQueue>>>,

    /// `price_map` is a hash map where the key is the price
    /// and the value is the order list for that price.
    price_map: HashMap<String, Arc<Mutex<OrderQueue>>>,

    length: usize,
    depth: usize,
    volume: f64,
}

impl OrderSide {
    /// `add_order` adds an order to the order side, keeping track of the order in two different
    /// places, `price_tree` and `price_map`, where orders are kept in a queue for each price level.
    pub fn add_order(&mut self, order: Arc<Mutex<Order>>) {
        let clone = order.clone();
        let order = order.lock().expect("order lock failed");
        let (price, volume) = (order.price, order.volume);
        drop(order);

        match self.price_map.get_mut(price.to_string().as_str()) {
            Some(queue) => {
                let mut queue = queue.lock().expect("order queue lock failed");

                queue.add(clone)
            }
            None => {
                let mut queue = OrderQueue::new();
                queue.add(clone);

                let queue = Arc::new(Mutex::new(queue));

                self.price_map.insert(price.to_string(), queue.clone());
                self.price_tree.insert(price.to_string(), queue.clone());
                self.depth += 1;
            }
        }

        self.length += 1;
        self.volume += volume;
    }

    pub fn get_max_price(&mut self) -> Option<String> {
        if let Some(entry) = self.price_tree.last_entry() {
            Some(entry.key().clone())
        } else {
            None
        }
    }

    pub fn get_min_price(&mut self) -> Option<String> {
        if let Some(entry) = self.price_tree.first_entry() {
            Some(entry.key().clone())
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
