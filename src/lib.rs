mod graph;
mod utils;
mod graph_parser;
use std::{collections::{HashMap, HashSet, VecDeque}, rc::Rc};
use std::hash::Hash;
use graph::{ Graph};
use layout::{core::{base::Orientation, style::StyleAttr, geometry::Point}, topo::layout::VisualGraph, std_shapes::shapes::{ShapeKind, Element, Arrow}, backends::svg::SVGWriter};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, gracalc!");
}

#[wasm_bindgen]
pub fn wuhu() -> String {
    //alert("Hello, gracalc!");
    graph::simple_graph()
}

#[wasm_bindgen]
#[derive(Hash, Eq, PartialEq, Clone, Default,Debug)]
pub struct Vertex(String);

#[wasm_bindgen]
#[derive(Hash, Eq, PartialEq, Clone, Default,Debug)]
pub struct Edge(Vertex,Vertex);



impl From<Vertex> for String {
    fn from(value: Vertex) -> Self {
        value.0
    }
}
impl From<&Vertex> for String {
    fn from(value: &Vertex) -> Self {
        value.0.clone()
    }
}
impl From<&&Vertex> for String {
    fn from(value: &&Vertex) -> Self {
        value.0.clone().clone()
    }
}


#[wasm_bindgen]
impl Vertex{
    pub fn new(val:&str) -> Self{
        Self(val.to_string())
    }
}




#[wasm_bindgen]
#[derive(Clone, Default,Debug)]
pub struct AdjList {
    adj_list: HashMap<usize, HashSet<usize>>,
    vertices: Vec<Rc<Vertex>>,
    vertex_idx: HashMap<Rc<Vertex>, usize>,
}

#[wasm_bindgen]
impl  AdjList
{
    pub fn new()->Self{
        Self::default()
    }

    pub fn try_parse(prog:&str) -> Result<AdjList,String>{
        graph_parser::parse(prog).map_err(|x| x.to_string())
    }

    pub fn get_vertex_idx(&self, vertex: &Vertex) -> Option<usize> {
        self.vertex_idx.get(vertex).copied()
    }
    pub fn get_vertex(&self, idx: usize) -> Option<Vertex> {
        self.vertices.get(idx).map(|c| c.as_ref()).cloned()
    }

    pub fn insert_vertex(&mut self, vertex: Vertex) -> usize {
        //
        //let vert = self.vertices.last().unwrap().as_ref();
        if self.vertex_idx.contains_key(&vertex) {
            return *self.vertex_idx.get(&vertex).unwrap();
        }

        self.vertices.push(Rc::new(vertex));
        self.vertex_idx.insert(
            self.vertices.last().unwrap().clone(),
            self.vertices.len() - 1,
        );
        self.adj_list
            .insert(self.vertices.len() - 1, HashSet::new());
        self.vertices.len() - 1
    }

    pub fn contains_vertex(&self, vertex: &Vertex) -> bool {
        self.vertex_idx.contains_key(vertex)
    }

    pub fn is_adjacent(&self, vertex: &Vertex, vertex2: &Vertex) -> bool {
        let vertex = self.get_vertex_idx(vertex);
        let vertex2 = self.get_vertex_idx(vertex2);
        if let (Some(vertex1), Some(vertex2)) = (vertex, vertex2) {
            self.adj_list
                .get(&vertex1)
                .is_some_and(|set| set.contains(&vertex2))
        } else {
            return false;
        }
    }

    pub fn get_predecessors(&self, vertex: &Vertex) -> Option<Box<[Vertex]>> {
        let vertex = self.get_vertex_idx(vertex)?;

        Some(
            self.adj_list
                .iter()
                .filter(|(_, v)| v.contains(&vertex))
                .map(|x| x.0)
                .map(|x| self.get_vertex(*x).unwrap())
                .collect(),
        )
    }

    pub fn get_children(&self, vertex: &Vertex) -> Option<Box<[Vertex]>> {
        let vertex = self.get_vertex_idx(vertex)?;
        Some(
            self.adj_list
                .get(&vertex)?
                .iter()
                .filter_map(|x| self.get_vertex(*x))
                .collect(),
        )
    }

    pub fn add_vertex(&mut self, vertex: Vertex) {
        self.insert_vertex(vertex);
    }

    pub fn remove_vertex(&mut self, vertex: Vertex) {
        if let Some(vertex) = self.get_vertex_idx(&vertex) {
            self.adj_list.iter_mut().for_each(|(_, v)| {
                v.remove(&vertex);
            });
            self.adj_list.remove(&vertex);
        }
    }

    
    pub fn add_edge(&mut self, v1: &Vertex, v2: &Vertex) {
        let v1 = self.insert_vertex(v1.clone());
        let v2 = self.insert_vertex(v2.clone());
        self.adj_list.get_mut(&v1).unwrap().insert(v2);
    }

    pub fn remove_edge_between(&mut self, v1: &Vertex, v2: &Vertex) {
        let v1 = self.get_vertex_idx(v1);
        let v2 = self.get_vertex_idx(v2);
        if let (Some(v1), Some(v2)) = (v1, v2) {
            self.adj_list.get_mut(&v1).unwrap().remove(&v2);
        }
    }

    pub fn get_vertices(&self) -> Box<[Vertex]> {
        self.vertices.iter().map(|x| Vertex(x.0.clone())).collect()
    }

    pub fn get_edges(&self) -> Box<[Edge]> {
        self.adj_list.iter().flat_map(|(k, v)| {
            v.iter()
                .map(move |v| Edge(self.get_vertex(*k).unwrap(), self.get_vertex(*v).unwrap()) )
        }).collect()
    }

    pub fn get_vertex_len(&self) -> usize {
        self.vertices.len()
    }

    pub fn dfs(& self, vertex: & Vertex) -> Vec<Vertex> {
        let mut visited = HashSet::new();

        let mut stack = vec![];
        stack.push(vertex.clone());
        let mut ret = vec![];
        while let Some(v) = stack.pop() {
            if !visited.contains(&v) {
                visited.insert(v.clone());
                ret.push(v.clone());
            }
            let e =  self
                .get_children(&v)
                .clone()
                .unwrap();
            for child in e.iter().map(|x|x.clone())
            {
                if !visited.contains(&child) {
                    stack.push(child.clone());
                }
            }
        }
        ret.into_iter().map(|x| x.clone()).collect()
    }

    pub fn bfs(& self, vertex: & Vertex) -> Vec<Vertex> {
        let mut visited = HashSet::new();

        let mut stack = VecDeque::new();
        visited.insert(vertex.clone());
        stack.push_back(vertex.clone());
        let mut ret = vec![];
        while let Some(v) = stack.pop_front() {
            ret.push(v.clone());
            let e =  self
                .get_children(&v)
                .clone()
                .unwrap();
            for child in e.iter().map(|x|x.clone())
            {
                if !visited.contains(&child) {
                    
                    stack.push_back(child.clone());
                    visited.insert(child);
                }
            }
        }
        ret.into_iter().map(|x| x.clone()).collect()
    }

    pub fn get_svg(&self) ->String where
         //String : for <'a,'b> From<&'a &'b Vertex> ,
    {
        let mut vg = VisualGraph::new(Orientation::LeftToRight);
        //self.get_edges()
        let mut map = HashMap::new();
        let bind  = self.get_vertices();
        bind.into_iter().for_each(|x| {
            let shape = ShapeKind::Circle(x.into());
            let look = StyleAttr::simple();
            let size = Point::new(30., 30.);
            let elem = Element::create(shape, look, Orientation::LeftToRight, size);
            let handle = vg.add_node(elem);
            map.insert(x, handle);
        });
        self.get_edges().into_iter().for_each(|edge|{
            let Edge(v1,v2) = edge;
            let v1 = map.get(v1).unwrap();
            let v2 = map.get(v2).unwrap();
            let arr = Arrow::simple("");
            vg.add_edge(arr, *v1, *v2);
        });
        let mut svg = SVGWriter::new();
        vg.do_it(false, false, false, &mut svg);
        svg.finalize()
    }

}

/*
#[derive(Clone, Default)]
pub struct AdjList<Vertex> {
    adj_list: HashMap<usize, HashSet<usize>>,
    vertices: Vec<Rc<Vertex>>,
    vertex_idx: HashMap<Rc<Vertex>, usize>,
}


impl<Vertex>  AdjList<Vertex>
where
    Vertex: Clone + Hash + Eq,
{
    fn get_vertex_idx(&self, vertex: &Vertex) -> Option<usize> {
        self.vertex_idx.get(vertex).copied()
    }
    fn get_vertex(&self, idx: usize) -> Option<&Vertex> {
        self.vertices.get(idx).map(|c| c.as_ref())
    }

    fn insert_vertex(&mut self, vertex: Vertex) -> usize {
        //
        //let vert = self.vertices.last().unwrap().as_ref();
        if self.vertex_idx.contains_key(&vertex) {
            return *self.vertex_idx.get(&vertex).unwrap();
        }

        self.vertices.push(Rc::new(vertex));
        self.vertex_idx.insert(
            self.vertices.last().unwrap().clone(),
            self.vertices.len() - 1,
        );
        self.adj_list
            .insert(self.vertices.len() - 1, HashSet::new());
        self.vertices.len() - 1
        //todo!()
    }
}

impl<'a, Vertex> Graph<Vertex> for AdjList<Vertex>
where
    Vertex: Hash + Eq + Clone,
    
{
    fn contains_vertex(&self, vertex: &Vertex) -> bool {
        self.vertex_idx.contains_key(vertex)
    }

    fn is_adjacent(&self, vertex: &Vertex, vertex2: &Vertex) -> bool {
        let vertex = self.get_vertex_idx(vertex);
        let vertex2 = self.get_vertex_idx(vertex2);
        if let (Some(vertex1), Some(vertex2)) = (vertex, vertex2) {
            self.adj_list
                .get(&vertex1)
                .is_some_and(|set| set.contains(&vertex2))
        } else {
            return false;
        }
    }

    fn get_predecessors(&self, vertex: &Vertex) -> Option<Box<[&Vertex]>> {
        let vertex = self.get_vertex_idx(vertex)?;

        Some(
            self.adj_list
                .iter()
                .filter(|(_, v)| v.contains(&vertex))
                .map(|x| x.0)
                .map(|x| self.get_vertex(*x).unwrap())
                .collect(),
        )
    }

    fn get_children(&self, vertex: &Vertex) -> Option<Box<[&Vertex]>> {
        let vertex = self.get_vertex_idx(vertex)?;
        Some(
            self.adj_list
                .get(&vertex)?
                .iter()
                .filter_map(|x| self.get_vertex(*x))
                .collect(),
        )
    }

    fn add_vertex(&mut self, vertex: Vertex) {
        self.insert_vertex(vertex);
    }

    fn remove_vertex(&mut self, vertex: Vertex) {
        if let Some(vertex) = self.get_vertex_idx(&vertex) {
            self.adj_list.iter_mut().for_each(|(_, v)| {
                v.remove(&vertex);
            });
            self.adj_list.remove(&vertex);
        }
    }

    
    fn add_edge_between(&mut self, v1: &Vertex, v2: &Vertex) {
        let v1 = self.insert_vertex(v1.clone());
        let v2 = self.insert_vertex(v2.clone());
        self.adj_list.get_mut(&v1).unwrap().insert(v2);
    }

    fn remove_edge_between(&mut self, v1: &Vertex, v2: &Vertex) {
        let v1 = self.get_vertex_idx(v1);
        let v2 = self.get_vertex_idx(v2);
        if let (Some(v1), Some(v2)) = (v1, v2) {
            self.adj_list.get_mut(&v1).unwrap().remove(&v2);
        }
    }

    fn get_vertices(&self) -> Box<[&Vertex]> {
        self.vertices.iter().map(|x| x.as_ref()).collect()
    }

    fn get_edges(&self) -> Box<[(&Vertex,&Vertex)]> {
        self.adj_list.iter().flat_map(|(k, v)| {
            v.iter()
                .map(move |v| (self.get_vertex(*k).unwrap(), self.get_vertex(*v).unwrap()) )
        }).collect()
    }

    fn get_vertex_len(&self) -> usize {
        self.vertices.len()
    }

    fn from_vertices<
            T,I,O
           >(it:T)-> O
        where
        O : Default +  Graph<Vertex>,
        T : Iterator<Item = (Vertex,I)>,
       I : Iterator<Item = Vertex>
    {
        let mut ret = O::default();
        for (vertex,nboors) in it  {
           for nboor in nboors{
               ret.add_edge_between(&vertex, &nboor);
           } 
        }
        ret
    }

}
*/
