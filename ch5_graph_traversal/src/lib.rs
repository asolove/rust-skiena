#![feature(test)]

extern crate rand;
extern crate test;

use rand::Rng;
use std::collections::VecDeque;
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
      edges: vec![vec![]; vertex_count],
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


fn random_sparse_graph(vertices: usize, edges: usize) -> AdjacencyList {
  let mut g = AdjacencyList::new(false, vertices);
  let range = Range::new(0, vertices-1);
  let mut rng = rand::thread_rng();

  for _ in 0..edges {
    g.add_edge(range.ind_sample(&mut rng), range.ind_sample(&mut rng));
  }
  g
}

fn make_random_sparse_graph(edges: usize) -> () {
  random_sparse_graph(100000, edges);
}

#[bench]
fn bench_init_small_graoh(b: &mut Bencher) {
  b.iter(|| AdjacencyList::new(false, 100))
}

#[bench]
fn bench_init_medium_graoh(b: &mut Bencher) {
  b.iter(|| AdjacencyList::new(false, 1000))
}

#[bench]
fn bench_init_large_graoh(b: &mut Bencher) {
  b.iter(|| AdjacencyList::new(false, 10000))
}

#[bench]
fn bench_add_edges_small(b: &mut Bencher) {
  b.iter(|| make_random_sparse_graph(10));
}
#[bench]
fn bench_add_edges_medium(b: &mut Bencher) {
  b.iter(|| make_random_sparse_graph(100));
}
#[bench]
fn bench_add_edges_large(b: &mut Bencher) {
  b.iter(|| make_random_sparse_graph(1000));
}

impl AdjacencyList {
  fn bfs(&self, root: usize) -> Vec<Option<usize>> {
    if root >= self.vertex_count || root < 0 {
      panic!("Root {} out of bounds", root);
    }

    let mut parents = vec![None; self.vertex_count];
    parents[root] = Some(root);

    let mut queue = VecDeque::new();
    queue.push_back(root);

    while let Some(node) = queue.pop_front() {
      for n in self.edges[node].iter().cloned() {
        if parents[n].is_none() {
          parents[n] = Some(node);
          queue.push_back(n.clone());
        }
      }
    }

    parents
  }
}

#[test]
fn test_bfs() {
  let mut g = AdjacencyList::new(false, 7);
  g.add_edge(0, 1);
  g.add_edge(1, 2);
  g.add_edge(2, 3);
  g.add_edge(1, 6);
  g.add_edge(3, 4);
  g.add_edge(4, 5);
  g.add_edge(2, 5);
  g.add_edge(5, 1);

  assert_eq!(
    vec![Some(1), Some(1), Some(1), Some(2), Some(5), Some(1), Some(1)],
    g.bfs(1));
}

#[bench]
fn bench_bfs_10(b: &mut Bencher) {
  let g = random_sparse_graph(1000, 10);
  b.iter(|| g.bfs(0) );
}

#[bench]
fn bench_bfs_1000(b: &mut Bencher) {
  let g = random_sparse_graph(1000, 1000);
  b.iter(|| g.bfs(0) );
}

#[bench]
fn bench_bfs_100000(b: &mut Bencher) {
  let g = random_sparse_graph(1000, 100000);
  b.iter(|| g.bfs(0) );
}