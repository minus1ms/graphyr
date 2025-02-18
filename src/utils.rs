use std::{cmp::Ordering, collections::BinaryHeap};

use floem::kurbo::{Point, Rect};

pub struct Segment {
    pub p1: Point,
    pub p2: Point,
}

impl Segment {
    /// Returns the intersection point between the segment and the rectangle's boundary.
    /// If the segment starts inside the rectangle, returns the exit point; otherwise, the entry point.
    /// Uses the Liang-Barsky algorithm for parametric clipping.
    pub fn intersect_rect(&self, rect: &Rect) -> Option<Point> {
        let dx = self.p2.x - self.p1.x;
        let dy = self.p2.y - self.p1.y;
        let mut t_min = 0.0;
        let mut t_max = 1.0;

        // Updates t_min and t_max for a boundary defined by (p, q).
        // Returns false if the segment is parallel to the boundary and outside.
        let update = |p: f64, q: f64, t_min: &mut f64, t_max: &mut f64| -> bool {
            if p.abs() < std::f64::EPSILON {
                if q < 0.0 {
                    return false;
                }
            } else {
                let t = q / p;
                if p < 0.0 {
                    if t > *t_min {
                        *t_min = t;
                    }
                } else {
                    if t < *t_max {
                        *t_max = t;
                    }
                }
            }
            true
        };

        // Left boundary: x = rect.x0
        if !update(-dx, self.p1.x - rect.x0, &mut t_min, &mut t_max) {
            return None;
        }
        // Right boundary: x = rect.x1
        if !update(dx, rect.x1 - self.p1.x, &mut t_min, &mut t_max) {
            return None;
        }
        // Top boundary: y = rect.y0
        if !update(-dy, self.p1.y - rect.y0, &mut t_min, &mut t_max) {
            return None;
        }
        // Bottom boundary: y = rect.y1
        if !update(dy, rect.y1 - self.p1.y, &mut t_min, &mut t_max) {
            return None;
        }

        if t_min > t_max {
            return None;
        }

        // Determine which intersection to return:
        // if p1 is inside, return the exit point (t_max); otherwise, the entry point (t_min).
        let inside = self.p1.x >= rect.x0
            && self.p1.x <= rect.x1
            && self.p1.y >= rect.y0
            && self.p1.y <= rect.y1;
        let t = if inside { t_max } else { t_min };

        if t < 0.0 || t > 1.0 {
            return None;
        }

        Some(Point {
            x: self.p1.x + t * dx,
            y: self.p1.y + t * dy,
        })
    }
}

pub fn compute_path(
    obstacles: &[Rect],
    start: &Point,
    goal: &Point,
    margin: f64,
) -> Option<Vec<Point>> {
    let candidate_nodes = extract_candidate_nodes(obstacles, margin);
    let (nodes, graph) = build_graph(&candidate_nodes, start, goal, obstacles, margin);
    // Start is at index 0 and goal at index 1.
    if let Some(path_indices) = astar(&nodes, &graph, 0, 1) {
        Some(path_indices.into_iter().map(|i| nodes[i]).collect())
    } else {
        None
    }
}

/// State used for the A* search.
#[derive(Copy, Clone, Debug)]
struct State {
    position: usize,
    g: f64, // actual cost from start
    f: f64, // g + heuristic
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.f == other.f
    }
}

impl Eq for State {}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // We want a min-heap based on f-value.
        other.f.partial_cmp(&self.f).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Perform A* search on the graph.
/// The heuristic used is the Euclidean distance from the current node to the goal.
fn astar(
    nodes: &[Point],
    graph: &[Vec<Edge>],
    start_idx: usize,
    goal_idx: usize,
) -> Option<Vec<usize>> {
    let n = nodes.len();
    let mut dist = vec![std::f64::INFINITY; n];
    let mut prev = vec![None; n];
    let mut heap = BinaryHeap::new();

    dist[start_idx] = 0.0;
    heap.push(State {
        position: start_idx,
        g: 0.0,
        f: nodes[start_idx].distance(nodes[goal_idx]),
    });

    while let Some(State { position, g, f: _ }) = heap.pop() {
        if position == goal_idx {
            // Reconstruct path from start to goal.
            let mut path = Vec::new();
            let mut current = goal_idx;
            while let Some(p) = prev[current] {
                path.push(current);
                current = p;
            }
            path.push(start_idx);
            path.reverse();
            return Some(path);
        }

        // If we've found a better path already, skip.
        if g > dist[position] {
            continue;
        }

        for edge in &graph[position] {
            let next = edge.to;
            let new_g = g + edge.cost;
            if new_g < dist[next] {
                dist[next] = new_g;
                prev[next] = Some(position);
                let new_f = new_g + nodes[next].distance(nodes[goal_idx]);
                heap.push(State {
                    position: next,
                    g: new_g,
                    f: new_f,
                });
            }
        }
    }
    None
}

/// An edge in the navigation graph.
#[derive(Debug, Clone)]
struct Edge {
    to: usize,
    cost: f64,
}

/// Build a graph from candidate nodes (including start and goal).
/// Each edge is added only if the straight-line segment is collision-free.
fn build_graph(
    candidate_nodes: &[Point],
    start: &Point,
    goal: &Point,
    obstacles: &[Rect],
    margin: f64,
) -> (Vec<Point>, Vec<Vec<Edge>>) {
    let mut nodes = Vec::new();
    // Insert start and goal at indices 0 and 1.
    nodes.push(*start);
    nodes.push(*goal);
    // Then add the candidate nodes from obstacles.
    nodes.extend_from_slice(candidate_nodes);

    let n = nodes.len();
    let mut graph: Vec<Vec<Edge>> = vec![vec![]; n];

    // Check every pair of nodes.
    for i in 0..n {
        for j in (i + 1)..n {
            if is_edge_valid(&nodes[i], &nodes[j], obstacles, margin) {
                let dist = nodes[i].distance(nodes[j]);
                graph[i].push(Edge { to: j, cost: dist });
                graph[j].push(Edge { to: i, cost: dist });
            }
        }
    }
    (nodes, graph)
}

/// Checks if the straight-line segment between p0 and p1 is collision-free.
/// If an edge touches an expanded obstacle, but both endpoints lie on the original boundary,
/// we allow it (this is our “narrow gap” handling).
fn is_edge_valid(p0: &Point, p1: &Point, obstacles: &[Rect], margin: f64) -> bool {
    for obs in obstacles {
        // Use the expanded rectangle for collision detection.
        let expanded = expand_rect(obs, margin);
        if (Segment { p1: *p0, p2: *p1 })
            .intersect_rect(&expanded)
            .is_some()
        {
            if (Segment { p1: *p0, p2: *p1 }).intersect_rect(obs).is_none() {
                continue;
            }
            return false;
        }
    }
    true
}

/// Extract candidate nodes from obstacles.
/// For each obstacle we take the corners of the expanded rectangle, and, if desired,
/// add midpoints of the original edges (to allow path segments that pass between obstacles).
fn extract_candidate_nodes(obstacles: &[Rect], margin: f64) -> Vec<Point> {
    let mut nodes = Vec::new();
    for obs in obstacles {
        // Candidate nodes from the expanded rectangle corners:
        let exp = expand_rect(obs, margin);
        nodes.push(Point {
            x: exp.x0,
            y: exp.y0,
        });
        nodes.push(Point {
            x: exp.x1,
            y: exp.y0,
        });
        nodes.push(Point {
            x: exp.x1,
            y: exp.y1,
        });
        nodes.push(Point {
            x: exp.x0,
            y: exp.y1,
        });

        // Additionally, add the midpoints of the original edges.
        // (This may be used in cases where the gap between obstacles is too narrow.)
        let mid_top = Point {
            x: (obs.x0 + obs.x1) / 2.0,
            y: obs.y0,
        };
        let mid_bottom = Point {
            x: (obs.x0 + obs.x1) / 2.0,
            y: obs.y1,
        };
        let mid_left = Point {
            x: obs.x0,
            y: (obs.y0 + obs.y1) / 2.0,
        };
        let mid_right = Point {
            x: obs.x1,
            y: (obs.y0 + obs.y1) / 2.0,
        };
        nodes.push(mid_top);
        nodes.push(mid_bottom);
        nodes.push(mid_left);
        nodes.push(mid_right);
    }
    nodes
}

/// Given a rectangle, return an “expanded” rectangle by subtracting margin from the
/// minimum coordinates and adding margin to the maximum coordinates.
fn expand_rect(rect: &Rect, margin: f64) -> Rect {
    Rect {
        x0: rect.x0 - margin,
        y0: rect.y0 - margin,
        x1: rect.x1 + margin,
        y1: rect.y1 + margin,
    }
}

pub mod signal_serde {
    use std::fmt::Debug;

    use crate::RwSignal;
    use floem::prelude::SignalGet;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    // Serialize the inner value of the signal.
    pub fn serialize<S, T>(signal: &RwSignal<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Serialize + Clone + 'static,
    {
        signal.get_untracked().serialize(serializer)
    }

    // Deserialize a value and wrap it in a new signal.
    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<RwSignal<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: Deserialize<'de> + Clone + Debug + 'static,
    {
        let value = T::deserialize(deserializer)?;
        Ok(RwSignal::new(value))
    }
}
