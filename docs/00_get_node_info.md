# 00_get_node_info

Get node information from an IOTA node.

```bash
cargo run --example 00_get_node_info
```

you should see some informations about the connected IOTA node, like this:

```bash
GetNodeInfoResponse {
    app_name: "HORNET",
    app_version: "0.4.0-rc12",
    jre_available_processors: None,
    jre_free_memory: None,
    jre_max_memory: None,
    jre_total_memory: None,
    jre_version: None,
    latest_milestone: "EC9XOVXNBHUHKVPTJGYMKZVEGDH9GYWPFDCBKVTDWIGAOKLISDDMTFOPXFEOJRMZVZCFYPSYUUBOA9999",
    latest_milestone_index: 1428959,
    latest_solid_subtangle_milestone: "EC9XOVXNBHUHKVPTJGYMKZVEGDH9GYWPFDCBKVTDWIGAOKLISDDMTFOPXFEOJRMZVZCFYPSYUUBOA9999",
    latest_solid_subtangle_milestone_index: 1428959,
    milestone_start_index: 1427599,
    neighbors: 4,
    packets_queue_size: None,
    time: 1590882172000,
    tips: 0,
    transactions_to_request: 0,
}
```