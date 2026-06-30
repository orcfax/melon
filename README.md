# Melon

[Why melon?][wiki-melon]

[wiki-melon]: https://en.wikipedia.org/wiki/Melon_(cetacean)

## Demo

### Generate

Generate a set of testnet configs.

```sh
cargo run -p melon-net -- generate
```

This generates addresses and keys allowing nodes to find and identify one
another. Inspect the configs in `./configs`.

### Run

#### Terminal 0

Launch the fx-server:

```sh
cargo run -p melon-fx --bin melon-fx
```

This serves fx data over HTTP.

#### Terminal 1

Launch rendezvous node, node-00

```
cargo run -p melon-app -- run --index 0
```

This node awaits connections from peers. It also polls the fx-server and caches
latest prices.

#### Terminal 2

Launch peer `node-01`

```
cargo run -p melon-app -- run --index 1
```

This node finds the rendezvous, and through them connects to other peers. We use
libp2p's internals to handle peer management.

Optionally launch another node in a new terminal.

#### Terminal 3

Send message to node-01

```sh
cargo run -p melon-ctl -- --socket /tmp/node-01.sock app ""
```

This should trigger node-01 to propose a new collection of statements, based on
its latest cache of price feed data. Node-00 receives the proposals and responds
with endorsements. The default config requires a quorum of 2 from 3, thus the
statements are identified as `Agreed`.

## Todo

- [ ] docs - need updating
- [ ] seq - sequencer needs updating
- [ ] exe - executor needs implementing
- [ ] blob - store needs implementing
