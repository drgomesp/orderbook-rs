pub mod order;
pub mod orderbook;
mod orderside;

fn main() {
    let mut ob = OrderBook::new();
    let s1 = Order::new(Sell, "sell-1".into(), 99.94, 10.0);
    let s2 = Order::new(Sell, "sell-2".into(), 99.96, 25.0);
    let s3 = Order::new(Sell, "sell-3".into(), 99.98, 20.0);
    let s4 = Order::new(Sell, "sell-4".into(), 99.94, 25.0);

    let b1 = Order::new(Buy, "buy-1".into(), 99.93, 10.0);
    let b2 = Order::new(Buy, "buy-2".into(), 99.95, 20.0);
    let b3 = Order::new(Buy, "buy-3".into(), 99.95, 10.0);

    let _ = ob.add_order(s1).expect("add order failed");
    let _ = ob.add_order(s2).expect("add order failed");
    let _ = ob.add_order(s3).expect("add order failed");
    let _ = ob.add_order(s4).expect("add order failed");

    let _ = ob.add_order(b1).expect("add order failed");
    let _ = ob.add_order(b2).expect("add order failed");
    let _ = ob.add_order(b3).expect("add order failed");

    println!("{ob}")
}

use crate::order::Order;
use crate::order::OrderKind::{Buy, Sell};
use crate::orderbook::OrderBook;

mod tests {
    use crate::order::Order;
    use crate::order::OrderKind::{Buy, Sell};
    use crate::orderbook::OrderBook;

    #[test]
    fn test_create_buy_order() {
        let order = Order::new(Buy, "buy-1".into(), 100.0, 10.0);

        assert_eq!(order.kind, Buy);
        assert_eq!(order.id, "buy-1");
        assert_eq!(order.price, 100.0);
        assert_eq!(order.volume, 10.0);
    }

    #[test]
    fn test_create_sell_order() {
        let order = Order::new(Sell, "sell-1".into(), 50.0, 5.0);

        assert_eq!(order.kind, Sell);
        assert_eq!(order.id, "sell-1");
        assert_eq!(order.price, 50.0);
        assert_eq!(order.volume, 5.0);
    }

    #[test]
    fn test_add_duplicate_order_error() {
        let mut ob = OrderBook::new();
        let order = Order::new(Sell, "sell-1".into(), 50.0, 5.0);

        let _ = ob.add_order(order.clone());
        let result = ob.add_order(order);

        assert!(result.is_err(), "expected Err for duplicate order id.");
    }

    #[test]
    fn test_orderbook_best_ask_bid() {
        let mut ob = OrderBook::new();

        let s1 = Order::new(Sell, "sell-1".into(), 99.94, 10.0);
        let s2 = Order::new(Sell, "sell-2".into(), 99.96, 20.0);
        let s3 = Order::new(Sell, "sell-3".into(), 99.98, 30.0);

        let b1 = Order::new(Buy, "buy-1".into(), 99.93, 10.0);
        let b2 = Order::new(Buy, "buy-2".into(), 99.95, 20.0);

        let _ = ob.add_order(s1);
        let _ = ob.add_order(s2);
        let _ = ob.add_order(s3);

        ob.add_order(b1).expect("add order failed");
        ob.add_order(b2).expect("add order failed");

        let best_ask = ob.get_best_ask().expect("best ask failed");
        let best_bid = ob.get_best_bid().expect("best bid failed");

        assert_eq!(ob.get_best_ask().unwrap(), "99.94");
        assert_eq!(ob.get_best_bid().unwrap(), "99.95");
    }
}
