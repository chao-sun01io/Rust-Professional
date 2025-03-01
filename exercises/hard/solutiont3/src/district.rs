use serde::Deserialize;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::{fs, result};
#[derive(Deserialize, Debug)]
pub struct DistrictData(HashMap<String, HashMap<String, Vec<String>>>);

pub fn count_provinces() -> String {
    let graphs = process_json();

    let mut result = Vec::new();
    for graph in graphs {
        let cnt = classify_by_connection(&graph);
        // println!("key {} cnt {}", k, cnt);
        result.push(cnt.to_string());
    }

    let result_str = result.join(",");
    println!("{:?}", result_str);

    result_str
}

fn process_json() -> Vec<HashMap<String, Vec<String>>> {
    let contents = fs::read_to_string("district.json").expect("Unable to read file");
    let json: Value = serde_json::from_str(&contents).expect("Parse Json error");
    let mut results = Vec::new();
    if let Value::Object(districts) = json {
        // Get sorted district keys
        let mut keys: Vec<&String> = districts.keys().collect();
        keys.sort();

        for key in keys {
            // println!("read {}", key);

            if let Some(Value::Object(data_set)) = districts.get(key) {
                let mut graph: HashMap<String, Vec<String>> = HashMap::new();

                for (vertex, neighbors) in data_set {
                    graph.entry(vertex.to_string()).or_insert(Vec::new());
                    // println!("vertex {}", vertex);
                    if let Value::Array(neighbor_vertices) = neighbors {
                        for neighbor in neighbor_vertices {
                            let v = neighbor.as_str().unwrap().to_string();
                            // if !graph.get(vertex).unwrap().contains(&v) {
                                graph.get_mut(vertex).unwrap().push(v.clone());
                            // }

                            graph.entry(v.clone()).or_insert(Vec::new());
                            if !graph.get(&v).unwrap().contains(&vertex) {
                                graph.get_mut(&v).unwrap().push(vertex.clone());
                            }
                        }
                    }
                }
                // println!("graph {:?}",graph);

                results.push(graph);
            }
        }
    }

    results
}

// // The input graph is not a complete graph, here process to complete it
// fn process_graph(graph: &mut HashMap<String, Vec<String>>) {
//     let keys: Vec<String> = graph.keys().cloned().collect();
//     for k in &keys {
//         let vertices: Vec<String> = graph.get(k).unwrap().clone();
//         for vertex in vertices {
//             graph.entry(vertex.clone()).or_insert_with(Vec::new);
//         }
//     }
// }

fn classify_by_connection(data: &HashMap<String, Vec<String>>) -> u32 {
    let mut visited: HashSet<&String> = HashSet::new();
    let mut count = 0;

    for (k, _) in data.iter() {
        if !visited.contains(k) {
            count += 1;
            // DFS for each city
            dfs(k, &mut visited, data);
        }
    }

    count
}

fn dfs<'a>(
    vertex: &'a String,
    visited: &mut HashSet<&'a String>,
    data: &'a HashMap<String, Vec<String>>,
) {
    if !visited.contains(vertex) {
        visited.insert(vertex);
        if let Some(connected) = data.get(vertex) {
            for neighbor in connected {
                dfs(neighbor, visited, data);
            }
        }
    }
}
