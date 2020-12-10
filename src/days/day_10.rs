use itertools::Itertools;
use petgraph::Directed;
use petgraph::Incoming;
use petgraph::{visit::Bfs};
use petgraph::{graph::NodeIndex, Graph};
use std::{collections::HashMap, fs};
use std::{iter, path::Path};

use crate::days::Challenge;

pub struct Day10<'a> {
    data: Vec<u32>,
    file_path: &'a Path,
    part_1_ans: Option<u32>,
    part_2_ans: Option<u128>,
}

impl<'a> Day10<'_> {
    pub fn build(file_path: &'a Path) -> Box<Day10> {
        Box::new(Day10 {
            data: Vec::new(),
            part_1_ans: None,
            part_2_ans: None,
            file_path: file_path,
        })
    }
}

impl Challenge<'_> for Day10<'_> {
    fn setup(&mut self) {
        let raw_str = fs::read_to_string(self.file_path).unwrap();

        self.data = "0"
            .to_string()
            .lines()
            .chain(raw_str.lines())
            .filter_map(|s| s.parse::<u32>().ok())
            .sorted()
            .collect();

        self.data.push(self.data.last().unwrap() + 3);
    }

    fn part_1(&mut self) {
        let count_map: HashMap<u32, u32> = HashMap::new();

        self.part_1_ans = Some(
            self.data
                .iter()
                .tuple_windows::<(&u32, &u32)>()
                .fold(count_map, |mut acc, (f, l)| {
                    let counter = acc.entry(l - f).or_insert(0);
                    *counter += 1;

                    acc
                })
                .values()
                .fold(1, |acc, v| acc * v),
        );
    }

    fn part_2(&mut self) {
        let mut idx_vert_map: HashMap<u32, NodeIndex<u32>> = HashMap::new();
        let mut dag: Graph<String, String, Directed> =
            Graph::with_capacity(self.data.len(), self.data.len() * 3);
        for n in self.data.iter().rev() {
            let idx = dag.add_node(n.to_string());
            idx_vert_map.insert(*n, idx);
        }

        let mut path_counts: HashMap<NodeIndex, u128> = idx_vert_map
            .iter()
            .map(|(_, v)| (v.to_owned(), 1))
            .collect();
        let src = idx_vert_map[self.data.last().unwrap()];
        let snk = idx_vert_map[self.data.first().unwrap()];

        let edges: Vec<(NodeIndex<u32>, NodeIndex<u32>, String)> = self
            .data
            .iter()
            .rev()
            .flat_map(|&n| {
                let valid_parents: Vec<u32> = self
                    .data
                    .iter()
                    .rev()
                    .filter(|&&v| v < n && v as i32 >= n as i32 - 3)
                    .cloned()
                    .collect();

                iter::repeat(n)
                    .zip(valid_parents)
                    .collect::<Vec<(u32, u32)>>()
            })
            .sorted()
            .map(|(s, e)| (idx_vert_map[&s], idx_vert_map[&e], format!("{}-{}", s, e)))
            .collect();
        dag.extend_with_edges(edges);

        let mut bfs = Bfs::new(&dag, src);
        while let Some(nx) = bfs.next(&dag) {
            let mut neighbours = 1;
            if nx != src {
                neighbours = dag
                    .neighbors_directed(nx, Incoming)
                    .map(|f| path_counts[&f])
                    .sum();
            }
            path_counts.insert(nx, neighbours);
        }

        self.part_2_ans = Some(path_counts[&snk]);
    }

    fn format_answers(&self) -> String {
        format!(
            "Part 1: {}\nPart 2: {}",
            self.part_1_ans.unwrap_or(u32::MAX),
            self.part_2_ans.unwrap_or(u128::MAX)
        )
    }
}
