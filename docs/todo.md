## TCP

It might make sense to use TCP directly rather than using web sockets. This
would involve dropping a lot of working code. It may improve performance
slightly.

[Using `NWFramework`](https://rderik.com/blog/building-a-server-client-aplication-using-apple-s-network-framework/)

## Security

Currently, the only form of security is only allowing one client to connect to
the server. We could have the client and server maintain random number
generators and send a random number with each message. The message would have to
match the number from the server for the server to accept the message.

This would involve establishing a shared secret. The server would need to
generate the seed and let the user transfer this to their phone. Perhaps using
a QR code.

Something like this could also be used for encryption. The sequence of random
numbers could be used as the key for an XOR stream cipher.
