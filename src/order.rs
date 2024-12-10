use std::time::SystemTime;

#[derive(Clone, Debug, PartialEq)]
pub enum OrderKind {
    Buy,
    Sell,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Order {
    pub kind: OrderKind,
    pub id: String,
    pub price: f64,
    pub volume: f64,
    pub time: SystemTime,
}

impl Order {
    pub(crate) fn new(kind: OrderKind, id: String, price: f64, volume: f64) -> Order {
        Order {
            kind,
            id,
            price,
            volume,
            time: SystemTime::now(),
        }
    }
}
