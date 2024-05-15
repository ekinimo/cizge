use std::borrow::BorrowMut;
use std::collections::{HashMap, HashSet, VecDeque};
use std::default;
use std::fmt::Debug;
use std::hash::Hash;
use std::slice::{Iter, IterMut};

use bit_matrix::row::BitSlice;
use bit_matrix::BitMatrix;
use layout::backends::svg::SVGWriter;
use layout::core::{base::Orientation, geometry::Point, style::*};

use layout::std_shapes::shapes::*;
use layout::topo::layout::VisualGraph;

use crate::my_svg_writer::MySVGWriter;

//use layout::topo::placer::Placer;

/*
#[derive(Clone,Debug)]
pub struct DirectedGraphDFS<'a, T: DirectedGraph<Vertex>+Debug, Vertex: Eq + Hash+Debug> {
    graph: &'a T,
    stack: Vec<&'a Vertex>,
    active_children: HashMap<&'a Vertex, usize>,
    parents: HashMap<&'a Vertex, &'a Vertex>,
    pub finished: HashSet<&'a Vertex>,
    pub discovered: HashSet<&'a Vertex>,
}

impl<'a, T: DirectedGraph<Vertex> + 'a + Debug, Vertex: Eq + Hash+Debug> DirectedGraphDFS<'a, T, Vertex> {
    pub fn new(graph: &'a T, start: &'a Vertex) -> Self {
        Self {
            graph,
            stack: vec![start],
            active_children: HashMap::default(),
            parents: HashMap::default(),
            finished: HashSet::default(),
            discovered: HashSet::default(),
        }
    }
}

impl<'a, T, Vertex> Iterator for DirectedGraphDFS<'a, T, Vertex> where
    Vertex:Debug,
    T: DirectedGraph<Vertex> + Debug,
    Vertex: Hash + Eq {
    type Item = &'a Vertex;

    fn next(&mut self) -> Option<Self::Item> {
        //dbg!(&self);
        let mut val = self.stack.pop()?;
        dbg!((&val,&self.discovered,&self.finished,&self.active_children));
        if !self.discovered.contains(val) {
            self.discovered.insert(val);
            let children = self.graph.get_children(val).unwrap();
            for child in children.into_iter() {
                if self.discovered.contains(child) {
                    *self.active_children.get_mut(child)? -= 1;
                }
                *self.active_children.get_mut(val)? += 1;
                self.parents.insert(child, val);
                self.stack.push(child);
                dbg!(("children loop",&child,&self.discovered,&self.finished,&self.active_children));
            }
            loop {
                dbg!(("second loop",&val,&self.discovered,&self.finished,&self.active_children));
                if !(!self.parents.contains_key(val)
                    && self.active_children.get(val).unwrap_or(&6969) == &0)
                {
                    break;
                }
                self.finished.insert(val);
                val = self.parents.get(val).unwrap();
                if !self.parents.contains_key(val) {
                    *self.active_children.get_mut(val)? -= 1;
                }
            }
            //return Some(val)
        }
        Some(val)
    }
}
*/
#[test]
fn test_is_cyclic() {
    let mut graph = AdjecencyList::default();
    let a = graph.get_new_vertex();
    let b = graph.get_new_vertex();
    let c = graph.get_new_vertex();
    let d = graph.get_new_vertex();
    let e = graph.get_new_vertex();
    let f = graph.get_new_vertex();
    let g = graph.get_new_vertex();
    let h = graph.get_new_vertex();
    let i = graph.get_new_vertex();
    let j = graph.get_new_vertex();

    graph.add_edge_between(&a, &b);
    graph.add_edge_between(&b, &c);
    graph.add_edge_between(&c, &d);
    graph.add_edge_between(&c, &e);
    graph.add_edge_between(&d, &a);
    graph.add_edge_between(&d, &e);
    graph.add_edge_between(&e, &c);
    graph.add_edge_between(&e, &f);
    graph.add_edge_between(&f, &g);
    graph.add_edge_between(&f, &i);
    graph.add_edge_between(&g, &f);
    graph.add_edge_between(&g, &h);
    graph.add_edge_between(&h, &j);
    graph.add_edge_between(&i, &f);
    graph.add_edge_between(&i, &g);
    graph.add_edge_between(&j, &i);
    
    //dbg!(&graph);
    assert_eq!(true, graph.is_cyclic());
    dbg!(graph.SCC());

}

pub trait DirectedGraph<Vertex>
where
    Vertex: Hash + Eq,
{
    fn contains_vertex(&self, vertex: &Vertex) -> bool;
    fn is_adjacent(&self, vertex: &Vertex, vertex2: &Vertex) -> bool;
    fn get_predecessors(&self, vertex: &Vertex) -> Option<Box<[Vertex]>>;
    fn get_children(&self, vertex: &Vertex) -> Option<Box<[&Vertex]>>;
    fn get_predecessor_count(&self, vertex: &Vertex) -> Option<usize>;
    fn get_children_count(&self, vertex: &Vertex) -> Option<usize>;

    fn get_vertices(&self) -> Box<[Vertex]>;
    fn get_edges(&self) -> Box<[(Vertex, Vertex)]>;
    fn get_vertex_len(&self) -> usize;

    fn topological_sort(&self) -> Option<Vec<Vertex>>
    where
        Self: AddOrRemoveEdge<Vertex> + Clone,
        Vertex: Clone,
    {
        let mut oth = self.clone();

        let mut no_entries: Vec<_> = oth
            .get_vertices()
            .iter()
            .cloned()
            .filter(|x| oth.get_predecessor_count(x).unwrap() == 0)
            .collect();
        let mut ret = vec![];
        while let Some(n) = no_entries.pop() {
            ret.push(n.clone());
            let children: Box<[_]> = oth
                .get_children(&n)
                .unwrap()
                .into_iter()
                .cloned()
                .cloned()
                .collect();

            children.into_iter().for_each(|child| {
                oth.remove_edge_between(&n, &child);
                if oth.get_predecessor_count(&child).unwrap() == 0 {
                    no_entries.push(child.clone().clone())
                }
            });
        }

        if oth.get_edges().len() != 0 {
            None
        } else {
            Some(ret.clone().into())
        }
    }

    fn topological_levels(&self) -> Option<Vec<Vec<Vertex>>>
    where
        Self: AddOrRemoveEdge<Vertex> + Clone,
        Vertex: Clone,
    {
        let mut oth = self.clone();
        let mut ret = vec![];
        loop {
            let no_entries: Vec<_> = oth
                .get_vertices()
                .iter()
                .cloned()
                .filter(|x| oth.get_predecessor_count(x).unwrap() == 0)
                .collect();
            if no_entries.len() == 0 {
                break;
            }
            ret.push(no_entries.clone());
            for n in no_entries {
                //ret.push(n.clone());
                let children: Box<[_]> = oth
                    .get_children(&n)
                    .unwrap()
                    .into_iter()
                    .cloned()
                    .cloned()
                    .collect();

                children.into_iter().for_each(|child| {
                    oth.remove_edge_between(&n, &child);
                });
            }
        }

        if oth.get_edges().len() != 0 {
            None
        } else {
            Some(ret.clone().into())
        }
    }

    fn is_cyclic(&self) -> bool
    where
        Self: Sized + Debug + AddOrRemoveEdge<Vertex> + Clone,
        Vertex: Debug + Clone,
    {
        //let mut finished: HashSet<&Vertex> = HashSet::new();
        //let mut discovered: HashSet<&Vertex> = HashSet::new();
        self.topological_sort().is_none()
        //false
    }
    /*fn get_svg(&self, orientation: Orientation) -> String
    where
        String: for<'a> From<&'a Vertex>,
    {
        let mut vg = VisualGraph::new(orientation);
        //self.get_edges()
        let mut map = HashMap::new();
        let bind = self.get_vertices();
        bind.iter().for_each(|x| {
            let shape = ShapeKind::Circle(x.into());
            let look = StyleAttr::simple();
            let size = Point::new(30., 30.);
            let elem = Element::create(shape, look, Orientation::LeftToRight, size);
            let handle = vg.add_node(elem);
            map.insert(x, handle);
        });
        self.get_edges().iter().for_each(|edge| {
            let (v1, v2) = edge;
            let v1 = map.get(v1).unwrap();
            let v2 = map.get(v2).unwrap();
            let arr = Arrow::simple("");
            vg.add_edge(arr, *v1, *v2);
        });
        let mut svg = SVGWriter::new();
        vg.do_it(false, false, false, &mut svg);
        svg.finalize()
    }*/

    fn SCC(&self) -> Vec<Vec<Vertex>>
    where
        Self : Sized,
        Vertex : Clone + Debug
    {
        let mut params: SccParams<'_, Vertex> =  SccParams {
             ret : vec![],
             time : 0,
             low : HashMap::default(),
             discovery : HashMap::default(),
             stack : vec![],
             stack_member :HashSet::default(),
        };
        let binding = self.get_vertices();
        for child in binding.into_iter()  {
            if *params.discovery.get(child).unwrap_or(&-1) == -1 {
                params = scc_util(self, child, params);
            }
        }
        params.ret
    }

}

struct SccParams<'a, Vertex:Debug> {
    ret: Vec<Vec<Vertex>>,
    time:  usize,

    low: HashMap<&'a Vertex, isize>,
    discovery: HashMap<&'a Vertex, isize>,

    stack: Vec<&'a Vertex>,
    stack_member: HashSet<&'a Vertex>,
}

fn scc_util<'a, Vertex: Debug+ Hash + Eq+Clone, T: DirectedGraph<Vertex>>(
    graph: &'a T,
    vertex: &'a Vertex,

    params: SccParams<'a, Vertex>,
) -> SccParams<'a, Vertex> {
    let SccParams {
        mut ret,
        mut time,
        mut low,
        mut discovery,
        mut stack,
        mut stack_member,
    } = params;

    use std::collections::hash_map::Entry::*;
    discovery.insert(vertex, time as isize);
    low.insert(vertex, time as isize);
    time += 1;
    stack_member.insert(vertex);
    stack.push(vertex);

    for v in graph.get_children(vertex).unwrap().iter() {
        if discovery.get(v).unwrap_or(&-1) == &-1 {
 SccParams {
                 ret,
                 time,
                 low,
                 discovery,
                 stack,
                 stack_member,
            } = scc_util(
                graph,
                v,
                
                SccParams {
                     ret,
                    time,
                    low,
                    discovery,
                    stack,
                    stack_member,
                },
            );
            let min = *low
                .get(vertex)
                .unwrap_or(&mut -1)
                .min(low.get(v).unwrap_or(&mut -1));
            match low.entry(vertex) {
                Occupied(mut occ) => *occ.get_mut() = min,
                Vacant(vac) => {
                    vac.insert(min);
                }
            }
        } else if stack_member.contains(v) {
            let min = *low
                .get(vertex)
                .unwrap_or(&mut -1)
                .min(discovery.get(v).unwrap_or(&mut -1));
            match low.entry(vertex) {
                Occupied(mut occ) => *occ.get_mut() = min,
                Vacant(vac) => {
                    vac.insert(min);
                }
            }
        }else{}
    }
    let mut w  = None;
    if low.get(vertex) == discovery.get(vertex){
        let mut tmp = vec![];
        /*while w != Some(vertex){
            w = stack.pop();
            if w.is_none(){break;}
            tmp.push(w.unwrap().clone().clone());
            stack_member.remove(w.unwrap());
        }*/
        loop{
            w = stack.pop();
            dbg!(&w);
            stack_member.remove(w.unwrap());
            tmp.push(w.unwrap().clone().clone());
            if Some(vertex) == w  || w == None {break}
        }
        ret.push( tmp);
        //insert ret here
    }
    SccParams {
        ret,
        time,
        low,
      discovery,
        stack,
        stack_member,
    }
}
pub trait DirectedGraphBuilder<Vertex> {
    type CanAddArbitaryVertexName;
    type VertexHandle;
}

pub trait GetVertex<Vertex> {
    fn get_new_vertex(&mut self) -> Vertex;
}

pub trait RemoveVertex<Vertex> {
    fn remove_vertex(&mut self, vertex: &Vertex);
}

pub trait AddVertex<Vertex> {
    fn add_vertex(&mut self, vertex: &Vertex);
}

pub trait AddOrRemoveEdge<Vertex> {
    fn add_edge_between(&mut self, v1: &Vertex, v2: &Vertex);
    fn remove_edge_between(&mut self, v1: &Vertex, v2: &Vertex);
}

#[derive(Clone, Default, Debug)]
pub struct AdjecencyList(Vec<HashSet<usize>>);

impl DirectedGraph<usize> for AdjecencyList {
    fn contains_vertex(&self, vertex: &usize) -> bool {
        self.len() > *vertex
    }

    fn is_adjacent(&self, vertex: &usize, vertex2: &usize) -> bool {
        self.get(*vertex).is_some_and(|set| set.contains(vertex2))
    }

    fn get_predecessors(&self, vertex: &usize) -> Option<Box<[usize]>> {
        if !(self.len() > *vertex) {
            return None;
        }
        Some(
            self.iter()
                .enumerate()
                .filter_map(|(i, x)| if x.contains(vertex) { Some(i) } else { None })
                .collect(),
        )
    }

    fn get_children(&self, vertex: &usize) -> Option<Box<[&usize]>> {
        Some(self.get(*vertex)?.iter().map(|x| x).collect())
    }

    fn get_predecessor_count(&self, vertex: &usize) -> Option<usize> {
        if !(self.len() > *vertex) {
            return None;
        }
        Some(
            self.iter()
                .enumerate()
                .filter_map(|(i, x)| if x.contains(vertex) { Some(i) } else { None })
                .count(),
        )
    }

    fn get_children_count(&self, vertex: &usize) -> Option<usize> {
        self.get(*vertex).map(HashSet::len)
    }

    fn get_vertices(&self) -> Box<[usize]> {
        (0..self.len()).collect()
    }

    fn get_edges(&self) -> Box<[(usize, usize)]> {
        self.iter()
            .enumerate()
            .flat_map(|(p, c)| c.iter().map(move |x| (p, *x)))
            .collect()
    }

    fn get_vertex_len(&self) -> usize {
        self.len()
    }
}

impl GetVertex<usize> for AdjecencyList {
    fn get_new_vertex(&mut self) -> usize {
        self.push();
        self.0.len() - 1
    }
}

impl AddOrRemoveEdge<usize> for AdjecencyList {
    fn add_edge_between(&mut self, v1: &usize, v2: &usize) {
        if !(self.len() > *v1 || self.len() > *v2) {
            eprint!(
                "Adjecency list doesnt supprt adding edges between nonexistent vertices {} {} {}",
                self.len(),
                v1,
                v2
            );
            return;
        }
        self.0[*v1].insert(*v2);
    }

    fn remove_edge_between(&mut self, v1: &usize, v2: &usize) {
        if !(self.len() > *v1 || self.len() > *v2) {
            eprint!("Adjecency list doesnt supprt removing edges between nonexistent vertices");
            return;
        }
        self.0[*v1].remove(v2);
    }
}

impl GetVertex<usize> for AdjecencyMatrix {
    fn get_new_vertex(&mut self) -> usize {
        let s = self.size();
        let mut ret = BitMatrix::new(s + 1, s + 1);

        (0..s)
            .flat_map(|i| (0..s).map(move |j| (i, j)))
            .for_each(|(i, j)| {
                let v = &self[(i, j)];
                ret.set(i, j, *v)
            });
        *self = AdjecencyMatrix(ret, self.1.clone());
        s + 1
    }
}

impl AddOrRemoveEdge<usize> for AdjecencyMatrix {
    fn add_edge_between(&mut self, v1: &usize, v2: &usize) {
        if !(self.size() > *v1 || self.size() > *v2) {
            eprint!("Adjecency list doesnt supprt adding edges between nonexistent vertices");
            return;
        }
        self.0.set(*v1, *v2, true);
    }

    fn remove_edge_between(&mut self, v1: &usize, v2: &usize) {
        if !(self.size() > *v1 || self.size() > *v2) {
            eprint!("Adjecency list doesnt supprt adding edges between nonexistent vertices");
            return;
        }
        self.0.set(*v1, *v2, false);
    }
}

impl AdjecencyList {
    pub fn push(&mut self) -> usize {
        self.0.push(HashSet::default());
        self.0.len()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Option<&HashSet<usize>> {
        self.0.get(index)
    }

    pub fn iter(&self) -> Iter<'_, HashSet<usize>> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, HashSet<usize>> {
        self.0.iter_mut()
    }
}
pub struct AdjecencyMatrix(BitMatrix, Box<[usize]>);
impl AdjecencyMatrix {
    fn size(&self) -> usize {
        self.0.size().0
    }
}

impl std::ops::Index<usize> for AdjecencyMatrix {
    type Output = BitSlice;

    fn index(&self, row: usize) -> &bit_matrix::row::BitSlice {
        &self.0[row]
    }
}
impl std::ops::Index<(usize, usize)> for AdjecencyMatrix {
    type Output = bool;

    fn index(&self, row: (usize, usize)) -> &bool {
        &self.0[row]
    }
}

impl DirectedGraph<usize> for AdjecencyMatrix {
    fn contains_vertex(&self, vertex: &usize) -> bool {
        self.size() > *vertex
    }

    fn is_adjacent(&self, vertex: &usize, vertex2: &usize) -> bool {
        if self.size() > *vertex && self.size() > *vertex2 {
            self[(*vertex, *vertex2)]
        } else {
            false
        }
    }

    fn get_predecessors(&self, vertex: &usize) -> Option<Box<[usize]>> {
        if self.size() > *vertex {
            let a: Box<[usize]> = (0..self.size())
                .map(|i| (i, self[(*vertex, 0)]))
                .filter_map(|x| if x.1 { Some(x.0) } else { None })
                .collect();
            Some(a)
        } else {
            None
        }
    }

    fn get_children(&self, vertex: &usize) -> Option<Box<[&usize]>> {
        if self.size() > *vertex {
            let a: Box<[&usize]> = self[*vertex]
                .iter_bits(self.size())
                .zip(self.1.iter())
                .filter_map(|x| if x.0 { Some(x.1) } else { None })
                .collect();
            Some(a)
        } else {
            None
        }
    }

    fn get_predecessor_count(&self, vertex: &usize) -> Option<usize> {
        if self.size() > *vertex {
            Some(
                (0..self.size())
                    .map(|i| (i, self[(*vertex, 0)]))
                    .filter(|(_, b)| *b)
                    .count(),
            )
        } else {
            None
        }
    }

    fn get_children_count(&self, vertex: &usize) -> Option<usize> {
        if self.size() > *vertex {
            Some(
                self[*vertex]
                    .iter_bits(self.size())
                    .enumerate()
                    .filter(|(_, b)| *b)
                    .count(),
            )
        } else {
            None
        }
    }

    fn get_vertices(&self) -> Box<[usize]> {
        (0..self.size()).collect()
    }

    fn get_edges(&self) -> Box<[(usize, usize)]> {
        (0..self.size())
            .flat_map(|i| {
                (0..self.size()).filter_map(move |j| if self[(i, j)] { Some((i, j)) } else { None })
            })
            .collect()
    }

    fn get_vertex_len(&self) -> usize {
        self.size()
    }
}

impl From<AdjecencyList> for AdjecencyMatrix {
    fn from(value: AdjecencyList) -> Self {
        let mut ret = AdjecencyMatrix(
            BitMatrix::new(value.len(), value.len()),
            (0..value.len()).collect(),
        );
        value
            .get_edges()
            .iter()
            .for_each(|(v1, v2)| ret.add_edge_between(v1, v2));
        ret
    }
}

impl From<AdjecencyMatrix> for AdjecencyList {
    fn from(value: AdjecencyMatrix) -> Self {
        let mut ret = AdjecencyList::default();
        let s = value.size();
        (0..s).for_each(|_| {
            ret.get_new_vertex();
        });
        value
            .get_edges()
            .iter()
            .for_each(|(v1, v2)| ret.add_edge_between(v1, v2));
        ret
    }
}
