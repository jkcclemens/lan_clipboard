# lan_clipboard

*Share your clipboard across your LAN... or the internet.*

lan_clipboard was designed to easily share your clipboard between multiple computers. It operates
using a server/client framework, where one computer (the server, which can also be a client) receives
updates from other computers (the clients) and tells the clients when to update their clipboards.

lan_clipboard is (primarily) platform agnostic. So long as your platform can connect to the network
and has a clipboard (that's supported), it will work. The current platforms supported are Windows,
Linux via X11, and macOS.

The server machine does not need a clipboard and only needs to be able to access the network to
function. It will need enough space for TLS certificates/keys on the disk and enough memory to store
the clipboard and information about clients.

## [To do](https://github.com/jkcclemens/lan_clipboard/projects/1)

## Compilation

For both the server and client, clone the repo, move into the necessary directory, and `cargo build --release` that sucker.

The binary will be `target/release`. Run it to see usage instructions, or see below.

To successfully compile, you need a few dependencies!

**Server/client**
- [protocol buffers](https://developers.google.com/protocol-buffers/), including the `protoc` binary

**Client**
- X11 (xorg-dev) on Linux only

## Usage

The client and server have usage instructions if you run them and specify the `-h` or `--help` flag.
While not necessary, specifying a configuration file will make it easiest to start them. Most
options can be specified on the command-line, but not all.

### Server

You need one of these!

An example configuration file is located
[here](https://github.com/jkcclemens/lan_clipboard/blob/master/configs/server.toml).

Start the server, then connect clients. When clients change their clipboard, the server will tell
the other clients to update automatically.

The server uses TLS to secure transmission of your data. You will need to provide a x509 v3
certificate chain and its private key, both in PEM format, to the server.
[webpki](https://github.com/briansmith/webpki) is picky.

### Client

You don't necessarily need any of these, but you need two to get any use out of this program!

An example configuration file is located
[here](https://github.com/jkcclemens/lan_clipboard/blob/master/configs/client.toml).

Start the client and connect it to a server. It is recommended to daemonize the client if possible.
Any time your clipboard changes, the client will automatically tell the server.

The client will connect using TLS to the server, so you'll need to provide the x509 v3 certificate
chain that the server is using in order to communicate.

Once your clients are up and running, anything copied on one machine will be propagated to every
other machine connected to the server (but not the server). If you want it to also update the
server's clipboard, have the server connect to itself as a client.

## Certificates? :(

Hey, I want your (and also my) data to be safe! webpki demands it be really safe! Your certificate
has to be x509 v3, meaning it must be issued from a CA, even if that CA is you. It must also pass
SNI checks.

This means that if you're connecting to a machine locally, you'll need to issue a cert to
`[hostname].local` and connect using that address.

## Other notes

### No binary

The current implementation of lan_clipboard does not support binary clipboard contents. See
[this issue](https://github.com/jkcclemens/lan_clipboard/issues/7).

### Internet

I suppose, technically, lan_clipboard isn't really bound to your LAN. If you don't start the server
on your LAN but instead bind to a publicly-accessible address, computers over the internet could
share clipboards. I did not design lan_clipboard to work this way.

**However**, lan_clipboard *does* use TLS for encryption. I'm no security expert, so I can't
guarantee I did it right. If you get a real cert from a real CA (versus the self-signed ones I use
for LAN), I suppose maybe that's feasible.

You're potentially sending your passwords and other sensitive information over the internet through
a tube some guy on the internet said was "probably safe, maybe."

### No OpenSSL

lan_clipboard uses [rustls](https://github.com/ctz/rustls) for TLS funsies, not OpenSSL. Just so you
know.

### Bandwidth

The client sends pings to the server every 15 seconds, and the server will respond to each ping.

The client will send the contents of the clipboard to the server whenever it detects a change,
optionally compressing it. In this implementation of the client, clipboards less than 17 bytes are
not compressed.

The server will send every client (including the updating client) the new clipboard, optionally
compressing it. In this implementation of the server, clipboards less than 17 bytes are not
compressed.

The stream of packets between server and client is not compressed in either direction. Most packets
are actually too small to be compressed, and compression increases their size.
