1. Communication protocol

1.1 Transfers

All transfers are done between TCP streams, using the noise protocol framework
for handshaking, establishing a secure channel, and processing data sent and
received via TCP, using the "" protocol. Transfers are always 64 kB blocks.
BLAKE2 is used as the noise protocol hashing function.

1.2 Server password

The noise protocol is initiated using a pre-shared key. The key used in the
initialisation of the protocol on both client and server is generated from a
pre-shared passphrase on both client and server, using BLAKE3 as a key
derivation function (PBKDF).

This passphrase becomes the "server password", set interactively on server
start, the key generated from which is saved in plain text in the server
configuration. The server passphrase is stored in a system keyring on client
applications, and generated on initiating a connection.

1.2.1 Key derivation

A 256 bit key is generated from the server passphrase using BLAKE3. The PSK is
this length as a stipulation of the noise protocol framework.

2. User authentication

A user is added on the server explicitly before a user can authenticate. The
process of adding a new user will take the unencrypted passphrase, and hash it
using the Argon2 hash function. The result of adding a new user is that a
stored Argon2 hash in the server's configuration, corresponding to the user
name.

Users are authenticated by a user passphrase, sent over the encrypted
communication channel, but otherwise in plain text, with an LOGIN command,
containing also the user's username.

Upon receiving this message, the server will verify the correctness of the
password by comparison with the stored Argon2 hash for the supplied user name.

2.1 Rate limiting of authentication attempts

An exponential roll-off will be used upon authentication failures from a given
client IP address (and not port). Initial delay after failure will be low, ~10
seconds, but with >5 repeated failures will be high, >10 minutes.

In all cases, authentication attempts within 5 seconds of each other from the
same IP will be denied.

3. Storage protocol

The server stores messages it receives, and a client interacts with the server
by downloading parts of the store as it requires, and pushing messages to the
server, to interact with users and channels.

Messages are stored in a tree-based content addressed store.

3.1 Formatting of data

All data stored in the following types is stored in binary using the
`MessagePack` representation. As such, complex data can be represented
compactly in a binary representation, and serialised and deserialised easily to
and from the binary representation.

3.2 Message model

- Messages.
- Channels (or queries) contain messages.
- Namespaces contain channels.
- A single `roffl` instance may contain many namespaces.

Messages are stored in blocks, see below, but other types have no such
restriction. [This may need to be revised to allow forward compatibility with
large servers.]

3.2.1 Messages

A message content record consists of the following fields:
- Sender nick
- Message content

A message contains the following fields:
- Timestamp (UTC Unix time of server)
- Message content hash
- Message content

Messages are stored in blocks, of up to 64 KB in size. A single block can store
any number of messages up to the upper size limit.

The fields the block contains are:
- Child block (hash)
- Array of message blocks

3.2.2 Channels

The channel contains the following fields:
- Head (hash)
- Flags

Head contains the hash of the most recent message block pertaining to it.

3.2.3 Namespaces

The namespace type contains the following fields:
- Name
- Type (IRC, XMPP, Slack, Tox, Roffl)

4. Commands

This section defines the possible network transactions between Roffl servers,
and between Roffl server and clients. They define the transactions

4.1 Send tree snapshot

When a server receives new messages, it pushes a SendTreeSnapshot command to
all connected clients and other servers. These clients wait for this command to
be received, and upon receiving it, verify the local tree with their own.

If differences are found in the tree, the client may request blocks which have
changed. These blocks are then merged according to the merge rules (see below),
and UpdateBlock commands are issued accordingly.

4.2 Request tree snapshot

4.2 Request block

If the block length exceeds 64 KB, the tree is modified, and the block tree is
updated, to finalise an old block, and append a new block to the end. This
requires multiple transfers, one per block.

4.3 Update block

4.4 Update tree

If a client requests changes to be made to a tree on a server (e.g. deleting a
channel), it may send an UpdateTree command.

4.5 Request tree

5.0 Encryption

Secret channels and queries are encrypted end-to-end. This is a symmetric
encryption using the
[chacha](https://en.wikipedia.org/wiki/Salsa20#ChaCha_variant) stream cypher.
