use crate::curb::CurbReceipt;

use versa_unstable_schema::receipt::{
    Currency, DiscountElement, DiscountType, Header, Interval, Itemization, Receipt, Subscription,
    SubscriptionItem, SubscriptionType,
};

/// parses an epoch time from a datetime of the format YYYY/MM/DD hh:mm:ss +offset (e.g. "2016/03/05 22:59:18 +0000")
pub fn parse_dt_to_unix(datetime: &str) -> Option<i64> {
    let fmt = "%Y/%m/%d %H:%M:%S %z";
    let dt = chrono::DateTime::parse_from_str(datetime, fmt);
    match dt {
        Ok(dt) => Some(dt.timestamp()),
        Err(_e) => None,
    }
}

pub fn transform_curb_receipt(receipt: CurbReceipt) -> Receipt {
    let bill = receipt.bill;

    Receipt {
        actions: Some(vec![]),
        header: Header {
            amount: bill.total,
            currency: bill.currency_code,
            customer: None,
            location: None,
            mcc: Some("4121".into()),
            receipt_id: receipt.id.to_string(),
            subtotal: Some(bill.items.fare),
            third_party: None,
            created_at: parse_dt_to_unix(&receipt.created_at)
                .expect("Improperly formatted created_at field"),
        },
        itemization: Itemization {
            general: Default::default(),
            lodging: Default::default(),
            ecommerce: Default::default(),
            car_rental: Default::default(),
            transit_route: Default::default(),
            subscription: Default::default(),
            flight: Default::default(),
        },
        payment: None,
        version: "0.2.0".into(),
    }
}

#[cfg(test)]
mod adapter_tests {
    use super::*;

    #[test]
    fn test_parse_dt_to_unix() {
        let dt = "2016/03/05 22:59:18 +0000";
        assert_eq!(parse_dt_to_unix(dt), Some(1457218758));
    }
}
