# 00_get_node_info

Get node information from an IOTA node.

```bash
cargo run --example 00_get_node_info
```

you should see some informations about the connected IOTA node, like this:

**Example Output**
```bash
NodeInfo {
    name: "HORNET",
    version: "0.6.0-alpha",
    is_healthy: true,
    network_id: "alphanet1",
    latest_milestone_index: 86951,
    solid_milestone_index: 86951,
    pruning_index: 27200,
    features: [
        "PoW",
    ],
}
```