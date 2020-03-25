
use pines::graph;

fn fib(n: i64, graph: &mut graph::GraphBuilder,  root: usize) -> i64 {
    let root = graph.add_child(&format!("Value {}", n), root);
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fib(n - 1, graph, root) + fib(n - 2, graph, root)
    }
}

fn main() {
    let mut builder = graph::GraphBuilder::new("Fibonacci", graph::GraphKind::Direct, graph::NodeShape::Diamond);
    let n = 8;
    let root = builder.add_node(&format!("{}-th Fibonacci Number", n));
    let _fib = fib(n, &mut builder, root);
    println!("{}", builder.export());
}
