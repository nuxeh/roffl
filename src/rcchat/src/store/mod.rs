// Persistent data store for client and server
//
// Storage is mainly used for storing the message tree, and configuration (?).
//
// Pluggable for allowing different providers for the backing key-value store,
// currently supporting:
//
// - sled <https://github.com/spacejam/sled>
