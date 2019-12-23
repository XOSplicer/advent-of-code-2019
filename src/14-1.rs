use anyhow::Result as AnyResult;
use daggy::petgraph::dot::{Config, Dot};
use daggy::petgraph::graph::NodeIndex;
use daggy::Dag;
use std::collections::HashMap;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::fmt;
use std::fs;
use std::io::Error as IoError;
use std::io::ErrorKind;
use std::iter;
use daggy::walker;

#[derive(Debug, Clone)]
struct Ingridient {
    name: String,
    amount: usize,
}

impl TryFrom<&str> for Ingridient {
    type Error = IoError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut ingridient_parts = s.trim().split(' ');
        let amount: usize = ingridient_parts
            .next()
            .ok_or(IoError::new(ErrorKind::InvalidData, "No amount".to_owned()))?
            .parse()
            .map_err(|_| IoError::new(ErrorKind::InvalidData, "Invalid amount".to_owned()))?;
        let name = ingridient_parts
            .next()
            .ok_or(IoError::new(ErrorKind::InvalidData, "No name".to_owned()))?;
        Ok(Ingridient {
            name: name.to_owned(),
            amount,
        })
    }
}

impl fmt::Display for Ingridient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", &self.amount, &self.name)
    }
}

#[derive(Debug, Clone)]
struct Formula {
    inputs: Vec<Ingridient>,
    output: Ingridient,
}

impl fmt::Display for Formula {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for input in self.inputs.iter() {
            fmt::Display::fmt(&input, f)?;
            f.write_str(", ")?;
        }
        f.write_str("=> ")?;
        fmt::Display::fmt(&self.output, f)
    }
}

impl TryFrom<&str> for Formula {
    type Error = IoError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut sides = s.split("=>");
        let inputs: Vec<Ingridient> = sides
            .next()
            .ok_or(IoError::new(ErrorKind::InvalidData, "No inputs".to_owned()))?
            .split(',')
            .map(Ingridient::try_from)
            .collect::<Result<_, _>>()?;
        let output: Ingridient = Ingridient::try_from(
            sides
                .next()
                .ok_or(IoError::new(ErrorKind::InvalidData, "No output".to_owned()))?,
        )?;
        Ok(Formula { inputs, output })
    }
}

#[derive(Clone, Debug)]
struct Formulas {
    inner: HashMap<String, Formula>,
}

impl TryFrom<&str> for Formulas {
    type Error = IoError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let inner = s
            .lines()
            .map(|s| Formula::try_from(s).map(|f| (f.output.name.clone(), f)))
            .collect::<Result<_, _>>()?;
        Ok(Formulas { inner })
    }
}

impl Formulas {
    fn element_set(&self) -> HashSet<String> {
        self.inner
            .values()
            .flat_map(|formula| {
                formula
                    .inputs
                    .iter()
                    .map(|i| i.name.clone())
                    .chain(iter::once(formula.output.name.clone()))
            })
            .collect()
    }
    fn formulas(&self) -> impl Iterator<Item = &Formula> {
        self.inner.values()
    }
}

#[derive(Clone, Debug)]
enum EdgeData {
    Input(usize),
    Output(usize),
}

#[derive(Clone, Debug)]
enum NodeKind {
    Formula,
    Element,
}

#[derive(Clone, Debug)]
struct NodeData {
    kind: NodeKind,
    name: String,
    needed: usize,
}

impl NodeData {
    fn formula(f: &Formula) -> Self {
        NodeData {
            kind: NodeKind::Formula,
            name: f.to_string(),
            needed: 0,
        }
    }
    fn element(name: String) -> Self {
        NodeData {
            kind: NodeKind::Element,
            name: name,
            needed: 0,
        }
    }
}

#[derive(Debug, Clone)]
struct Factory {
    dag: Dag<NodeData, EdgeData>,
    element_nodes: HashMap<String, NodeIndex>,
}

impl Factory {

    fn from_formulas(formulas: &Formulas) -> AnyResult<Self> {
        let elements = formulas.element_set();
        let mut dag: Dag<NodeData, EdgeData> = Dag::new();
        let mut element_nodes: HashMap<String, NodeIndex> = HashMap::new();
        for element in elements {
            element_nodes.insert(element.clone(), dag.add_node(NodeData::element(element)));
        }
        for formula in formulas.formulas() {
            let formula_node = dag.add_node(NodeData::formula(formula));
            dag.add_edge(
                formula_node,
                element_nodes.get(&formula.output.name).unwrap().clone(),
                EdgeData::Output(formula.output.amount),
            )?;
            for input in formula.inputs.iter() {
                dag.add_edge(
                    element_nodes.get(&input.name).unwrap().clone(),
                    formula_node,
                    EdgeData::Input(input.amount),
                )?;
            }
        }
        dag.node_weight_mut(element_nodes["FUEL"]).unwrap().needed = 1;
        Ok(Self { element_nodes, dag })
    }

    fn print_dot(&self) {
        println!("{:?}", Dot::with_config(&self.dag.graph(), &[]));
    }

    fn is_finished(&self) -> bool {
        self.dag
            .raw_nodes()
            .iter()
            .map(|node| &node.weight)
            .all(|node_data| match node_data.kind {
                NodeKind::Formula => node_data.needed == 0,
                NodeKind::Element => {
                    if node_data.name == "ORE" {
                        node_data.needed > 0
                    } else {
                        node_data.needed == 0
                    }
                }
            })
    }

    fn reduce_once(&mut self) {
        let mut node_id = self.element_nodes["FUEL"];
        while self.dag.node_weight(node_id).unwrap().needed == 0 {
            // node_id = self.dag.parents(node_id).walk_next(&self.dag).unwrap().1;
            unimplemented!();
        }

        // unimplemented!()
    }

    fn reduce(&mut self) -> usize {
        while !self.is_finished() {
            self.reduce_once();
        }
        self.ore_needed()
    }

    fn ore_needed(&self) -> usize {
        self.dag.node_weight(self.element_nodes["ORE"]).unwrap().needed
    }
}

fn main() -> AnyResult<()> {
    let file = fs::read_to_string("input/14-example-1")?;
    let formulas = Formulas::try_from(file.as_str())?;
    println!("{:#?}", &formulas);
    let mut factory = Factory::from_formulas(&formulas)?;
    println!("{:#?}", &factory);
    factory.print_dot();
    let answer = factory.reduce();
    println!("Ore needed: {}", answer);
    Ok(())
}
