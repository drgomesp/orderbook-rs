use crate::order::Order;
use crate::order::OrderKind::{Buy, Sell};
use crate::orderbook::OrderBook;
use std::time::Instant;

pub mod order;
pub mod orderbook;
mod orderside;

fn main() {
    let mut orderbook = OrderBook::new();

    let mut timer = Instant::now();
    let mut count = 0;

    for i in 0..3_000_000 {
        orderbook
            .add_order(Order::new(
                if i % 2 == 0 { Sell } else { Buy },
                format!("order-{i}"),
                99.95,
                10.0,
            ))
            .expect("failed to add order");

        count += 1;

        if timer.elapsed().as_secs() >= 1 {
            println!("{count} adds/s");

            count = 0;
            timer = Instant::now();
        }
    }

    // println!("{orderbook}");
}

#[cfg(test)]
mod tests {
    use crate::order::Order;
    use crate::order::OrderKind::{Buy, Sell};
    use crate::orderbook::OrderBook;

    #[test]
    fn test_create_buy_order() {
        let order = Order::new(Buy, "buy-1".into(), 100.0, 10.0);

        assert_eq!(order.kind, Buy);
        assert_eq!(order.id, "buy-1");
        assert!(order.price.eq(&100.0));
        assert!(order.volume.eq(&10.0));
    }

    #[test]
    fn test_create_sell_order() {
        let order = Order::new(Sell, "sell-1".into(), 50.0, 5.0);

        assert_eq!(order.kind, Sell);
        assert_eq!(order.id, "sell-1");
        assert!(order.price.eq(&50.0));
        assert!(order.volume.eq(&5.0));
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
    fn test_orderbook() {
        let mut ob = OrderBook::new();

        let s1 = Order::new(Sell, "sell-1".into(), 99.95, 10.0);
        let s2 = Order::new(Sell, "sell-2".into(), 99.95, 20.0);
        let s3 = Order::new(Sell, "sell-3".into(), 99.95, 30.0);

        let b1 = Order::new(Buy, "buy-1".into(), 99.95, 10.0);
        let b2 = Order::new(Buy, "buy-2".into(), 99.95, 20.0);

        ob.add_order(s1).expect("add order failed");
        ob.add_order(s2).expect("add order failed");
        ob.add_order(s3).expect("add order failed");

        ob.add_order(b1).expect("add order failed");
        ob.add_order(b2).expect("add order failed");

        let volume_at_ask_price = ob.get_volume_at_ask_price(99.95).unwrap();
        let volume_at_bid_price = ob.get_volume_at_bid_price(99.95).unwrap();

        assert!(volume_at_ask_price.eq(&60.0));
        assert!(volume_at_bid_price.eq(&30.0));
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

        assert!(best_ask.eq(&99.94));
        assert!(best_bid.eq(&99.95));
    }
}
