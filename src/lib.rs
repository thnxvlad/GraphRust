use std::collections::{HashSet, HashMap};
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[cfg(test)]
mod tests {
    use crate::Graph;

    #[test]
    fn test1() {
        let mut g: Graph<String, String> = Graph::read_from_file(String::from("tests/test1.tgf"));
        assert_eq!(g.serialize_to_tgf(),
                   "1 ROOT\n2 2\n3 3\n4 4\n5 5\n#\n1 2 \n1 3 \n2 4 \n2 5 \n");
    }
    #[test]
    fn test2() {
        let mut g: Graph<String, String> = Graph::read_from_file(String::from("tests/test2.tgf"));
        assert_eq!(g.serialize_to_tgf(),
                   "1 ROOT\n2 2\n3 3\n4 4\n5 5\n#\n");
    }
    #[test]
    fn test3() {
        let mut g: Graph<String, String> = Graph::read_from_file(String::from("tests/test3.tgf"));
        assert_eq!(g.serialize_to_tgf(),
                   "1 1\n2 2\n3 3\n4 \n5 5\n#\n2 1 edge\n2 3 tap\n2 3 tap\n2 4 \n3 5 \n4 3 32432\n4 5 \n5 1 \n5 3 ewrer\n");
    }
    #[test]
    fn test4() {
        let mut g: Graph<String, String> = Graph::read_from_file(String::from("tests/test4.tgf"));
        g.add_edge(&2, &5, String::from("111"));
        assert_eq!(g.serialize_to_tgf(),
                   "1 ROOT\n2 2\n3 3\n4 4\n5 5\n#\n1 2 \n1 3 \n2 5 111\n2 5 111\n3 4 \n");
    }
    #[test]
    fn test5() {
        let mut g: Graph<String, String> = Graph::read_from_file(String::from("tests/test5.tgf"));
        g.add_vertex(String::from("f"));
        assert_eq!(g.serialize_to_tgf(),
                   "1 f\n2 f\n3 f\n4 f\n5 f\n6 f\n#\n2 1 \n2 3 \n2 5 \n3 1 \n3 4 \n5 1 \n5 3 \n");
    }
    #[test]
    fn test6() {
        let mut g: Graph<String, String> = Graph::read_from_file(String::from("tests/test6.tgf"));
        g.remove_vertex(&3);
        g.remove_edge(&2, &5, String::new());
        g.remove_edge(&5, &2, String::from("hhhhhh"));
        g.remove_vertex(&4);
        g.add_vertex("646".to_string());
        assert_eq!(g.serialize_to_tgf(),
                   "1 0\n2 1\n3 4\n4 646\n#\n2 1 7777\n3 1 555\n");
    }
    #[test]
    fn test7() {
        let mut g: Graph<String, String> = Graph::read_from_file(String::from("tests/test7.tgf"));
        assert_eq!(g.serialize_to_tgf(), "#\n");
    }
    #[test]
    fn test8() {
        let mut g: Graph<String, String> = Graph::read_from_file(String::from("tests/test8.tgf"));
        assert_eq!(g.serialize_to_tgf(), "#\n");
    }
}

pub struct Graph<V, E> {
    // HashMap<vertex_id, (vertex_label, HashMap<vertex2_id, edges_labels>)>
    data: HashMap<usize, (Option<V>, HashMap<usize, Vec<Option<E>>>)>,
    // HashMap<vertex2_id, HashSet<vertex_id>>
    // - saves some info about edges to current vertex for providing best asymptotics of methods
    reversed_edges: HashMap<usize, HashSet<usize>>,
    last_vertex_id: usize
}

impl<V, E> Graph<V, E> {
    pub fn new() -> Self {
        Graph {
            data: HashMap::new(),
            reversed_edges: HashMap::new(),
            last_vertex_id: 0
        }
    }
    // returns id of new vertex
    pub fn add_vertex(&mut self, label: V) -> usize {
        self.last_vertex_id = self.last_vertex_id + 1;
        self.data.insert(self.last_vertex_id, (Some(label), HashMap::new()));
        self.reversed_edges.insert(self.last_vertex_id, HashSet::new());
        self.last_vertex_id
    }

    pub fn remove_vertex(&mut self, id: &usize) {
        match self.data.get(id) {
            Some((_, edges)) =>
                for (id2, _) in edges.iter() {
                    self.reversed_edges.get_mut(id2).map(|r_edges| r_edges.remove(id));
                },
            None => {
                println!("\nThere is no vertex with id {}", id);
                return;
            }
        };
        self.data.remove(id);

        self.reversed_edges.get(id).map(|r_edges| {
            for id2 in r_edges.iter() {
                self.data.get_mut(id2)
                    .map(|(_, edges)| {
                        edges.remove(id);
                    });
            };
        });
        self.reversed_edges.remove(id);
    }

    pub fn add_edge(&mut self, from_id: &usize, to_id: &usize, label: E) where E: Clone {
        match self.data.get_mut(from_id) {
            Some((_, edges)) =>
                match self.reversed_edges.get_mut(to_id) {
                    Some(r_edges) => {
                        match edges.get_mut(to_id) {
                            Some(e_labels_o_vec) => e_labels_o_vec.push(Some(label)),
                            None => {
                                edges.insert(*to_id, [Some(label)].to_vec());
                                r_edges.insert(*from_id);
                            }
                        }
                    },
                    None => println!("\nThere is no vertex with id {}", to_id)
                },
            None => println!("\nThere is no vertex with id {}", from_id)
        };
    }

    pub fn remove_edge(&mut self, from_id: &usize, to_id: &usize, label: E) where E: std::cmp::PartialEq {
        match self.data.get_mut(from_id) {
            Some((_, edges)) => {
                match self.reversed_edges.get_mut(to_id) {
                    Some(r_edges) =>
                        match edges.get_mut(to_id) {
                            Some(e_labels_o_vec) => {
                                let index = e_labels_o_vec.iter().position(|r| r.as_ref() == Some(&label)).unwrap_or(e_labels_o_vec.len());
                                if index == e_labels_o_vec.len() {
                                    println!("\nThere is no edge between vertices {} and {} with this label", from_id, to_id);
                                } else {
                                    e_labels_o_vec.swap_remove(index);
                                    if e_labels_o_vec.is_empty() {
                                        r_edges.remove(from_id);
                                        edges.remove(to_id);
                                    }
                                }
                            },
                            None =>  println!("\nThere are no edges between vertices {} and {}", from_id, to_id)
                        }
                    None => println!("\nThere is no vertex with id {}", to_id)
                }
            },
            None => println!("\nThere is no vertex with id {}", from_id)
        };
    }

    fn get_new_ids_after_fit(&self) -> HashMap<usize, usize> {
        let mut new_ids_vec: Vec<usize> = Vec::with_capacity(self.data.len());
        for (old_id, _) in self.data.iter() {
            new_ids_vec.push(*old_id);
        }
        new_ids_vec.sort();

        let mut new_id: usize = 0;
        new_ids_vec.into_iter().map(|old_id| {
            new_id += 1;
            (old_id, new_id)
        }).collect()
    }
    // after remove_vertex() we should to fit ids of other vertexes if it is possible to avoid id value overflow
    pub fn fit(&mut self) where V: Clone, E: Clone {
        let new_ids: HashMap<usize, usize> = self.get_new_ids_after_fit();
        self.reversed_edges =
            self.reversed_edges.iter().map(|(id2, ids)| {
                (new_ids[id2], ids.iter().map(|id| {
                    new_ids[id]
                }).collect())
            }).collect();

        self.data =
            self.data.iter()
                .map(|(id, (v_label_o, edges))| (new_ids[id], (v_label_o.clone(), edges.iter()
                    .map(|(id2, e_label_o_vec)| (new_ids[id2], e_label_o_vec.clone()))
                    .collect())))
                .collect();

        self.last_vertex_id = self.data.len();
    }

    pub fn get_vertices_vec(&self) -> Vec<(&usize, &Option<V>)>{
        self.data.iter()
            .map(|(id, (v_label_o, _))| (id, v_label_o))
            .collect()
    }

    pub fn get_edges_vec(&self) -> Vec<(&usize, &usize, &Vec<Option<E>>)> {
        self.data.iter()
            .map(|(id, (_, edges))| {
                edges.iter()
                    .map(|(id2, e_label_o_vec)| {
                        (id, id2, e_label_o_vec)
                    })
                    .collect::<Vec<(&usize, &usize, &Vec<Option<E>>)>>()
            })
            .flatten()
            .collect::<Vec<(&usize, &usize, &Vec<Option<E>>)>>()
    }
}

impl Graph <String, String> {
    fn deserialize_from_tgf(contents: String) -> Self {
        let vertices_and_edges: Vec<Vec<String>> = contents.as_str().split("#")
            .map(|records| records.lines()
                .map(|record| record.to_string())
                .collect())
            .collect();
        let mut vertices: Vec<Vec<String>> = vertices_and_edges[0].iter()
            .map(|v_info| v_info.splitn(2," ").filter(|&v_info| v_info != "")
                .map(|id_or_label| id_or_label.trim().to_string())
                .collect())
            .collect();
        vertices.retain(|v| !v.is_empty());
        let mut edges: Vec<Vec<String>> = vertices_and_edges[1].iter()
            .map(|e_info| e_info.splitn(3," ").filter(|&e_info| e_info != "")
                .map(|id_or_label| id_or_label.trim().to_string())
                .collect())
            .collect();
        edges.retain(|e| !e.is_empty());

        let mut data: HashMap<usize, (Option<String>, HashMap<usize, Vec<Option<String>>>)> =
            vertices.iter().map(|v_info: &Vec<String>| {
                let label_o: Option<String> = match v_info.get(1) {
                    Some(label) => Some(label.to_owned()),
                    None => None
                };
                (v_info[0].parse::<usize>().unwrap(), (label_o, HashMap::new()))
            }).collect();

        let mut reversed_edges: HashMap<usize, HashSet<usize>> =
            vertices.iter()
                .map(|v_info| (v_info[0].parse::<usize>().unwrap(), HashSet::new()))
                .collect();

        edges.iter().for_each(|edge_info:&Vec<String>|{
            let id1: usize = edge_info[0].to_string().parse::<usize>().unwrap();
            let id2: usize = edge_info[1].to_string().parse::<usize>().unwrap();
            let label_o: Option<String> = match edge_info.get(2) {
                Some(label) => Some(label.to_owned()),
                None => Some(String::new())
            };
            data.get_mut(&id1).map(|(_, edges)| {
                if !edges.contains_key(&id2) {
                    edges.insert(id2, Vec::new());
                }
                edges.get_mut(&id2)
                    .map(|e_label_o_vec|
                        e_label_o_vec.push(label_o)
                    );
            });
            reversed_edges.get_mut(&id2).map(|ids| ids.insert(id1));
        });

        let last_vertex_id: usize = data.len();

        Graph {
            data,
            reversed_edges,
            last_vertex_id
        }
    }

    pub fn read_from_file(file_path: String) -> Self {
        let path = Path::new(file_path.as_str());
        let display = path.display();

        let contents  = match std::fs::read_to_string(path) {
            Err(why) => panic!("unable to read from {}: {}", display, why),
            Ok(contents) => contents
        };

        Self::deserialize_from_tgf(contents)
    }

    pub fn serialize_to_tgf(&mut self) -> String {
        self.fit();
        let mut vertices: Vec<(&usize, &Option<String>)> = self.get_vertices_vec();
        vertices.sort();

        let mut edges: Vec<(&usize, &usize, &Vec<Option<String>>)> = self.get_edges_vec();
        edges.sort();

        vertices.into_iter()
            .map(|(id, v_label_o)|
                [
                    id.to_string(),
                    match v_label_o {
                        Some(v_label) => v_label.clone(),
                        None => String::new()
                    }
                ].join(" ") + "\n"
            )
            .collect::<String>()
                + "#\n"
                    + edges.into_iter()
                        .map(|(id, id2, e_label_o_vec)|
                            e_label_o_vec.iter().map(|e_label_o|
                                [
                                    id.to_string(),
                                    id2.to_string(),
                                    match e_label_o {
                                        Some(e_label) => e_label.clone(),
                                        None => String::new()
                                    }
                                ].join(" ") + "\n"
                            ).collect::<String>()
                        )
                        .collect::<String>()
                        .as_str()
    }

    pub fn write_to_file(&mut self, file_path: String) {
        let path = Path::new(file_path.as_str());
        let display = path.display();

        let mut file = match File::create(&path) {
            Err(why) => panic!("unable to create {}: {}", display, why),
            Ok(file) => file,
        };

        match file.write_all(self.serialize_to_tgf().as_bytes()) {
            Err(why) => panic!("unable to write to {}: {}", display, why),
            Ok(_) => println!("successfully written to {}", display),
        }
    }

    fn display_vertex_info(graph: &Graph<String, String>, id: &usize) {
        println!(
            "id: {}, label: {}\nadjacent vertices: {:?}\n",
            id,
            match graph.data.get(id) {
                Some((v_label_o, _)) =>
                    match v_label_o {
                        Some(v_label) => v_label.clone(),
                        None => String::new()
                    },
                None => String::new()
            },
            match graph.data.get(id) {
                Some((_, edges)) => {
                    edges.iter()
                        .map(|(id2, e_labels_o_vec)| (id2, e_labels_o_vec.len()))
                        .collect()
                },
                None => [].to_vec()
            }
        );
    }

    // depth-first search
    fn dfs (&self, id: &usize, used: &mut HashMap<&usize, bool>, func: fn(&Graph<String, String>, &usize)) {
        used.get_mut(id).map(|is_used| *is_used = true);
        func(&self, id);
        for (_, edges) in self.data.get(id) {
            for id2 in edges.keys() {
                match used[id2] {
                    true => continue,
                    false => self.dfs(id2, used, func)
                };
            }
        }
    }
    // displaying info about every vertex in format:
    // id, label, [(second vertex id of edge, amount of multiple edges),...]
    pub fn display (&self) {
        let mut used: HashMap<&usize, bool>  = self.data.iter()
            .map(|(id, _)| {
                (id, false)
            }).collect();
        println!("\n###DISPLAY###\n");
        for id in self.data.keys() {
            match used[id] {
                true => continue,
                false => self.dfs(id, &mut used, Graph::<String, String>::display_vertex_info)
            };
        };
        print!("###THE END###\n");
    }
}