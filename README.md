
# BEL

A toy project to learn about manipulation of binary files and co-occurrence.

It is supposed to mimic a graph database, but it is hyper-simplified/specialized and can only store a single type of node with a single type of relationship, the "edge list", hence the name BEL (Binary Edge List).

It can only encode co-occurrence of nodes, the primary example would be co-occurrence of products in a shopping cart. If you store the co-occurrence "factor" of all the products bought together (in the same shopping cart) for all the purchases, you can later query the database to answer the question, "which are the top n products that are usually bought together with product X?".

---

### Usage

- run the server binary with `cargo run --bin server`
- run the client binary with `cargo run --bin client`
- write your query manually on the client stdin and press Enter to send
- the server will respond with the result of the query or an error message

### Queries
there exists only two queries that are very similar to linux commands:
- `put <list-of-tuples>`: update the edge lists based on the provided tuples, the list must be in the format `(<node_id:u32>:<group_id:u32>)[,(<node_id:u32>:<group_id:u32>)...]` (e.g `put (179001:1),(18045:1),(288389:1),(179001:2),(288398:2),(1200007:2)`)
- `get top <k:u32> from <node_id:u32>`: get top k nodes related to the provided node id (e.g `get top 5 from 179001`)
