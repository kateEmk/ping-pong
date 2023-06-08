# Launch server
`python3 http3_server.py --certificate ssl/ssl_cert.pem --private-key ssl/ssl_key.pem`
# Run tests
`pytest tests.py`
# Autodiscovery
The autodiscovery process for server registration and connection establishment using WebTransport involves the following steps:
1. **Server Registration**. Servers register themselves with a centralized or decentralized system, providing metadata about their capabilities, location, or services.
2. **Discovery Mechanism**. Clients request server information from a centralized authority, use decentralized protocols like multicast or peer-to-peer discovery, or query the DNS for server addresses.
3. **Client Discovery**. Clients initiate the autodiscovery process by querying the discovery mechanism to obtain a list of available server endpoints or relevant information.
4. **Connection Establishment**. Clients use the server information to establish connections with desired servers using the WebTransport protocol, negotiating connection parameters such as security settings and transport characteristics.
It is important to acknowledge that the implementation specifics and precise steps may vary based on the chosen autodiscovery mechanism and the specific WebTransport library or framework utilized.