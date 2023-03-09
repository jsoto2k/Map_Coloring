// use std::io;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::time::Instant;

#[derive(Clone, Debug)]
struct Territory {
    name: String,
    neighbors: HashSet<String>,
    color: Option<String>,
}

fn read_input_file(input_file: &str) -> Vec<Territory> {
    //Parse the file
    let file = fs::read_to_string(input_file).expect("Unable to read file");
    let mut lines = file.lines();
    let headers = lines.next().unwrap().split(',').skip(1);
    let mut territories: Vec<Territory> = headers
        .map(|h| Territory {
            name: h.to_owned(),
            neighbors: HashSet::new(),
            color: None,
        })
        .collect();

    for (_i, line) in lines.enumerate() {
        let territory_info: Vec<&str> = line.split(',').collect();
        let territory_name = territory_info[0];
        let neighbors = territory_info
            .iter()
            .skip(1)
            .enumerate()
            .filter(|(_, &val)| val == "1")
            .map(|(j, _)| territories.get(j).unwrap().name.clone())
            .collect::<HashSet<_>>();
        let territory_index = territories
            .iter()
            .position(|s| s.name == territory_name)
            .unwrap();
        territories[territory_index].neighbors = neighbors;
    }

    territories
}

fn main() {
    // let id: usize = 0;
    // let territory_vector: Vec<usize> = Vec::new();
    let time = Instant::now(); 
    let domain = vec!["Yellow", "Blue", "Green", "Red"];
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let territory_vector = read_input_file(filename);

    let mut territory_stack = Vec::new();
    territory_stack.push(territory_vector);
    while let Some(evaluation_vector) = territory_stack.pop() {
        if let Some(index) = evaluation_vector.iter().position(|s| s.color.is_none()) {
            let neighbor_colors = &evaluation_vector[index]
                .neighbors
                .iter()
                .map(|x| evaluation_vector.iter().find(|y| &y.name == x).unwrap())
                .filter(|x| x.color.is_some())
                .map(|x| x.color.as_ref().unwrap().to_string())
                .collect::<Vec<String>>();
            for color in domain
                .iter()
                .filter(|x| !neighbor_colors.contains(&x.to_string()))
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
            {
                let mut color_state = evaluation_vector.clone();
                color_state[index].color = Some(color);
                territory_stack.push(color_state);
            }
        } else {
            // println!("Found a solution: {:?}", evaluation_vector);

            for territory in evaluation_vector {
                println!("{}: {}", territory.name, territory.color.unwrap());
               
                
            }

            let time_string = time.elapsed().as_micros().to_string(); 
            println!("Time passed in microseconds: {}", time_string); 
            break;
        }
    }
}
