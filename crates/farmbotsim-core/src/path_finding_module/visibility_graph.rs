use egui::Pos2;
use petgraph::{
    algo::astar,
    graph::{NodeIndex, UnGraph},
    visit::EdgeRef,
};

use crate::{environment::obstacle::Obstacle, path_finding_module::path_finding::PathFinding};

/// A graph-based pathfinding structure using a visibility graph approach.
#[derive(Clone, Debug)]
pub struct VisibilityGraph {
    pub graph: UnGraph<Pos2, ()>,
    obstacles: Vec<Obstacle>,
}

impl PathFinding for VisibilityGraph {
    fn find_path(&mut self, start: Pos2, end: Pos2) -> Option<Vec<Pos2>> {
        let mut added_nodes: Vec<_> = Vec::new();
        // Check if start/end are already in graph
        let start_node = match self.find_existing_node(start) {
            Some(node) => node,
            None => {
                let node = self.add_node_with_connections(start);
                added_nodes.push(node);
                node
            }
        };

        let end_node = match self.find_existing_node(end) {
            Some(node) => node,
            None => {
                let node = self.add_node_with_connections(end);
                added_nodes.push(node);
                node
            }
        };

        // Run A* algorithm
        let result = astar(
            &self.graph,
            start_node,
            |n| n == end_node,
            |e| {
                // Safely handle Option from edge_endpoints
                self.graph
                    .edge_endpoints(e.id())
                    .map_or(f32::INFINITY, |(a, b)| {
                        self.graph[a].distance(self.graph[b])
                    })
            },
            |n| self.graph[n].distance(end),
        )
        .map(|(_, path)| path.into_iter().map(|n| self.graph[n]).collect());

        // Cleanup any temporary nodes added
        for node in added_nodes.iter().rev() {
            self.graph.remove_node(*node);
        }
        result
    }
}

impl VisibilityGraph {
    /// Creates a new `VisibilityGraph` from given points and obstacles.
    pub fn new(points: &[Pos2], obstacles: Vec<Obstacle>) -> Self {
        Self {
            graph: Self::build_graph(points, &obstacles),
            obstacles,
        }
    }

    /// Recalculates the graph with new points and obstacles, rebuilding the visibility edges.
    pub fn recalculate(&mut self, points: &[Pos2], obstacles: &[Obstacle]) {
        self.obstacles = obstacles.to_vec();
        self.graph = Self::build_graph(points, obstacles);
    }

    /// Builds the visibility graph edges between points, excluding edges intersecting obstacles.
    fn build_graph(points: &[Pos2], obstacles: &[Obstacle]) -> UnGraph<Pos2, ()> {
        let mut graph = UnGraph::<Pos2, ()>::new_undirected();

        let mut unique_points = Vec::new();
        for p in points {
            if !unique_points
                .iter()
                .any(|u: &Pos2| (u.x - p.x).abs() < 0.001 && (u.y - p.y).abs() < 0.001)
            {
                unique_points.push(*p);
            }
        }

        // Add all points as nodes to the graph
        let node_indices: Vec<NodeIndex> = unique_points
            .iter()
            .map(|&point| graph.add_node(point))
            .collect();

        // Check all possible edges between points
        for (i, &point1) in unique_points.iter().enumerate() {
            for (j, &point2) in unique_points.iter().enumerate().skip(i + 1) {
                let edge_line = (point1, point2);

                // Check if this edge intersects any obstacle
                let intersects_obstacle = obstacles
                    .iter()
                    .any(|obstacle| Self::lines_intersect(edge_line, obstacle));

                // If no intersection, add the edge
                if !intersects_obstacle {
                    graph.add_edge(node_indices[i], node_indices[j], ());
                }
            }
        }

        graph
    }

    /// Check if a line segment intersects with any segment of an obstacle
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

    /// Determines if two line segments intersect using cross product test.
    fn line_segments_intersect(a1: Pos2, a2: Pos2, b1: Pos2, b2: Pos2) -> bool {
        let ccw = |a: Pos2, b: Pos2, c: Pos2| (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x);

        let d1 = ccw(a1, a2, b1);
        let d2 = ccw(a1, a2, b2);
        let d3 = ccw(b1, b2, a1);
        let d4 = ccw(b1, b2, a2);

        // Check if the segments straddle each other
        ((d1 * d2) < 0.0) && ((d3 * d4) < 0.0)
    }

    /// Finds a node in the graph that exactly matches the given position, if any.
    fn find_existing_node(&self, pos: Pos2) -> Option<NodeIndex> {
        self.graph.node_indices().find(|&n| self.graph[n] == pos)
    }

    /// Adds a new node with position `pos` and connects it to all visible existing nodes.
    fn add_node_with_connections(&mut self, pos: Pos2) -> NodeIndex {
        let new_node = self.graph.add_node(pos);

        // Connect to all other nodes if line doesn't intersect obstacles
        for existing_node in self.graph.node_indices() {
            if existing_node != new_node {
                let existing_pos = self.graph[existing_node];
                let edge_line = (pos, existing_pos);
                let intersects_obstacle = self
                    .obstacles
                    .iter()
                    .any(|obstacle| Self::lines_intersect(edge_line, obstacle));
                if !intersects_obstacle {
                    self.graph.add_edge(new_node, existing_node, ());
                }
            }
        }

        new_node
    }
}
