use anyhow::Result as AnyResult;
use daggy::petgraph::graph::DefaultIx;
use daggy::petgraph::visit::Walker;
use daggy::Dag;
use daggy::NodeIndex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::fs;
use thiserror::Error;

#[derive(Debug, Error)]
enum MyError {
    #[error("invalid orbit syntax `{0}`")]
    InvalidOrbitSyntax(String),
}

#[derive(Debug)]
struct Orbit<'a> {
    central: &'a str,
    trabant: &'a str,
}

impl<'a> TryFrom<&'a str> for Orbit<'a> {
    type Error = MyError;
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let mut parts = s.split(')');
        let central = parts
            .next()
            .ok_or_else(|| MyError::InvalidOrbitSyntax(s.to_owned()))?;
        let trabant = parts
            .next()
            .ok_or_else(|| MyError::InvalidOrbitSyntax(s.to_owned()))?;
        Ok(Orbit { central, trabant })
    }
}

#[derive(Debug)]
struct Weight;

#[derive(Debug)]
struct TransitiveParentsIter<'a> {
    dag: &'a Dag<Weight, Weight>,
    index_stack: Vec<NodeIndex<DefaultIx>>,
}

impl<'a> TransitiveParentsIter<'a> {
    fn new(dag: &'a Dag<Weight, Weight>, start_index: NodeIndex<DefaultIx>) -> Self {
        Self {
            dag,
            index_stack: dag
                .parents(start_index)
                .iter(&dag)
                .map(|(_, n)| n)
                .collect(),
        }
    }
}

impl<'a> Iterator for TransitiveParentsIter<'a> {
    type Item = NodeIndex<DefaultIx>;
    fn next(&mut self) -> Option<Self::Item> {
        self.index_stack.pop().map(|idx| {
            self.index_stack
                .extend(self.dag.parents(idx).iter(&self.dag).map(|(_, n)| n));
            idx
        })
    }
}

fn main() -> AnyResult<()> {
    let input = fs::read_to_string("input/06")?;
    let orbits: Vec<Orbit> = input
        .lines()
        .map(Orbit::try_from)
        .collect::<Result<_, _>>()?;
    println!("Num of orbits: {}", orbits.len());
    let centrals = orbits.iter().map(|o| o.central);
    let trabants = orbits.iter().map(|o| o.trabant);
    let nodes: HashSet<&str> = centrals.chain(trabants).collect();
    println!("Num of Nodes: {}", nodes.len());
    let mut dag = Dag::<Weight, Weight>::with_capacity(nodes.len(), orbits.len());
    let inserted_nodes: HashMap<&str, _> =
        nodes.iter().map(|&n| (n, dag.add_node(Weight))).collect();
    // println!("inserted nodes: {:#?}", &inserted_nodes);
    for orbit in orbits {
        dag.add_edge(
            *inserted_nodes.get(orbit.central).unwrap(),
            *inserted_nodes.get(orbit.trabant).unwrap(),
            Weight,
        )?;
    }
    println!("Num of dag nodes: {}", dag.node_count());
    println!("Num of dag edges: {}", dag.edge_count());
    // println!("Dag: {:#?}", &dag);
    let answer: usize = inserted_nodes
        .values()
        .map(|idx| TransitiveParentsIter::new(&dag, *idx).count())
        .sum();
    println!("{}", answer);
    Ok(())
}
