use petgraph::graph::{NodeIndex,UnGraph,Graph};
use petgraph::Undirected;
use petgraph::Direction;


#[derive(Debug)]
struct Fighter {
    name: String,
}

impl Fighter {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string()
        }
    }
}

pub fn add_edge(graph: &mut Graph<&Fighter, f32, Undirected>, nodes: &Vec<NodeIndex>, a: usize, b: usize)
{
    graph.add_edge(nodes[a], nodes[b], 1.0);
}

pub fn graph_ufc_centrality() {
    let dustin = 0usize;
    let khabib = 1usize;
    let mcgregor = 2usize;
    let diaz = 3usize;
    let aldo = 4usize;
    let mut fighters = vec![
        Fighter::new("Dustin Poirier"),
        Fighter::new("Khabib"),
        Fighter::new("McGregor"),
        Fighter::new("Diaz"),
        Fighter::new("Jose Aldo"),
    ];

    let mut graph: Graph<&Fighter, f32, Undirected> = UnGraph::new_undirected();

    let mut nodes: Vec<NodeIndex> = fighters.iter()
    .map(|fighter| graph.add_node(fighter)).collect();

    add_edge(&mut graph, &nodes, dustin, khabib);
    add_edge(&mut graph, &nodes, khabib, mcgregor);
    add_edge(&mut graph, &nodes, mcgregor, dustin);
    add_edge(&mut graph, &nodes, mcgregor, aldo);
    add_edge(&mut graph, &nodes, mcgregor, diaz);
    add_edge(&mut graph, &nodes, dustin, diaz);
    add_edge(&mut graph, &nodes, aldo, diaz);
    
    // println!("{:?}", fighters);
    // println!("{:?}", nodes);

    for (i, &node) in nodes.iter().enumerate() {
        let name = &fighters[i].name;
        let degree = graph.edges_directed(node, Direction::Outgoing).count() as f32;
        let closeness = 1.0 / degree;
        println!("The closeness centrality of {name} is {closeness:.2}");
    }
}