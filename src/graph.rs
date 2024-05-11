
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;


use layout::backends::svg::SVGWriter;
use layout::core::{base::Orientation, geometry::Point, style::*};

use layout::std_shapes::shapes::*;
use layout::topo::layout::VisualGraph;
//use layout::topo::placer::Placer;

#[test]
fn test_simple_graph() {
    let s = simple_graph();
    dbg!(s);
}

pub fn simple_graph() -> String {
    let mut vg = VisualGraph::new(Orientation::LeftToRight);

    // Define the node styles:
    let sp0 = ShapeKind::new_box("one");
    let sp1 = ShapeKind::new_box("two");
    let look0 = StyleAttr::simple();
    let look1 = StyleAttr::simple();
    let sz = Point::new(30., 30.);
    // Create the nodes:

    let node0 = Element::create(sp0, look0, Orientation::LeftToRight, sz);
    let node1 = Element::create(sp1, look1, Orientation::LeftToRight, sz);
    

    // Add the nodes to the graph, and save a handle to each node.
    let handle0 = vg.add_node(node0);
    let handle1 = vg.add_node(node1);

    // Add an edge between the nodes.
    let arrow = Arrow::simple("123");
    vg.add_edge(arrow, handle0, handle1);

    // Render the nodes to some rendering backend.
    let mut svg = SVGWriter::new();
    vg.do_it(false, false, false, &mut svg);
    svg.finalize()
}





pub trait Graph<Vertex>
where
    Vertex: Hash + Eq,
    

{
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
    fn contains_vertex(&self, vertex: &Vertex) -> bool;
    fn is_adjacent(&self, vertex: &Vertex, vertex2: &Vertex) -> bool;
    fn get_predecessors(&self, vertex: &Vertex) -> Option<Box<[&Vertex]>>;
    fn get_children(&self, vertex: &Vertex) -> Option<Box<[&Vertex]>>;

    fn add_vertex(&mut self, vertex: Vertex);
    fn remove_vertex(&mut self, vertex: Vertex);
    fn add_edge_between(&mut self, v1: &Vertex, v2: &Vertex);
    fn remove_edge_between(&mut self, v1: &Vertex, v2: &Vertex);
    fn get_vertices(&self) -> Box<[&Vertex]>;
    fn get_edges(&self) -> Box<[(&Vertex,&Vertex)]>;
    fn get_vertex_len(&self) -> usize;

    fn dfs<'a>(&'a self, vertex: &'a Vertex) -> Vec<&Vertex> {
        let mut visited = HashSet::new();

        let mut stack = vec![];
        stack.push(vertex);
        let mut ret = vec![];
        while let Some(v) = stack.pop() {
            if !visited.contains(&v) {
                visited.insert(v);
                ret.push(v);
            }

            for child in self
                .get_children(v)
                .expect("this shouldnt happen")
                .iter()
            {
                if !visited.contains(child) {
                    stack.push(child);
                }
            }
        }
        ret
    }

    fn bfs<'a>(&'a self, vertex: &'a Vertex) -> Vec<&Vertex> {
        let mut visited = HashSet::new();

        let mut stack = VecDeque::new();
        visited.insert(vertex);
        stack.push_back(vertex);
        let mut ret = vec![];
        while let Some(v) = stack.pop_front() {
            ret.push(v);
            for child in self
                .get_children(v)
                .expect("this shouldnt happen")
                .iter()
            {
                if !visited.contains(child) {
                    visited.insert(child);
                    stack.push_back(child);
                }
            }
        }
        ret
    }

    fn get_svg(&self,orientation:Orientation) ->String where
         String : for <'a,'b> From<&'a &'b Vertex> ,
    {
        let mut vg = VisualGraph::new(orientation);
        //self.get_edges()
        let mut map = HashMap::new();
        let bind  = self.get_vertices();
        bind.iter().for_each(|x| {
            let shape = ShapeKind::Circle(x.into());
            let look = StyleAttr::simple();
            let size = Point::new(30., 30.);
            let elem = Element::create(shape, look, Orientation::LeftToRight, size);
            let handle = vg.add_node(elem);
            map.insert(x, handle);
        });
        self.get_edges().iter().for_each(|edge|{
            let (v1,v2) = edge;
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

#[test]
fn simple_test() {}
