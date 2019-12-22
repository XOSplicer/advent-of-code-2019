use anyhow::Result as AnyResult;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt;
use std::fs;
use std::io::Error as IoError;
use std::io::ErrorKind;
use std::iter;

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

impl<'a> TryFrom<&str> for Formula {
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

#[derive(Debug, Clone)]
struct Factory {
    stack: HashMap<String, usize>,
}

impl Factory {
    fn new<'a, I: IntoIterator<Item = &'a str>>(iter: I) -> Self {
        let mut stack: HashMap<String, usize> =
            iter.into_iter().map(|s| (s.to_owned(), 0)).collect();
        stack.insert("ORE".to_owned(), 0);
        stack.insert("FUEL".to_owned(), 1);
        Factory { stack }
    }
    fn find_next_formula<'b>(&self, formulas: &'b HashMap<String, Formula>) -> Option<&'b Formula> {
        // TODO: Reducing any  valid formula will not lead to an optimal solution
        self.stack
            .iter()
            .filter(|(_, &amount)| amount > 0)
            .filter_map(|(name, _)| formulas.get(name))
            .next()
    }
    fn unapply_formula(&mut self, formula: &Formula) {
        if self.stack.get(&formula.output.name).unwrap_or(&0) == &0 {
            panic!(
                "Cannot unapply formula where output should not be produced: {:?}",
                &formula
            );
        }
        let new_output = self
            .stack
            .get(&formula.output.name)
            .unwrap()
            .saturating_sub(formula.output.amount);
        *self.stack.get_mut(&formula.output.name).unwrap() = new_output;
        for input in formula.inputs.iter() {
            *self.stack.entry(input.name.clone()).or_insert(0) += input.amount;
        }
    }
    fn is_finished(&self) -> bool {
        self.stack
            .iter()
            .filter(|(name, _)| name != &"ORE")
            .all(|(_, &value)| value == 0)
    }
    fn ore_needed(&self) -> usize {
        *self.stack.get("ORE").unwrap()
    }
}

fn main() -> AnyResult<()> {
    let file = fs::read_to_string("input/14-example-1")?;
    let formulas: HashMap<String, Formula> = file
        .lines()
        .map(|s| Formula::try_from(s).map(|f| (f.output.name.clone(), f)))
        .collect::<Result<_, _>>()?;
    println!("{:#?}", &formulas);
    let mut factory = Factory::new(
        formulas
            .values()
            .flat_map(|f| f.inputs.iter().chain(iter::once(&f.output)))
            .map(|i| i.name.as_str()),
    );
    println!("{:#?}", &factory);
    while !factory.is_finished() {
        let formula = factory
            .find_next_formula(&formulas)
            .expect("Found no formula to unapply");
        println!("Unapplying {}", &formula);
        factory.unapply_formula(&formula);
        println!("New factory {:?}", &factory);
    }
    println!("{}", factory.ore_needed());
    Ok(())
}
