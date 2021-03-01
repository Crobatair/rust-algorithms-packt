# Information related to Graphs.


This module contains information related to how to build a node graph on Rust.

The follow image taken from Course and it used on all chapter.

![graph](https://github.com/crobatair/rust-algorithms-packt/blob/main/p4_connected_nodes/graph.png?raw=true)


---
In [v1_graphs_connected_nodes](../../main/p4_connected_nodes/v1_graphs_connected_nodes/src/main.rs), contains different method to implement a Graph using different techniques in Rust.

---

On [v2_graphs](../../main/p4_connected_nodes/v2_graphs/src/main.rs) its implemented a graph on pointer based.
Implemented a basic add node, and add edge methods.

---

On [v3_router_structure](../../main/p4_connected_nodes/v3_router_structure/src/main.rs) moved err to a module and added a test to test if new Err class, instance, contains a mess with asserteq on message.
Also,  created a trait for i32 and impl trait.

----

On [v4_shortest_path](../../main/p4_connected_nodes/v4_shortest_path/src/main.rs) is implemented the method to find the shortest path from __A__ to a point __B__.
> If run cargo run on this folder, will return the shortest path on __from: A, to: D__.
The short path to **A** -> **D** is: **'A'-7-'H'-13-'D'**

![shortest_path_a_to_d](https://github.com/crobatair/rust-algorithms-packt/blob/main/p4_connected_nodes/shortest_path_a_to_d.png?raw=true)

----

On [v5_greed_traveling_salesman](../../main/p4_connected_nodes/v5_greed_traveling_salesman/src/main.rs) is implemented a primitive way to solve the salesman traveling problem. But on this aproach, each node visit several time the principal node **A**.


----
On [v6_improve_salesman_with_iterators](../../main/p4_connected_nodes/v6_improve_salesman_with_iterators/src/main.rs).
Its implemented a random swap, to test different combinations of short path to node.

If the algorithm does not improve len between iterations ( 50 time on each improvement ), it returns the best traveling salesman.