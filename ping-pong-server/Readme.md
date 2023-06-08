# Launch server
`python3 http3_server.py --certificate ssl/ssl_cert.pem --private-key ssl/ssl_key.pem`
# Run tests
`pytest tests.py`
# Autodiscovery
The following stages are part of the autodiscovery procedure for registering servers and establishing connections when utilizing WebTransport:
1. Registration of the server. With the help of metadata describing their capabilities, location, or services, servers register themselves with a centralized or decentralized system.
2. Mechanism for Discovery. Clients can use decentralized protocols like multicast or peer-to-peer discovery, ask the DNS for server addresses, or request server information from a centralized authority.
3. Client Research. The autodiscovery process is started by clients asking the discovery mechanism to get a list of available server endpoints or pertinent data.
4. Establishing of the Connection. In order to connect to the desired servers using the WebTransport protocol, clients use the server information to negotiate connection details including security options and transport characteristics.
It is crucial to recognize that the implementation details and exact procedures may change depending on the autodiscovery mechanism selected and the particular WebTransport library or framework used.
