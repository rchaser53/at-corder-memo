#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use itertools::Itertools;
use maplit::{btreemap, btreeset, hashmap, hashset};
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, DiGraph, UnGraph};
use permutohedron::{Heap, heap_recursive};
use std::collections::*;
use superslice::Ext;

const MOD:usize = 1_000_000_007;
const MAX: usize = 1000;

fn main() {
  input!{
    n:usize,
  }
  
  let mut x = 1000;
  let mut count = 0;
  while x <= n {
    count += n - x + 1;
    x *= 1000;
  }
  println!("{}", count);
}