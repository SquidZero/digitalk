# digitalk
An online chatting app written in rust. rename probabally required in the future.

# Structure
crate ``digitalk`` is split into three subcrates:

## <ins>`libdigitalk`</ins>

The library containing basic definitions required by both `client` and `server`.

## <ins>`client`</ins>

Client for digitalk, used to communicate with server. (not started yet)

## <ins>`server`</ins>

Server for digitalk. The server recieves messages from each client and tells other clients what those messages are.
