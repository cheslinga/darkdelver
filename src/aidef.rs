use crate::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum AIState {
    Idle,
    Chasing,
    Hunting
}

#[derive(Serialize, Deserialize)]
pub struct AIClass {
    pub state: AIState,
    pub target: Option<usize>,
    pub tgt_memory: u16,
    pub tgt_heatmap: HeatMap   
}
impl AIClass {
    pub fn new() -> AIClass {
        AIClass {
            state: AIState::Idle,
            target: None,
            tgt_memory: 0,
            tgt_heatmap: HeatMap::new()
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct HeatMap {
    pub lifetime: u16,
    pub nodes: Vec<Point>,
    pub old_nodes: Vec<Point>
}
impl HeatMap {
    pub fn new() -> HeatMap {
        HeatMap {
            lifetime: 0,
            nodes: Vec::new(),
            old_nodes: Vec::new()
        }
    }
    //Spread the heatmap out onto walkable tiles over time
    pub fn spread(&mut self, pos: Point, map: &Map) {
        if self.lifetime < 1 { return }

        //Gets the 3x3 area around the unit we're processing the heatmap for
        let mut ai_area: Vec<Point> = pos.get_neighbors();
        ai_area.push(pos);

        let mut new_graph: Vec<Point> = Vec::new();
        //For each already growable tile, add its neighbors to the new graph
        for p in self.nodes.iter() {
            new_graph.append(&mut p.get_neighbors());
        }

        //Deduplicate, drop unwalkable tiles, drop tiles around the enemy, and drop tiles that are already in the old graph
        new_graph.dedup();
        new_graph.retain(|p| map.walkable(p.x, p.y));
        new_graph.retain(|p| !ai_area.contains(p));
        new_graph.retain(|p| !self.old_nodes.contains(p));

        //Set the old graph to the last current graph, and dedupe. Then set the current graph to the new one we made.
        self.old_nodes.append(&mut self.nodes.to_vec());
        self.old_nodes.dedup();
        self.nodes = new_graph.to_vec();
 
        self.lifetime -= 1;
    }
    //Clear the entire graph, reset the lifetime, and add the player's position to the growable node graph
    pub fn reset_to_single_node(&mut self, pos: &Point, lifetime: u16) {
        self.nodes.clear();
        self.old_nodes.clear();
        self.lifetime = lifetime;
        self.nodes.push(*pos);
    }
    //Clears out all nodes containing points in a vector
    pub fn clear_heat_area(&mut self, points: &Vec<Point>) {
        //Make a vec of spreading tiles that the AI can already see
        let mut nodes_adjusted = self.nodes.to_vec();
        nodes_adjusted.retain(|p| !points.contains(p));

        //If there is enough left to spread off of after adjusting for vision, reset the nodes to what we just calculated
        if nodes_adjusted.len() > 3 {
            self.nodes = nodes_adjusted;
        }
        self.old_nodes.retain(|p| !points.contains(p));
    }
    //Finds the shortest path to the nearest hot node in the heatmap
    pub fn get_closest_heat(&self, map: &Map, start: Point) -> Point {
        let targets = {
            let mut targets: Vec<usize> = Vec::new();
            for n in self.nodes.iter() {
                targets.push(map.index(n.x, n.y));
            }
            for o in self.old_nodes.iter() {
                targets.push(map.index(o.x, o.y));
            }
            targets.sort();
            targets
        };
        let dijkstra_map = DijkstraMap::new(80, 80, &targets, map, 64.0);
        return if let Some(destidx) = DijkstraMap::find_lowest_exit(&dijkstra_map, map.index(start.x, start.y), map) {
            map.point_from_idx(destidx)
        } else {
            start
        }
    }
}