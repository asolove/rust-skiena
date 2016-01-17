#![feature(test)]

extern crate rand;
extern crate test;

use rand::Rng;
use rand::distributions::{IndependentSample, Range};
use test::Bencher;

#[test]
fn test_init_graph() {
  let mut g = AdjacencyList::new(true, 100);
  g.add_edge(1, 2);
  g.add_edge(2, 3);
  assert_eq!(2, g.edge_count());

  let mut g = AdjacencyList::new(false, 100);
  g.add_edge(1, 2);
  g.add_edge(2, 3);
  assert_eq!(4, g.edge_count());
}

pub trait Graph {
  fn edge_count(&self) -> usize;
  fn add_edge(&mut self, usize, usize);
}

pub struct AdjacencyList {
  directed: bool,
  vertex_count: usize,
  edge_count: usize,
  edges: Vec<Vec<usize>>,
  degree: Vec<usize>,
}

impl AdjacencyList {
  fn new(directed: bool, vertex_count: usize) -> AdjacencyList {
    AdjacencyList {
      directed: directed,
      vertex_count: vertex_count,
      edge_count: 0,
      edges: vec![vec!(); vertex_count],
      degree: vec![0; vertex_count]
    }
  }
}

impl Graph for AdjacencyList {
  fn edge_count(&self) -> usize {
    self.edge_count
  }

  // NB: doesn't check for duplicate, self edges.
  fn add_edge(&mut self, source: usize, destination: usize) {
    self.edges[source].push(destination);
    self.edge_count += 1;
    self.degree[source] += 1;

    if !self.directed {
      self.edges[destination].push(source);
      self.edge_count += 1;
      self.degree[destination] += 1;
    }
  }
}


fn random_sparse_graph(edges: usize) -> AdjacencyList {
  let mut g = AdjacencyList::new(false, edges);
  let range = Range::new(0, edges-1);
  let mut rng = rand::thread_rng();

  for _ in 0..edges {
    g.add_edge(range.ind_sample(&mut rng), range.ind_sample(&mut rng));
  }
  g
}

fn make_random_sparse_graph(edges: usize) -> () {
  random_sparse_graph(edges);
}

#[bench]
fn bench_small_graph(b: &mut Bencher) {
  b.iter(|| make_random_sparse_graph(10));
}
#[bench]
fn bench_medium_graph(b: &mut Bencher) {
  b.iter(|| make_random_sparse_graph(100));
}
#[bench]
fn bench_big_graph(b: &mut Bencher) {
  b.iter(|| make_random_sparse_graph(1000));
}