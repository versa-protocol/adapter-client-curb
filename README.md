# adapter-client-curb

An adapter client designed to consume Curb rides and forward them to the Versa protocol

## Prerequisites

Before you can send receipts across the network, you'll need to register with the protocol.

1. Sign up for a developer account at https://app.versa.org
2. Configure your profile and learn about our receipt schema in the [Studio](https://app.versa.org/studio) or [Docs](https://docs.versa.org)
3. Issue client credentials for the sandbox environment
4. When ready, email support@versa.org and we'll verify your account and gate you into the production environment

## Setup and Deployment

Since this adapter does not have an authentication step, we recommend deploying the image in your cloud with internal access limits. You can get the latest image from Docker Hub, here: https://hub.docker.com/r/versaprotocol/adapter-client-curb

Set your Versa Client ID, Client Secret, and the Registry URL as environment variables for the deployed container. The Registry URL should always be https://registry.versa.org unless you are using a custom registry for testing purposes. See [Environment](#Environment) for more details.

## Usage

The deployed container listens for POST requests at the root URL to which it is deployed (on port 8000).

The request body should contain a JSON object structured as follows, where the ride is a full `Ride` object as specified in the [Curb API Docs](https://bookwithcurb.docs.apiary.io/#reference/endpoints/rides/retrieve-a-ride), complete with bill: 

```sh
{
    "ride": { 
        "id": 36,
        ...
    },
    "customer_email": "mike@bloomberg.com"
}
```

This payload should be sent once the bill is paid by the customer. The email is required for us to route Versa receipts to customers by domain.

## Environment

You can view an example "env" file in the root of this repository. In production, you'll set each of these variables on the deployed Docker container.

You can issue client credentials at https://app.versa.org

```bash
 # Your Versa client ID 
CLIENT_ID=versa_cid_test_xxxxxxxxxxxxxx

# Your Versa client secret, which authenticates requests to the registry — note this should never be sent to a receiver!
CLIENT_SECRET=versa_csk_test_xxxxxxxxxx 

# The URL of the Versa registry, where the client will register data hashes and decryption keys
REGISTRY_URL=https://registry.versa.org
```

## Testing the Docker Image Locally

```sh
docker run --env  CLIENT_ID=versa_cid_test_xxxxx --env CLIENT_SECRET=versa_csk_test_xxxxx --env REGISTRY_URL=https://registry.versa.org [DOCKER_IMAGE]

```