use crate::datatypes::{OrderType, Side, TimeInForce};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NewOrder<'a> {
    product_code: &'a str,
    #[serde(rename = "child_order_type")]
    order_type: OrderType,
    side: Side,
    price: Decimal,
    size: f64,
    minute_to_expire: i64,
    time_in_force: TimeInForce,
}
