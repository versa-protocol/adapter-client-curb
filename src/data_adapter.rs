use crate::curb::{CurbReceipt, Items};

use versa_unstable_schema::receipt::{
    AddressClass, Header, Itemization, Receipt, TaxElement, TransitRoute,
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
            subtotal: bill.items.fare,
            third_party: None,
            created_at: parse_dt_to_unix(&receipt.created_at)
                .expect("Improperly formatted created_at field"),
        },
        itemization: Itemization {
            general: Default::default(),
            lodging: Default::default(),
            ecommerce: Default::default(),
            car_rental: Default::default(),
            transit_route: Some(TransitRoute {
                departure_address: Some(transform_address(receipt.pickup_location)),
                arrival_address: Some(transform_address(receipt.dropoff_location)),
                arrival_time: parse_dt_to_unix(&receipt.created_at),
                departure_time: parse_dt_to_unix(&receipt.pickup_time),
                invoice_level_discounts: None,
                metadata: None,
                polyline: None,
                taxes: Some(transform_taxes(&bill.items)),
                tip: bill.items.tip,
            }),
            subscription: Default::default(),
            flight: Default::default(),
        },
        payment: None,
        version: "0.2.0".into(),
    }
}

fn transform_address(location: crate::curb::Location) -> AddressClass {
    AddressClass {
        street_address: Some(match location.line2 {
            Some(line2) => format!("{} {}", location.line1, line2),
            None => location.line1,
        }),
        city: Some(location.city),
        region: Some(location.state),
        postal_code: Some(location.postal_code),
        country: "US".into(),
        lat: location.latitude,
        lon: location.longitude,
    }
}

// One object for every non-null value of: improvement_surcharge, state_surcharge, workers_comp, airport_fee, congestion_surcharge, sales_tax

fn transform_taxes(items: &Items) -> Vec<TaxElement> {
    let mut taxes = vec![];
    if let Some(improvement_surcharge) = items.improvement_surcharge {
        taxes.push(TaxElement {
            amount: improvement_surcharge,
            name: "Improvement Surcharge".into(),
            rate: None,
        });
    }
    if let Some(state_surcharge) = items.state_surcharge {
        taxes.push(TaxElement {
            amount: state_surcharge,
            name: "State Surcharge".into(),
            rate: None,
        });
    }
    if let Some(sales_tax) = items.sales_tax {
        taxes.push(TaxElement {
            amount: sales_tax,
            name: "Sales Tax".into(),
            rate: None,
        });
    }
    taxes
}

#[cfg(test)]
mod adapter_tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_dt_to_unix() {
        let dt = "2016/03/05 22:59:18 +0000";
        assert_eq!(parse_dt_to_unix(dt), Some(1457218758));
    }

    #[test]
    fn test_transform_address() {
        let location = crate::curb::Location {
            line1: "123 Main St".into(),
            line2: Some("Apt 2".into()),
            city: "New York".into(),
            state: "NY".into(),
            postal_code: "10001".into(),
            latitude: 40.7128,
            longitude: -74.0060,
        };
        let address = transform_address(location);
        assert_eq!(address.street_address, Some("123 Main St Apt 2".into()));
        assert_eq!(address.city, Some("New York".into()));
        assert_eq!(address.region, Some("NY".into()));
        assert_eq!(address.postal_code, Some("10001".into()));
        assert_eq!(address.country, "US");
        assert_eq!(address.lat, 40.7128);
        assert_eq!(address.lon, -74.0060);

        let location2 = crate::curb::Location {
            line1: "123 Main St".into(),
            line2: None,
            city: "New York".into(),
            state: "NY".into(),
            postal_code: "10001".into(),
            latitude: 40.7128,
            longitude: -74.0060,
        };
        let address2 = transform_address(location2);
        assert_eq!(address2.street_address, Some("123 Main St".into()));
    }

    #[test]
    fn test_transform_taxes() {
        let items = Items {
            service_fee: None,
            fare: None,
            improvement_surcharge: Some(100),
            state_surcharge: Some(200),
            sales_tax: Some(300),
            tip: None,
            toll: None,
        };
        let taxes = transform_taxes(&items);
        assert_eq!(taxes.len(), 3);
        assert_eq!(taxes[0].amount, 100);
        assert_eq!(taxes[0].name, "Improvement Surcharge");
        assert_eq!(taxes[1].amount, 200);
        assert_eq!(taxes[1].name, "State Surcharge");
        assert_eq!(taxes[2].amount, 300);
        assert_eq!(taxes[2].name, "Sales Tax");
    }
}
