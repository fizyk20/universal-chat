# universal-chat

This crate aims to present a unified API for multiple chat services. This way, a bot or client will
be able to connect to multiple different chats in a transparent manner.

Chat services are represented by so-called "sources", which are event sources. Among the events are
things like "connected", "disconnected", "message received" etc. Every event contains a source ID,
so the source service is easily identifiable, but the whole interface is service-agnostic, so the
consumer doesn't have to care whether they are talking via Slack, Discord or something else.

## Supported services

The current set of supported services is as follows:

* IRC (partial)
* Slack (partial)
* Discord (partial)

**Note**: I'm creating this crate for my own use, and I don't need all the features, so my goal
is not to reach full support for all protocols. However, I'll gladly accept pull requests extending
the feature set if there is interest in wider support.
