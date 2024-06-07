use axum::extract::Json;

use crate::protocol::encrypt_and_send;

#[derive(Debug, serde::Deserialize)]
pub struct Payload {
    pub ride: crate::curb::Ride,
    pub customer_email: Option<String>,
}

pub async fn target(
    Json(body): Json<Payload>,
) -> Result<http::StatusCode, (http::StatusCode, String)> {
    // 1. Transform Curb ride into the Versa receipt schema
    let customer_email = body.customer_email.clone();
    let receipt = crate::data_adapter::transform_curb_receipt(body.ride);
    let sender_client_id = std::env::var("CLIENT_ID").unwrap();
    info!("Received invoice for customer email: {:?}", customer_email);

    // 2. Encrypt, hash, and register with Versa registry

    let registration_hash = crate::encryption::generate_hash(&receipt);

    // // Authorized receivers subscribed to this email or domain will be returned by the registry
    // let routing_info = crate::model::RoutingInfo {
    //     customer_email,
    //     ..Default::default()
    // };

    let sender_client_secret = std::env::var("CLIENT_SECRET").unwrap_or_default();

    let response = crate::protocol::register(
        &sender_client_id,
        &sender_client_secret,
        customer_email,
        registration_hash,
    )
    .await
    .map_err(|e| {
        info!("Registration failed: {:?}", e);
        (
            http::StatusCode::SERVICE_UNAVAILABLE,
            format!("Registration failed: {:?}", e),
        )
    })?;

    info!(
        "Registration successful, received {} receivers",
        response.receivers.len()
    );

    // 3. Send encrypted data to receiver endpoints returned by the registry
    for receiver in response.receivers {
        info!(
            "Encrypting and sending envelope to receiver {} at {}",
            receiver.org_id, receiver.address
        );
        match encrypt_and_send(
            &receiver,
            &sender_client_id,
            &response.encryption_key,
            &receipt,
        )
        .await
        {
            Ok(_) => info!("Successfully sent to receiver: {}", receiver.address),
            Err(e) => {
                info!("Failed to send to receiver: {:?}", e)
            }
        }
    }

    Ok(http::StatusCode::ACCEPTED)
}
