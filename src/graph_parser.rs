use pest::{error::Error, iterators::Pairs, Parser};


use crate::{AdjList, Vertex};

/*#[derive(Parser)]
#[grammar = "graph.pest"]
pub struct GraphParser;

#[test]
fn hmm(){
    let ret = parse("a -> b -> c\n a -> d");
    dbg!(ret);

}

pub fn parse(prog: &str) -> Result<AdjList, Error<Rule>> {
    let mut ret: _ = GraphParser::parse(Rule::GeneralGraph, &prog)?.into_iter();
    let mut list = AdjList::default();
    let general_graph = ret.next().unwrap();
    match general_graph.as_rule() {
        Rule::Graph => {
            for snip in general_graph.into_inner().into_iter() {
                let mut di_graph_elem = snip.into_inner();
                let mut last_name = None;
                while let Some(elem) = di_graph_elem.next() {
                    match elem.as_rule() {
                        Rule::VertexName => {
                            if last_name.is_none() {
                                list.add_vertex(Vertex::new(elem.as_str()));
                            } else {
                                let last = Vertex(last_name.unwrap());
                                let curr = Vertex::new(elem.as_str());

                                list.add_edge( &last,&curr);
                                list.add_edge( &curr,&last);

                            }
                            last_name = Some(elem.as_str().to_string());
                        }
                        Rule::Dash => (),
                        _ => unreachable!(),
                    }
                }
            }
        }
        _ => unreachable!(),
    };

    Ok(list)
}
*/

#[derive(pest_derive::Parser)]
#[grammar = "beefy_graph.pest"]
pub struct AdvancedSyntaxParser;

#[derive(Debug, Clone)]
pub enum Expr<'a> {
    Var(&'a str),
    Bracket(Box<[Expr<'a>]>),
}

#[derive(Debug, Clone,Copy)]
pub enum Direction {
    Left,
    Right,
    Both,
}

#[derive(Debug, Clone)]
pub enum Op {
    Arrow(Direction),
    FatArrow(Direction),
}

pub fn parse(prog: &str) -> Result<AdjList, Error<Rule>> {
    let ret: _ = AdvancedSyntaxParser::parse(Rule::GeneralGraph, prog)?;
    let mut list = AdjList::default();
    //let mut op_stack = vec![];
    //let mut val_stack = vec![];
    graph_expr(ret, &mut list);
    Ok(list)
}

impl From<&str> for Vertex {
    fn from(value: &str) -> Self {
        Vertex(value.into())
    }
}
fn graph_expr<'a>(ret: Pairs<'a, Rule>, list: &mut AdjList) -> Vec<Expr<'a>> {
    let mut val_stack: Vec<Expr<'a>> = vec![];
    let mut op_stack: Vec<Op> = vec![];
    use Direction::*;
    use Expr::*;
    use Op::*;
    let _wait :Option<u32> = None;
    //let mut
    for elem in ret.into_iter() {
        match elem.as_rule() {
            Rule::EOI => (),
            Rule::BracketedExpr => {
                let  new_val_stack = graph_expr(elem.into_inner(), list);
                //new_val_stack.iter().for_each(|elem| list.add_vertex(vertex))
                val_stack.push(Bracket(new_val_stack.into()));

                while !op_stack.is_empty() && val_stack.len() > 1{
                    let target = val_stack.pop().unwrap();

                    let source = val_stack.pop().unwrap();
                    let op = op_stack.pop().unwrap();
                    let ret = target.eval(source, op, list);
                    val_stack.push(ret);    
                } 
                

            }
            Rule::VertexName => {
                val_stack.push(Expr::Var(elem.as_str()));
                list.add_vertex(elem.as_str().into());

                while !op_stack.is_empty() && val_stack.len() > 1 {
                    let target = val_stack.pop().unwrap();

                    let source = val_stack.pop().unwrap();
                    let op = op_stack.pop().unwrap();
                    let ret = target.eval(source, op, list);
                    val_stack.push(ret);    
                }

            }

            Rule::RightArrow => {
                op_stack.push(Arrow(Right))
            }
            Rule::LeftArrow => {
                op_stack.push(Arrow(Left))
            }
            Rule::Dash => {
                op_stack.push(Arrow(Both))
            }
            Rule::RightFatArrow => {
                op_stack.push(FatArrow(Right))
            }
            Rule::LeftFatArrow => {
                op_stack.push(FatArrow(Left))
            }
            Rule::FatDash => {
                op_stack.push(FatArrow(Both))
            }

            Rule::GraphSingleton => (),
            rest => unreachable!("{:?}",rest),
        };
        
    }
    val_stack
}

impl<'a> Expr<'a> {
    

    fn eval(self, other: Self, op: Op, list: &mut AdjList) -> Self {
        use Direction::*;
        
        use Op::*;
        match (op, self, other) {
            (Arrow(arr@Left), target, source) | (Arrow(arr@Right), source, target) => {
                source.cartesian_product(arr,target, list)
            }

            (FatArrow(arr@Left), target, source) | (FatArrow(arr@Right), source, target) => {
                source.par_sum(arr,target, list)
            }

            (FatArrow(arr@Both), source, target) => {
                target.clone().par_sum(arr,source.clone(), list);
                source.par_sum(arr,target, list)
            }
            (Arrow(arr@Both), source, target) => {
                target.clone().cartesian_product(arr,source.clone(), list);
                source.cartesian_product(arr,target, list)
            }
        }
    }

    fn cartesian_product(self, dir: Direction,other: Self, list: &mut AdjList) -> Expr<'a> {
        //let mut val_stack = val_stack;
        match (self, other) {
            (Expr::Var(x), Expr::Var(y)) => {
                list.add_edge(&x.into(), &y.into());
                return Expr::Var( match dir  {
                    
                    Direction::Right => x,
                    _ => y,
                    
                });
            }
            ( Expr::Var(x), Expr::Bracket(y)) => {
                let ret = y
                    .iter()
                    .map(move |y| Expr::Var(x).cartesian_product(dir,y.clone(), list))
                    .collect();
                return match dir {
                    Direction::Right => Expr::Var(x),
                    _ => Expr::Bracket(ret),
                    
                }
            }
            (Expr::Bracket(x), Expr::Var(y)) => {
                let ret = x
                    .iter()
                    .map(|x| {
                        x.clone().cartesian_product(dir,Expr::Var(y), list)
                    })
                    .collect();
                return match dir {
                    Direction::Right => Expr::Bracket(ret),
                    _ => Expr::Var(y),
                    
                }
                
            }
            (Expr::Bracket(x), Expr::Bracket(y)) => {
                let mut ret = vec![];
                for e1 in x.iter() {
                    for e2 in y.iter() {
                        let v_stack =e1.clone().cartesian_product(dir,e2.clone(), list);
                        
                        ret.push(v_stack);
                    }
                }
                return Expr::Bracket(ret.into());
            }
        };
    }

    fn par_sum(self, dir: Direction,other: Self, list: &mut AdjList) -> Expr<'a> {
        //let mut val_stack = val_stack;
        match (self, other) {
            (Expr::Var(x), Expr::Var(y)) => {
                list.add_edge(&x.into(), &y.into());
                return Expr::Var( match dir  {
                    
                    Direction::Right => x,
                    _ => y,
                    
                });
            }
            ( Expr::Var(x), Expr::Bracket(y)) => {
                let ret = y
                    .iter()
                    .map(move |y| Expr::Var(x).par_sum(dir,y.clone(), list))
                    .collect();
                return match dir {
                    Direction::Right => Expr::Var(x),
                    _ => Expr::Bracket(ret),
                    
                }
            }
            (Expr::Bracket(x), Expr::Var(y)) => {
                let ret = x
                    .iter()
                    .map(|x| {
                        x.clone().par_sum(dir,Expr::Var(y), list)
                    })
                    .collect();
                return match dir {
                    Direction::Right => Expr::Bracket(ret),
                    _ => Expr::Var(y),
                    
                }
            }
            (Expr::Bracket(x), Expr::Bracket(y)) => {
                let mut ret = vec![];
                let iter = x.iter().zip(y.iter());
                for (e1, e2) in iter {
                    {
                        let v_stack = e1.clone().par_sum(dir,e2.clone(), list);
                        ret.push(v_stack);
                    }
                }
                return Expr::Bracket(ret.into());
            }
        };
    }
}

#[test]
fn test() {
    let ret = parse("a -> ( b c d ) -> ( d e f )")
        //.map(parse_expr)
        .map_err(|x| x.to_string());
    dbg!(ret);
}
