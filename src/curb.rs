use serde::{Deserialize, Serialize};
use versa_unstable_schema::receipt::Currency;

#[derive(Debug, Serialize, Deserialize)]
pub struct Ride {
    pub id: i64,
    pub pickup_location: Location,
    pub dropoff_location: Location,
    pub status: Status,
    pub vehicle: Vehicle,
    pub bill: Bill,
    pub metadata: Metadata,
    pub rider: Rider,
    pub details: Details,
    pub created_at: String,
    pub pickup_time: String,
    pub driver_phone: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bill {
    pub items: Items,
    pub total: i64,
    pub currency_code: Currency,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Items {
    pub service_fee: Option<i64>,
    pub fare: Option<i64>,
    pub improvement_surcharge: Option<i64>,
    pub state_surcharge: Option<i64>,
    pub sales_tax: Option<i64>,
    pub tip: Option<i64>,
    pub toll: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Details {
    pub external_ride_id: String,
    pub is_no_show: bool,
    pub is_wav_ride: bool,
    pub is_flat_fare: bool,
    pub notes: Option<serde_json::Value>,
    pub cancel_fee: i64,
    pub copay: Option<serde_json::Value>,
    pub streethail: bool,
    pub partner_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub line1: String,
    pub line2: Option<String>,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub external_ride_id: String,
    pub external_user_id: String,
    pub other_example_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rider {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    pub status_code: String,
    pub distance_from_pickup: f64,
    pub distance_units: String,
    pub vehicle_latitude: f64,
    pub vehicle_longitude: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Vehicle {
    pub driver_id: String,
    pub driver_incentive: i64,
    pub driver_name: String,
    pub vehicle_number: String,
    pub estimated_fare: i64,
    pub vehicle_type_subclass: String,
}
