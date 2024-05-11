use pest::{error::Error, Parser};
use pest_derive::Parser;

use crate::{AdjList, Vertex};

#[derive(Parser)]
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
        Rule::DiGraph => {
            for snip in general_graph.into_inner().into_iter() {
                let mut di_graph_elem = snip.into_inner();
                let mut last_name = None;
                while let Some(elem) = di_graph_elem.next() {
                    match elem.as_rule() {
                        Rule::VertexName => {
                            if last_name.is_none() {
                                list.add_vertex(Vertex::new(elem.as_str()));
                            } else {
                                list.add_edge(
                                    &Vertex(last_name.unwrap()),
                                    &Vertex::new(elem.as_str()),
                                )
                            }
                            last_name = Some(elem.as_str().to_string());
                        }
                        Rule::Arrow => (),
                        _ => unreachable!(),
                    }
                }
            }
        }
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
