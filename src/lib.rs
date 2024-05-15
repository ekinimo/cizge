mod graph;
mod graph_parser;
mod my_svg_writer;
mod utils;
use graph::{AdjecencyList, GetVertex, AddOrRemoveEdge};
//use graph::Graph;
use layout::{
    backends::svg::SVGWriter,
    core::{base::Orientation, geometry::Point, style::StyleAttr, color::Color},
    std_shapes::shapes::{Arrow, Element, ShapeKind},
    topo::layout::VisualGraph,
};
use std::{hash::Hash, fmt::Display, ops::{Shl, Shr}};
use std::{
    cell::RefCell,
    collections::{hash_set, vec_deque, HashMap, HashSet, VecDeque},
    rc::Rc,
};
use wasm_bindgen::prelude::*;

use crate::my_svg_writer::MySVGWriter;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
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
#[derive(Hash, Eq, PartialEq, Clone, Default, Debug)]
pub struct Vertex(String);

#[wasm_bindgen]
#[derive(Hash, Eq, PartialEq, Clone, Default, Debug)]
pub struct Edge(Vertex, Vertex);

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
impl Vertex {
    pub fn new(val: &str) -> Self {
        Self(val.to_string())
    }
}

#[wasm_bindgen]
#[derive(Clone, Default, Debug)]
pub struct AdjList {
    adj_list: HashMap<usize, HashSet<usize>>,
    vertices: Vec<Rc<Vertex>>,
    vertex_idx: HashMap<Rc<Vertex>, usize>,
}

#[wasm_bindgen]
impl AdjList {
    pub fn is_cyclic(&self) -> bool {
        log("DFS    :     ");
        for vertex in self.get_vertices_ref().iter(){
            let mut map = HashSet::new();
            let visited = Rc::new( RefCell::new( &mut map));
            log("DFS    :     ");
            log(&format!("         {:?}",vertex));
            DFS::new(self,  visited, vertex).for_each(|_x|
                                                      {
                                                          log(&format!("                  {:?}",vertex));        
                                                      }
            );

        }
        false
    }
    pub fn new() -> Self {
        Self::default()
    }

    pub fn try_parse(prog: &str) -> Result<AdjList, String> {
        graph_parser::parse(prog).map_err(|x| x.to_string())
    }

    pub fn get_vertex_idx(&self, vertex: &Vertex) -> Option<usize> {
        self.vertex_idx.get(vertex).copied()
    }
    pub fn get_vertex(&self, idx: usize) -> Option<Vertex> {
        self.vertices.get(idx).map(|c| c.as_ref()).cloned()
    }

    fn get_vertex_ref(&self, idx: usize) -> Option<&Vertex> {
        self.vertices.get(idx).map(|c| c.as_ref())
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
            false
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

    fn get_predecessors_ref(&self, vertex: &Vertex) -> Option<Box<[&Vertex]>> {
        let vertex = self.get_vertex_idx(vertex)?;

        Some(
            self.adj_list
                .iter()
                .filter(|(_, v)| v.contains(&vertex))
                .map(|x| x.0)
                .map(|x| self.get_vertex_ref(*x).unwrap())
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

    fn get_children_ref(&self, vertex: &Vertex) -> Option<Box<[&Vertex]>> {
        let vertex = self.get_vertex_idx(vertex)?;
        Some(
            self.adj_list
                .get(&vertex)?
                .iter()
                .filter_map(|x| self.get_vertex_ref(*x))
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
        self.adj_list
            .iter()
            .flat_map(|(k, v)| {
                v.iter()
                    .map(move |v| Edge(self.get_vertex(*k).unwrap(), self.get_vertex(*v).unwrap()))
            })
            .collect()
    }

    fn get_vertices_ref(&self) -> Box<[&Vertex]> {
        self.vertices.iter().map(|x| x.as_ref()).collect()
    }

    fn get_edges_ref(&self) -> Box<[(&Vertex, &Vertex)]> {
        self.adj_list
            .iter()
            .flat_map(|(k, v)| {
                v.iter().map(move |v| {
                    (
                        self.get_vertex_ref(*k).unwrap(),
                        self.get_vertex_ref(*v).unwrap(),
                    )
                })
            })
            .collect()
    }

    pub fn get_vertex_len(&self) -> usize {
        self.vertices.len()
    }

    pub fn dfs(&self, vertex: &Vertex) -> Vec<Vertex> {
        let mut visited = HashSet::new();

        let mut stack = vec![];
        stack.push(vertex.clone());
        let mut ret = vec![];
        while let Some(v) = stack.pop() {
            if !visited.contains(&v) {
                visited.insert(v.clone());
                ret.push(v.clone());
            }
            let e = self.get_children(&v).clone().unwrap();
            for child in e.iter() {
                if !visited.contains(child) {
                    stack.push(child.clone());
                }
            }
        }
        ret.into_iter().collect()
    }

    /*fn dfs_ref<'a>(&'a self, vertex: &'a Vertex) -> DFS<'a> {
        /*let mut visited = HashSet::new();

        let mut stack = vec![];
        stack.push(vertex);
        let mut ret = vec![];
        while let Some(v) = stack.pop() {
            if !visited.contains(&v) {
                visited.insert(v);
                ret.push(v);
            }
            let e =  self
                .get_children_ref(&v)
                .clone()
                .unwrap();
            for child in e.iter()
            {
                if !visited.contains(child) {
                    stack.push(child);
                }
            }
        }
        ret.into_iter().collect()*/
        let mut visited = HashSet::new();
        DFS::new(self, &mut visited,vertex)
    }*/

    fn bfs_ref<'a>(&'a self, vertex: &'a Vertex) -> Vec<&Vertex> {
        let mut visited = HashSet::new();

        let mut stack = VecDeque::new();
        visited.insert(vertex);
        stack.push_back(vertex);
        let mut ret = vec![];
        while let Some(v) = stack.pop_front() {
            ret.push(v);
            let e = self.get_children_ref(v).clone().unwrap();
            for child in e.iter() {
                if !visited.contains(child) {
                    stack.push_back(child);
                    visited.insert(child);
                }
            }
        }
        ret.into_iter().collect()
    }
    pub fn bfs(&self, vertex: &Vertex) -> Vec<Vertex> {
        let mut visited = HashSet::new();

        let mut stack = VecDeque::new();
        visited.insert(vertex.clone());
        stack.push_back(vertex.clone());
        let mut ret = vec![];
        while let Some(v) = stack.pop_front() {
            ret.push(v.clone());
            let e = self.get_children(&v).clone().unwrap();
            for child in e.iter().cloned() {
                if !visited.contains(&child) {
                    stack.push_back(child.clone());
                    visited.insert(child);
                }
            }
        }
        ret.into_iter().collect()
    }



    pub fn get_svg(&self) -> String where
         //String : for <'a,'b> From<&'a &'b Vertex> ,
        Vertex:Display,
    {
        use graph ;
        use crate::graph::DirectedGraph;
        let adjl : AdjecencyList = self.into();
        let scc = adjl.SCC();
        //let mut col = 0xDEADCAB;
        //let inc = 0x0A0A0A;
        let mut style_map = HashMap::new();
        log(&format!("{:?}",&scc));
        let n = (scc.len()) as f32;
        let n = 340.0/n; // can go upto max 360 but then colors get super similar
        let mut start = 0.0;
        
        for group in scc{
         
            for elem in group{
                let col = hsv2rgb(start, 0.3, 0.95);
                log(&format!("{start} => {col}"));
                style_map.insert(self.get_vertex(elem).unwrap(), col);
            }
            
            start += n;
        }

        let mut vg = VisualGraph::new(Orientation::LeftToRight);
        //self.get_edges()
        let mut map = HashMap::new();
        let bind = self.get_vertices();
        for x in bind.iter() {
            
            let shape = ShapeKind::Box(x.into());
            let col = style_map.get(x).unwrap();

            let width :f64 = if x.0.starts_with("__") && x.0.ends_with("__"){ 0.75*5.0 *  f64::from((x.0.len()-4) as u32)} else{ 5. *  f64::from(x.0.len() as u32) } ;
            let size = Point::new(35. + width, 35.);
            let look = StyleAttr::new(Color::new(0xffffff), 1, Some(Color::new(*col)),  5,10);
            let elem = Element::create(shape, look, Orientation::LeftToRight, size);
            let handle = vg.add_node(elem);
            map.insert(x, handle);
        };
        let binding = self.get_edges();
        let edges = binding.into_iter().map(|edge| {
            let Edge(v1, v2) = edge;
            let v11 = map.get(v1).unwrap();
            let v22 = map.get(v2).unwrap();
            let arr = Arrow::simple("");
            vg.add_edge(arr, *v11, *v22);
            (v1,v2)
        }).rev().collect();
        let mut svg = MySVGWriter::new(edges);
        vg.do_it(false, false, false, &mut svg);
        svg.finalize()
    }


}

impl From<&AdjList> for graph::AdjecencyList{
    fn from(value: &AdjList) -> Self {
        let mut  ret = AdjecencyList::default();
        value.adj_list.keys().for_each(|me| { let a = ret.get_new_vertex(); });
        value.adj_list.iter().for_each(|(k,v)| {
            v.iter().for_each(|v| {
                ret.add_edge_between(&k, v);
            });
        });

        ret

    }
}

impl Display for Vertex{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, Clone)]
struct DFS<'a> {
    pub start: &'a Vertex,
    pub to_visit: Vec<&'a Vertex>,
    pub visited: Rc<RefCell<&'a mut HashSet<&'a Vertex>>>,
    pub graph: &'a AdjList,
}

impl<'a> DFS<'a> {
    fn new(
        graph: &'a AdjList,
        visited: Rc<RefCell<&'a mut HashSet<&'a Vertex>>>,
        start: &'a Vertex,
    ) -> Self {
        let to_visit = vec![start];
        //let mut visited = Rc::new(RefCell::new(set));
        //visited.borrow_mut().insert(start);

        Self {
            start,
            to_visit,
            visited,
            graph,
        }
    }

    pub fn visited_contains(&self, value: &Vertex) -> bool {
        self.visited.borrow().contains(value)
    }

    pub fn to_visit_iter(&self) -> std::slice::Iter<'_, &Vertex> {
        self.to_visit.iter()
    }
}

impl<'a> Iterator for DFS<'a> {
    type Item = &'a Vertex;

    fn next(&mut self) -> Option<Self::Item> {
        let candidate = self.to_visit.pop()?;
        if !self.visited.borrow().contains(candidate) {
            self.visited.borrow_mut().insert(candidate);
        }
        for child in self.graph.get_children_ref(candidate).unwrap().iter() {
            if !self.visited.borrow().contains(child) {
                self.to_visit.push(child);
            }
        }
        Some(candidate)
    }
}

struct BFS<'a> {
    start: &'a Vertex,
    to_visit: VecDeque<&'a Vertex>,
    visited: HashSet<&'a Vertex>,
    graph: &'a AdjList,
}

impl<'a> BFS<'a> {
    fn new(start: &'a Vertex, graph: &'a AdjList) -> Self {
        let mut to_visit = VecDeque::new();
        let mut visited = HashSet::new();
        visited.insert(start);
        to_visit.push_back(start);
        Self {
            start,
            to_visit,
            visited,
            graph,
        }
    }

    pub fn visited_contains(&self, value: &Vertex) -> bool {
        self.visited.contains(value)
    }

    pub fn visited_iter(&self) -> hash_set::Iter<'_, &Vertex> {
        self.visited.iter()
    }

    pub fn to_visit_iter(&self) -> vec_deque::Iter<'_, &Vertex> {
        self.to_visit.iter()
    }
}

impl<'a> Iterator for BFS<'a> {
    type Item = &'a Vertex;

    fn next(&mut self) -> Option<Self::Item> {
        let candidate = self.to_visit.pop_front()?;
        for child in self.graph.get_children_ref(candidate).unwrap().iter() {
            if !self.visited.contains(child) {
                self.to_visit.push_back(child);
                self.visited.insert(child);
            }
        }
        Some(candidate)
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

fn hsv2rgb(h:f32,s:f32,v:f32)->u32{

    let c = s*v;
    let x = c*(1.0 - (((h   /60.0) % 2.0) -1.0).abs());
    let m = v-c;

    let (r,g,b) = match h {
        _ if h < 60. =>  (c+m,x+m,m),
        _ if h < 120. => (x+m,c+m,m),
        _ if h < 180. => (m,c+m,x+m),
        _ if h < 240. => (m,x+m,c+m),
        _ if h < 300. => (x+m,m,c+m),
        _   => (c+m,m,x+m),
        
    };


    let r = (255.0 *r ).round() as   u32;
    let g = (255.0 *g ).round() as  u32;
    let b = (255.0 *b ).round() as u32;
    let ret = r.shl(16) | g.shl(8) | b;
    //log()rust
    log(&format!(" r = {r:x}  g = {g:x}  b = {b:x}   ==> {ret:x}"));
    return ret
}
