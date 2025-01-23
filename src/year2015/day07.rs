use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Operation {
    Const,
    And,
    Or,
    Not,
    LShift,
    RShift,
}

#[derive(Debug)]
pub struct Gate {
    op: Operation,
    inp_a: Option<String>,
    inp_b: Option<String>,
    name: String,
}

impl Gate {
    fn calculate_value(&self, lhs: Option<u16>, rhs: Option<u16>) -> Option<u16> {
        match self.op {
            Operation::Const => if let Some(a) = lhs {
                Some(a)
            } else {
                None
            },
            Operation::And => if let (Some(a), Some(b)) = (lhs, rhs) {
                Some(a & b)
            } else {
                None
            },
            Operation::Or => if let (Some(a), Some(b)) = (lhs, rhs) {
                Some(a | b)
            } else {
                None
            },
            Operation::Not => if let Some(a) = lhs {
                Some(!a)
            } else {
                None
            },
            Operation::RShift => if let (Some(a), Some(b)) = (lhs, rhs) {
                Some(a >> b)
            } else {
                None
            },
            Operation::LShift => if let (Some(a), Some(b)) = (lhs, rhs) {
                Some(a << b)
            } else {
                None
            },
        }
    }
}

pub fn parse(input: &str) -> Vec<Gate> {
    input.split('\n').filter(|s| !s.is_empty()).map(|g| match g {
        _x if _x.contains("AND") => {
            let fields: Vec<_> = g.split(' ').collect();
            Gate {
                op: Operation::And,
                inp_a: Some(fields[0].to_string()),
                inp_b: Some(fields[2].to_string()),
                name: fields[4].to_string(),
            }
        },
        _x if _x.contains("OR") => {
            let fields: Vec<_> = g.split(' ').collect();
            Gate {
                op: Operation::Or,
                inp_a: Some(fields[0].to_string()),
                inp_b: Some(fields[2].to_string()),
                name: fields[4].to_string(),
            }
        },
        _x if _x.contains("NOT") => {
            let fields: Vec<_> = g.split(' ').collect();
            Gate {
                op: Operation::Not,
                inp_a: Some(fields[1].to_string()),
                inp_b: None,
                name: fields[3].to_string(),
            }
        },
        _x if _x.contains("RSHIFT") => {
            let fields: Vec<_> = g.split(' ').collect();
            Gate {
                op: Operation::RShift,
                inp_a: Some(fields[0].to_string()),
                inp_b: Some(fields[2].to_string()),
                name: fields[4].to_string(),
            }
        },
        _x if _x.contains("LSHIFT") => {
            let fields: Vec<_> = g.split(' ').collect();
            Gate {
                op: Operation::LShift,
                inp_a: Some(fields[0].to_string()),
                inp_b: Some(fields[2].to_string()),
                name: fields[4].to_string(),
            }
        },
        _x => {
            let fields: Vec<_> = g.split(' ').collect();
            Gate {
                op: Operation::Const,
                inp_a: Some(fields[0].to_string()),
                inp_b: None,
                name: fields[2].to_string(),
            }
        },
    }).collect()
}

pub fn part1(input: &[Gate]) -> u16 {
    let mut lut = HashMap::<String, u16>::new();

    let mut fin: bool = false;
    while !fin {
        fin = true;
        for g in input.iter() {
            if !lut.contains_key(&g.name) {
                fin = false;
                let lhs = match &g.inp_a {
                    Some(wire) => match wire.parse::<u16>() {
                        Ok(v) => Some(v),
                        Err(_) => lut.get(wire).copied(),
                    },
                    None => panic!("All wires should have at least one input!"),
                };

                let rhs = match &g.inp_b {
                    Some(wire) => match wire.parse::<u16>() {
                        Ok(v) => Some(v),
                        Err(_) => lut.get(wire).copied(),
                    },
                    None => None,
                };

                if let Some(v) = g.calculate_value(lhs, rhs) {
                    lut.insert(g.name.clone(), v);
                }
            }
        }
    }

    *lut.get("a").unwrap()
}

pub fn part2(input: &[Gate]) -> u16 {
    let prev_sol = part1(input);
    
    let mut lut = HashMap::<String, u16>::new();
    lut.insert("b".to_string(), prev_sol);

    let mut fin: bool = false;
    while !fin {
        fin = true;
        for g in input.iter() {
            if !lut.contains_key(&g.name) {
                fin = false;
                let lhs = match &g.inp_a {
                    Some(wire) => match wire.parse::<u16>() {
                        Ok(v) => Some(v),
                        Err(_) => lut.get(wire).copied(),
                    },
                    None => panic!("All wires should have at least one input!"),
                };

                let rhs = match &g.inp_b {
                    Some(wire) => match wire.parse::<u16>() {
                        Ok(v) => Some(v),
                        Err(_) => lut.get(wire).copied(),
                    },
                    None => None,
                };

                if let Some(v) = g.calculate_value(lhs, rhs) {
                    lut.insert(g.name.clone(), v);
                }
            }
        }
    }

    *lut.get("a").unwrap()
}
