use egui::Pos2;
use petgraph::{graph::{NodeIndex, UnGraph}, visit::EdgeRef};
use petgraph::algo::astar;

use crate::environment::obstacle::Obstacle;


#[derive(Clone)]
pub struct VisibilityGraph {
    pub graph: UnGraph<Pos2, ()>,
    obstacles: Vec<Obstacle>
}

impl VisibilityGraph {
    pub fn new(points: &[Pos2], obstacles: Vec<Obstacle>) -> Self {
        Self {
            graph: Self::build_graph(points, &obstacles),
            obstacles,
        }
    }
    
    pub fn recalculate(&mut self, points: &[Pos2], obstacles: &[Obstacle]) {
        self.graph = Self::build_graph(points, obstacles);
    }

    fn build_graph(points: &[Pos2], obstacles: &[Obstacle]) -> UnGraph<Pos2, ()> {
        let mut graph = UnGraph::<Pos2, ()>::new_undirected();
        
        // Add all points as nodes to the graph
        let node_indices: Vec<NodeIndex> = points
            .iter()
            .map(|&point| graph.add_node(point))
            .collect();
        
        // Check all possible edges between points
        for (i, &point1) in points.iter().enumerate() {
            for (j, &point2) in points.iter().enumerate().skip(i + 1) {
                let edge_line = (point1, point2);
                
                // Check if this edge intersects any obstacle
                let intersects_obstacle = obstacles.iter().any(|obstacle| {
                    Self::lines_intersect(edge_line, obstacle)
                });
                
                // If no intersection, add the edge
                if !intersects_obstacle {
                    graph.add_edge(node_indices[i], node_indices[j], ());
                }
            }
        }
        
        graph
    }
    
    /// Helper function to check if a line segment intersects with any segment of an obstacle
    fn lines_intersect(line: (Pos2, Pos2), obstacle: &Obstacle) -> bool {
        let (a1, a2) = line;
        
        // Check against all line segments in the obstacle
        for window in obstacle.points.windows(2) {
            if let [b1, b2] = window {
                if Self::line_segments_intersect(a1, a2, *b1, *b2) {
                    return true;
                }
            }
        }
        
        false
    }
    
    /// Line segment intersection test using cross products
    fn line_segments_intersect(a1: Pos2, a2: Pos2, b1: Pos2, b2: Pos2) -> bool {
        let ccw = |a: Pos2, b: Pos2, c: Pos2| {
            (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
        };
        
        let d1 = ccw(a1, a2, b1);
        let d2 = ccw(a1, a2, b2);
        let d3 = ccw(b1, b2, a1);
        let d4 = ccw(b1, b2, a2);
        
        // Check if the segments straddle each other
        ((d1 * d2) < 0.0) && ((d3 * d4) < 0.0)
    }
    
    pub fn find_path(&mut self, start: Pos2, end: Pos2) -> Option<Vec<Pos2>> {
        // Check if start/end are already in graph
        let start_node = self.find_existing_node(start).unwrap_or_else(|| {
            self.add_node_with_connections(start)
        });
        
        let end_node = self.find_existing_node(end).unwrap_or_else(|| {
            self.add_node_with_connections(end)
        });

        // Run A* algorithm
        astar(
            &self.graph,
            start_node,
            |n| n == end_node,
            |e| {
                // Safely handle Option from edge_endpoints
                self.graph.edge_endpoints(e.id()).map_or(f32::INFINITY, |(a, b)| {
                    self.graph[a].distance(self.graph[b])
                })
            },
            |n| self.graph[n].distance(end),
        )
        .map(|(_, path)| path.into_iter().map(|n| self.graph[n]).collect())
    }

    fn find_existing_node(&self, pos: Pos2) -> Option<NodeIndex> {
        self.graph.node_indices()
            .find(|&n| self.graph[n] == pos)
    }

    fn add_node_with_connections(&mut self, pos: Pos2) -> NodeIndex {
        let new_node = self.graph.add_node(pos);
        
        // Connect to all other nodes if line doesn't intersect obstacles
        for existing_node in self.graph.node_indices() {
            if existing_node != new_node {
                let existing_pos = self.graph[existing_node];
                let edge_line = (pos, existing_pos);
                let intersects_obstacle = self.obstacles.iter().any(|obstacle| {
                    Self::lines_intersect(edge_line, obstacle)
                });
                if !intersects_obstacle {
                    self.graph.add_edge(new_node, existing_node, ());
                }
            }
        }
        
        new_node
    }

}

