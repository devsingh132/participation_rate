# Beacon Chain Indexer
## Setup
    Replace the ETH_BASE_URL in the .env with the http url of the ethereum testnet/mainnet endpoint.
## Running
    docker-compose up -d
## Endpoints
- Accepts 2 get requests at 8080 port.
    -   http://127.0.0.1:8080/get 
        -  Calculates the overall participation rate of the network.
        -  Response:
            -  Statuscode : 200
                -   Returns a number denoting the participation rate of the network.
            -   StatusCode : 500
                -  Internal error
    -  http://127.0.0.1:8080/get/{pubkey}
        - Gives the participation rate of the given validator.
        - Responses:
            -  Statuscode : 200
                -   Returns a number denoting the participation rate of the validator over the network.
            -   StatusCode : 404
                -  Unable to find the given validator.
            -   StatusCode : 500
                -  Internal error

## Features
-   Indexes the slots for the 5 most recent epochs.
-   Indexes the upcoming finalized epochs.
-   Exposes the port 8080 which accepts the API get requests.

## Working
-   Launches 2 docker containers
    -   db_pp :
        -   Container for mysql. exposes 3306 port can be connected using host:127.0.0.1, port:3306.
        -   Creates a database pp_rate and then a table attestations in it.
    -   participation_rate:
        -  Container where the application is running.
        -  Exposes 8080 port for the API connection.
-  Creates a separate thread for accepting API requests for calculating participation rate.
-  Indexes the upcoming finalized epochs:
    -  A separate thread is used to initiate the indexing process for upcoming epochs.
    -   The indexing frequency is set to every half epoch to ensure that new finalized blocks are not missed, considering the time required to index the current epoch.
-  Start Indexing the 5 recent epochs.
    - Creates a table named attestations in the pp_rate database.
    - The application fetches the most recent finalized block from the API.
        - For each indexed slot:
            - Fetches the block attestation data from the API.
            - Retrieves the committee set for the block.
            - Extracts the validator array based on the committee index in the attestation data.
            - Checks the bit of the aggregation_bits to determine if the validator index has attested.
            - Fetches validator data to map the public key with the validator index.
            - Stores the (epoch, slot, pubkey, attested) data in the database.
- DB used for this purpose was MYSQL, as we have a structured data.

## Structs 
- clients
    - apiclient
        - Responsible for interating with the ethereum beacon chain api's, fetching, indexing, sending data to the DB.
    - apiserver
        - Responsible for creating the api server which exposes the above get requests on 8080 port.
- models
    - committee_response_data
        - Maps the response form the api to a struct.
    - committee_set
        - To store the list of validators based on committee index.
    - validator_response
        - Maps the response from the api to a struct.
    - validator_data
        - Struct to store the index and check if it attested or not.
    - block_data_model
        - Stores the block data from the api.

## Troubleshooting
- Check the docker logs for participation_rate container if connection to mysql is getting timeout then increase the DELAY variable in docker-compose.yml happens due to mysql takes time to run.
- Make sure the mentioned ports are free 3306, 8080.
    - Make sure the provided url in the .env is correct.