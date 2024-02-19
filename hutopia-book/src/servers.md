# Servers

The following section covers different servers
behind a hutopia ecosystem.

## Relay server(s)

The `relay` server is the centralized server which handles:
- user authentication;
- website service;
- profile storage.

The relay needs an external database server to
store its data (e.g. user profiles, spaces you
are part of, authentication tokens and such).

## Space server

A `space` server is a server which users
can host to create their own custom space.
The space itself is a custom website
which is rendered on the client.
Users can join multiple space servers.

The space needs an external database server
to store its members and other data.