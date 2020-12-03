use crate::days::Challenge;
use std::{fs, path::Path};

type Gradient = (usize, usize);

#[derive(Debug)]
enum MapElement {
    Clear,
    Tree,
}

pub struct Day3<'a> {
    pub(crate) data: Option<SledMap>,
    pub(crate) file_path: &'a Path,
    pub(crate) part_1_ans: Option<usize>,
    pub(crate) part_2_ans: Option<i64>,
}

impl<'a> Day3<'_> {
    pub fn build(file_path: &'a Path) -> Result<Day3, ()> {
        Ok(Day3 {
            data: None,
            part_1_ans: None,
            part_2_ans: None,
            file_path: file_path
        })
    }
}

#[derive(Debug)]
pub struct SledMap {
    size: (usize, usize),
    elements: Vec<Vec<MapElement>>,
}

impl MapElement {
    fn from_char(s: &char) -> std::result::Result<Self, String> {
        match s {
            '#' => Ok(MapElement::Tree),
            '.' => Ok(MapElement::Clear),
            x => Err(format!("Unsupported map element found: {}", x)),
        }
    }

    fn is_tree(&self) -> bool {
        match self {
            MapElement::Tree => true,
            _ => false,
        }
    }
}

impl SledMap {
    fn build(input: &Vec<&str>, repeat_x: usize) -> Result<SledMap, ()> {
        let dims = (input.len(), input[0].len());
        let data: Vec<Vec<MapElement>> = input
            .iter()
            .cloned()
            .map(|s| {
                s.chars()
                    .map(|c| MapElement::from_char(&c).unwrap())
                    .cycle()
                    .take(dims.0 * repeat_x) // 3x wider than it is tall
                    .collect()
            })
            .collect();

        Ok(SledMap {
            size: dims,
            elements: data,
        })
    }

    fn get_coord(&self, x: usize, y: usize) -> Option<&MapElement> {
        self.elements.iter().nth(y).and_then(|r| r.iter().nth(x))
    }
}

fn count_trees(sled_map: &SledMap, (x, y): Gradient) -> usize {
    (0..(sled_map.size.0 / y))
        .map(|i| sled_map.get_coord(i * x, i * y).unwrap())
        .filter(|e| e.is_tree())
        .count()
}

impl Challenge<'_> for Day3<'_> {
    fn setup(&mut self) {
        let raw_data = fs::read_to_string(self.file_path).unwrap();
        let lines: Vec<&str> = raw_data.lines().collect();

        self.data = SledMap::build(&lines, 7).map(|m| Some(m)).unwrap();
    }

    fn part_1(&mut self) {
        self.part_1_ans = Some(count_trees(self.data.as_ref().unwrap(), (3, 1)));
    }

    fn part_2(&mut self) {
        let gradients: Vec<Gradient> = vec![(1, 1), (5, 1), (7, 1), (1, 2)];

        self.part_2_ans = Some(
            gradients
                .iter()
                .map(|(x, y)| count_trees(self.data.as_ref().unwrap(), (*x, *y)))
                .fold(1, |x, y| x * y as i64)
                * self.part_1_ans.unwrap() as i64,
        );
    }

    fn format_answers(&self) -> String {
        format!(
            "Part 1: {}\nPart 2: {}",
            self.part_1_ans.unwrap(),
            self.part_2_ans.unwrap_or(-1) // self.part_2_ans.unwrap()
        )
    }
}
