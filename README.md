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

## Other notes

### No binary

The current implementation of lan_clipboard does not support binary clipboard contents. See
https://github.com/jkcclemens/lan_clipboard/issues/7.

### Internet

I suppose, technically, lan_clipboard isn't really bound to your LAN. If you don't start the server
on your LAN but instead bind to a publicly-accessible address, computers over the internet could
share clipboards. I did not design lan_clipboard to work this way.

**However**, lan_clipboard *does* use TLS for encryption. I'm no security expert, so I can't
guarantee I did it right. If you get a real cert from a real CA (versus the self-signed ones I use
for LAN), I suppose maybe that's feasible.

You're potentially sending your passwords and other sensitive information over the internet through
a tube some guy on the internet said was "probably safe, maybe."
