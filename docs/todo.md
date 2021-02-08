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

## Tap controller expansion

Allowing the tap controller to access all commands could be interesting. Moving
the mouse around and clicking things at the tap of a button would be pretty
cool. I'm not sure exactly what it could be used for though. Doing this would
require reworking to configuration screen.

If this was to be implemented, then it would make sense to be able to save and
load multiple configurations. This would be a big overhaul of the configuration
page.

[This](https://stackoverflow.com/questions/2855857/how-to-display-multiple-columns-in-a-uitableview)
might be helpful.

## Running the server

You could open up the terminal and type in a command but that's not particularly
convenient. Investigate the alternates.
[This](https://apple.stackexchange.com/questions/376778/a-way-to-run-console-commands-in-menu-bar)
is a possibility.

## Go public

Push to a public github repo and publish to crates.io. Maybe make a post to
r/rust too.
