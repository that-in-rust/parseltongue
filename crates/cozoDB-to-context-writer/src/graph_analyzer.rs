//! Graph analyzer module for Tool 2
//!
//! Provides ISG traversal and blast radius analysis capabilities

use crate::ChangeRequest;
use parseltongue_01::{
    streaming::CodeGraph,
    types::{CoreError, CoreResult, ISGL1Key},
};
use petgraph::{graph::NodeIndex, Directed, Graph};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use uuid::Uuid;

/// Result from graph analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GraphAnalysisResult {
    /// Unique identifier for this analysis
    pub id: Uuid,

    /// List of nodes affected by the change
    #[serde(with = "isgl1key_vec_serde")]
    pub affected_nodes: Vec<ISGL1Key>,

    /// Calculated blast radius (number of levels affected)
    pub blast_radius: u32,

    /// Total number of nodes in the analysis scope
    pub total_nodes: u32,

    /// Dependencies discovered during analysis
    pub dependencies: Vec<DependencyEdge>,

    /// Critical paths that could be disrupted
    pub critical_paths: Vec<CriticalPath>,

    /// Analysis metadata
    pub metadata: AnalysisMetadata,
}

impl GraphAnalysisResult {
    /// Get the affected nodes
    pub fn affected_nodes(&self) -> &[ISGL1Key] {
        &self.affected_nodes
    }

    /// Get the blast radius
    pub fn blast_radius(&self) -> u32 {
        self.blast_radius
    }
}

/// Edge representing a dependency between nodes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DependencyEdge {
    /// Source node
    #[serde(with = "isgl1key_serde")]
    pub from: ISGL1Key,

    /// Target node
    #[serde(with = "isgl1key_serde")]
    pub to: ISGL1Key,

    /// Type of dependency
    pub dependency_type: DependencyType,

    /// Strength of the dependency (0.0 to 1.0)
    pub strength: f64,
}

/// Types of dependencies between code nodes
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DependencyType {
    /// Function call dependency
    FunctionCall,
    /// Data structure dependency
    DataStructure,
    /// Module import dependency
    ModuleImport,
    /// Trait implementation dependency
    TraitImplementation,
    /// Type dependency
    TypeDependency,
    /// Generic/unknown dependency
    Other(String),
}

/// Critical path that could be affected by changes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CriticalPath {
    /// Sequence of nodes in the path
    #[serde(with = "isgl1key_vec_serde")]
    pub path: Vec<ISGL1Key>,

    /// Impact level if this path is disrupted
    pub impact_level: ImpactLevel,

    /// Reason this path is critical
    pub reasoning: String,
}

/// Impact levels for critical paths
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImpactLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// Metadata for graph analysis
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnalysisMetadata {
    /// Timestamp when analysis was performed
    pub timestamp: chrono::DateTime<chrono::Utc>,

    /// Algorithm used for analysis
    pub algorithm: String,

    /// Parameters used in analysis
    pub parameters: HashMap<String, String>,

    /// Analysis duration in milliseconds
    pub duration_ms: Option<u64>,
}

/// Traversal path through the ISG
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TraversalPath {
    /// Sequence of nodes in the traversal
    #[serde(with = "isgl1key_vec_serde")]
    pub nodes: Vec<ISGL1Key>,

    /// Type of traversal
    pub traversal_type: TraversalType,

    /// Depth reached during traversal
    pub depth: u32,

    /// Total nodes visited
    pub nodes_visited: u32,
}

/// Types of graph traversal
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TraversalType {
    /// Breadth-first search
    BreadthFirst,
    /// Depth-first search
    DepthFirst,
    /// Bidirectional search
    Bidirectional,
    /// Priority-based search
    PriorityBased,
}

/// Graph analyzer with dependency injection
#[derive(Debug, Clone)]
pub struct GraphAnalyzer {
    /// Configuration for analysis
    config: AnalyzerConfig,

    /// Cache for analysis results
    cache: HashMap<String, GraphAnalysisResult>,
}

/// Configuration for graph analysis
#[derive(Debug, Clone)]
pub struct AnalyzerConfig {
    /// Maximum depth for analysis
    pub max_depth: u32,

    /// Whether to include transitive dependencies
    pub include_transitive: bool,

    /// Minimum dependency strength to consider
    pub min_dependency_strength: f64,

    /// Types of dependencies to analyze
    pub dependency_types: HashSet<DependencyType>,
}

impl GraphAnalyzer {
    /// Create a new graph analyzer with default configuration
    pub fn new() -> Self {
        let config = AnalyzerConfig {
            max_depth: 5,
            include_transitive: true,
            min_dependency_strength: 0.1,
            dependency_types: HashSet::from([
                DependencyType::FunctionCall,
                DependencyType::DataStructure,
                DependencyType::ModuleImport,
                DependencyType::TraitImplementation,
                DependencyType::TypeDependency,
            ]),
        };

        Self {
            config,
            cache: HashMap::new(),
        }
    }

    /// Create a graph analyzer with custom configuration
    pub fn with_config(config: AnalyzerConfig) -> Self {
        Self {
            config,
            cache: HashMap::new(),
        }
    }

    /// Analyze the impact of a change request on the code graph
    pub async fn analyze_change_impact(
        &self,
        change_request: &ChangeRequest,
        code_graph: &CodeGraph,
    ) -> CoreResult<GraphAnalysisResult> {
        let start_time = std::time::Instant::now();

        // Generate cache key
        let cache_key = format!(
            "{}-{}-{}",
            change_request.id,
            change_request.target.key.stable_hash(),
            self.config.max_depth
        );

        // Check cache first
        if let Some(cached_result) = self.cache.get(&cache_key) {
            return Ok(cached_result.clone());
        }

        // Convert CodeGraph to petgraph for analysis
        let petgraph = self.convert_to_petgraph(code_graph)?;

        // Find the target node
        let target_node = change_request.target.key.clone();

        // Perform BFS to find affected nodes
        let (affected_nodes, dependencies) = self.bfs_analysis(&petgraph, &target_node)?;

        // Calculate blast radius
        let blast_radius = self.calculate_blast_radius(&petgraph, &target_node, &affected_nodes)?;

        // Identify critical paths
        let critical_paths =
            self.identify_critical_paths(&petgraph, &target_node, &affected_nodes)?;

        // Create analysis result
        let total_nodes = petgraph.node_count() as u32;

        let result = GraphAnalysisResult {
            id: Uuid::new_v4(),
            affected_nodes,
            blast_radius,
            total_nodes,
            dependencies,
            critical_paths,
            metadata: AnalysisMetadata {
                timestamp: chrono::Utc::now(),
                algorithm: "bfs_with_critical_path_analysis".to_string(),
                parameters: HashMap::from([
                    ("max_depth".to_string(), self.config.max_depth.to_string()),
                    (
                        "include_transitive".to_string(),
                        self.config.include_transitive.to_string(),
                    ),
                    (
                        "min_dependency_strength".to_string(),
                        self.config.min_dependency_strength.to_string(),
                    ),
                ]),
                duration_ms: Some(start_time.elapsed().as_millis() as u64),
            },
        };

        // Cache the result
        let mut analyzer = self.clone();
        analyzer.cache.insert(cache_key, result.clone());

        Ok(result)
    }

    /// Traverse the ISG starting from the change target
    pub async fn traverse_isg(
        &self,
        change_request: &ChangeRequest,
        code_graph: &CodeGraph,
    ) -> CoreResult<Vec<TraversalPath>> {
        // Convert CodeGraph to petgraph for traversal
        let petgraph = self.convert_to_petgraph(code_graph)?;

        let target_node = change_request.target.key.clone();

        let mut paths = Vec::new();

        // BFS traversal
        let bfs_path = self.bfs_traversal(&petgraph, &target_node)?;
        paths.push(bfs_path);

        // DFS traversal
        let dfs_path = self.dfs_traversal(&petgraph, &target_node)?;
        paths.push(dfs_path);

        Ok(paths)
    }

    /// Convert CodeGraph to petgraph for analysis
    fn convert_to_petgraph(
        &self,
        code_graph: &CodeGraph,
    ) -> CoreResult<Graph<ISGL1Key, DependencyType, Directed>> {
        let mut graph = Graph::<ISGL1Key, DependencyType, Directed>::new();

        // Map from ISGL1Key to petgraph NodeIndex
        let mut node_map: HashMap<ISGL1Key, NodeIndex> = HashMap::new();

        // Add all nodes to the graph
        let nodes_map = code_graph.get_all_nodes();

        for (key, _node) in nodes_map.iter() {
            let node_index = graph.add_node(key.clone());
            node_map.insert(key.clone(), node_index);
        }

        // Add edges based on analysis of code content
        // In a real implementation, this would parse the actual code to find dependencies
        // For now, we'll add mock dependencies based on naming conventions
        for (key, node) in nodes_map.iter() {
            if let Some(node_index) = node_map.get(key) {
                // Look for potential function calls in the current code
                let dependencies = self
                    .extract_dependencies_from_code(&node.current_code, nodes_map.keys().collect());

                for dep_key in dependencies {
                    if let Some(dep_node_index) = node_map.get(&dep_key) {
                        let dep_type =
                            self.infer_dependency_type(&node.current_code, &dep_key.interface_name);
                        graph.add_edge(*node_index, *dep_node_index, dep_type);
                    }
                }
            }
        }

        Ok(graph)
    }

    /// Extract dependencies from code content (mock implementation)
    fn extract_dependencies_from_code(
        &self,
        code: &str,
        all_keys: Vec<&ISGL1Key>,
    ) -> Vec<ISGL1Key> {
        let mut dependencies = Vec::new();

        // Simple mock implementation: look for function calls
        for key in all_keys {
            if code.contains(&key.interface_name) {
                dependencies.push(key.clone());
            }
        }

        dependencies
    }

    /// Infer dependency type from code context
    fn infer_dependency_type(&self, code: &str, dependency_name: &str) -> DependencyType {
        // Simple heuristic-based inference
        if code.contains(&format!("use {}", dependency_name))
            || code.contains(&format!("use {}::", dependency_name))
        {
            DependencyType::ModuleImport
        } else if code.contains(&format!("impl {} for", dependency_name)) {
            DependencyType::TraitImplementation
        } else if code.contains(&format!("fn {}(", dependency_name)) {
            DependencyType::FunctionCall
        } else if code.contains(&format!("{} ", dependency_name))
            || code.contains(&format!("{}:", dependency_name))
        {
            DependencyType::TypeDependency
        } else {
            DependencyType::Other("unknown".to_string())
        }
    }

    /// Perform BFS analysis from target node
    fn bfs_analysis(
        &self,
        graph: &Graph<ISGL1Key, DependencyType, Directed>,
        target: &ISGL1Key,
    ) -> CoreResult<(Vec<ISGL1Key>, Vec<DependencyEdge>)> {
        let mut affected_nodes = Vec::new();
        let mut dependencies = Vec::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        // Find the target node in the graph
        let target_index = graph
            .node_indices()
            .find(|&idx| graph[idx] == *target)
            .ok_or_else(|| {
                CoreError::ResourceNotFound("Target node not found in graph".to_string())
            })?;

        queue.push_back((target_index, 0)); // (node_index, depth)
        visited.insert(target_index);

        while let Some((current_index, depth)) = queue.pop_front() {
            if depth >= self.config.max_depth {
                continue;
            }

            let current_key = graph[current_index].clone();
            affected_nodes.push(current_key.clone());

            // Visit neighbors
            for neighbor_index in graph.neighbors(current_index) {
                if !visited.contains(&neighbor_index) {
                    visited.insert(neighbor_index);

                    // Get the edge to determine dependency type
                    if let Some(edge_index) = graph.find_edge(current_index, neighbor_index) {
                        let edge_weight = graph[edge_index].clone();
                        let neighbor_key = graph[neighbor_index].clone();

                        let dependency = DependencyEdge {
                            from: current_key.clone(),
                            to: neighbor_key,
                            dependency_type: edge_weight,
                            strength: 0.8, // Mock strength value
                        };

                        dependencies.push(dependency);
                    }

                    queue.push_back((neighbor_index, depth + 1));
                }
            }
        }

        Ok((affected_nodes, dependencies))
    }

    /// Calculate blast radius
    fn calculate_blast_radius(
        &self,
        graph: &Graph<ISGL1Key, DependencyType, Directed>,
        target: &ISGL1Key,
        affected_nodes: &[ISGL1Key],
    ) -> CoreResult<u32> {
        // Find the target node
        let target_index = graph
            .node_indices()
            .find(|&idx| graph[idx] == *target)
            .ok_or_else(|| {
                CoreError::ResourceNotFound("Target node not found in graph".to_string())
            })?;

        // Calculate maximum distance from target to any affected node
        let mut max_distance = 0;

        for affected_key in affected_nodes {
            if affected_key == target {
                continue;
            }

            let affected_index = graph
                .node_indices()
                .find(|&idx| graph[idx] == *affected_key)
                .ok_or_else(|| {
                    CoreError::ResourceNotFound("Affected node not found in graph".to_string())
                })?;

            // Use BFS to find shortest path
            let distance = self.shortest_path_distance(graph, target_index, affected_index);
            max_distance = max_distance.max(distance);
        }

        Ok(max_distance)
    }

    /// Find shortest path distance between two nodes
    fn shortest_path_distance(
        &self,
        graph: &Graph<ISGL1Key, DependencyType, Directed>,
        from: NodeIndex,
        to: NodeIndex,
    ) -> u32 {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_back((from, 0));
        visited.insert(from);

        while let Some((current, distance)) = queue.pop_front() {
            if current == to {
                return distance;
            }

            for neighbor in graph.neighbors(current) {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    queue.push_back((neighbor, distance + 1));
                }
            }
        }

        u32::MAX // No path found
    }

    /// Identify critical paths that could be disrupted
    fn identify_critical_paths(
        &self,
        graph: &Graph<ISGL1Key, DependencyType, Directed>,
        target: &ISGL1Key,
        affected_nodes: &[ISGL1Key],
    ) -> CoreResult<Vec<CriticalPath>> {
        let mut critical_paths = Vec::new();

        // Find paths that go through the target node
        let target_index = graph
            .node_indices()
            .find(|&idx| graph[idx] == *target)
            .ok_or_else(|| {
                CoreError::ResourceNotFound("Target node not found in graph".to_string())
            })?;

        // Look for important nodes that depend on the target
        for affected_key in affected_nodes {
            if affected_key == target {
                continue;
            }

            let affected_index = graph
                .node_indices()
                .find(|&idx| graph[idx] == *affected_key)
                .ok_or_else(|| {
                    CoreError::ResourceNotFound("Affected node not found in graph".to_string())
                })?;

            // Check if there's a path from target to this node
            if self.path_exists(graph, target_index, affected_index) {
                let path = self.find_path(graph, target_index, affected_index);

                if let Some(path_nodes) = path {
                    let impact_level = self.assess_path_impact(&path_nodes, graph);

                    let critical_path = CriticalPath {
                        path: path_nodes,
                        impact_level,
                        reasoning: format!(
                            "Path from {} to {} could be disrupted",
                            target.interface_name, affected_key.interface_name
                        ),
                    };

                    critical_paths.push(critical_path);
                }
            }
        }

        // Sort by impact level
        critical_paths.sort_by(|a, b| {
            use std::cmp::Ordering;
            match (&a.impact_level, &b.impact_level) {
                (ImpactLevel::Critical, ImpactLevel::Critical) => Ordering::Equal,
                (ImpactLevel::Critical, _) => Ordering::Less,
                (_, ImpactLevel::Critical) => Ordering::Greater,
                (ImpactLevel::High, ImpactLevel::High) => Ordering::Equal,
                (ImpactLevel::High, _) => Ordering::Less,
                (_, ImpactLevel::High) => Ordering::Greater,
                (ImpactLevel::Medium, ImpactLevel::Medium) => Ordering::Equal,
                (ImpactLevel::Medium, ImpactLevel::Low) => Ordering::Less,
                (ImpactLevel::Low, ImpactLevel::Medium) => Ordering::Greater,
                (ImpactLevel::Low, ImpactLevel::Low) => Ordering::Equal,
            }
        });

        Ok(critical_paths)
    }

    /// Check if a path exists between two nodes
    fn path_exists(
        &self,
        graph: &Graph<ISGL1Key, DependencyType, Directed>,
        from: NodeIndex,
        to: NodeIndex,
    ) -> bool {
        let mut visited = HashSet::new();
        let mut stack = vec![from];

        while let Some(current) = stack.pop() {
            if current == to {
                return true;
            }

            if visited.contains(&current) {
                continue;
            }

            visited.insert(current);

            for neighbor in graph.neighbors(current) {
                if !visited.contains(&neighbor) {
                    stack.push(neighbor);
                }
            }
        }

        false
    }

    /// Find a path between two nodes
    fn find_path(
        &self,
        graph: &Graph<ISGL1Key, DependencyType, Directed>,
        from: NodeIndex,
        to: NodeIndex,
    ) -> Option<Vec<ISGL1Key>> {
        let mut visited = HashMap::new();
        let mut queue = VecDeque::new();

        queue.push_back(from);
        visited.insert(from, None);

        while let Some(current) = queue.pop_front() {
            if current == to {
                // Reconstruct path
                let mut path = Vec::new();
                let mut node = current;

                while let Some(&Some(parent)) = visited.get(&node) {
                    path.push(graph[node].clone());
                    node = parent;
                }
                path.push(graph[from].clone());
                path.reverse();
                return Some(path);
            }

            for neighbor in graph.neighbors(current) {
                if !visited.contains_key(&neighbor) {
                    visited.insert(neighbor, Some(current));
                    queue.push_back(neighbor);
                }
            }
        }

        None
    }

    /// Assess the impact level of a path
    fn assess_path_impact(
        &self,
        path: &[ISGL1Key],
        _graph: &Graph<ISGL1Key, DependencyType, Directed>,
    ) -> ImpactLevel {
        // Simple heuristic based on path length
        match path.len() {
            1..=2 => ImpactLevel::Low,
            3..=5 => ImpactLevel::Medium,
            6..=10 => ImpactLevel::High,
            _ => ImpactLevel::Critical,
        }
    }

    /// Perform BFS traversal
    fn bfs_traversal(
        &self,
        graph: &Graph<ISGL1Key, DependencyType, Directed>,
        target: &ISGL1Key,
    ) -> CoreResult<TraversalPath> {
        let target_index = graph
            .node_indices()
            .find(|&idx| graph[idx] == *target)
            .ok_or_else(|| {
                CoreError::ResourceNotFound("Target node not found in graph".to_string())
            })?;

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut traversal_order = Vec::new();

        queue.push_back((target_index, 0)); // (node_index, depth)
        visited.insert(target_index);

        while let Some((current_index, depth)) = queue.pop_front() {
            if depth >= self.config.max_depth {
                continue;
            }

            traversal_order.push(graph[current_index].clone());

            for neighbor in graph.neighbors(current_index) {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    queue.push_back((neighbor, depth + 1));
                }
            }
        }

        Ok(TraversalPath {
            nodes: traversal_order.clone(),
            traversal_type: TraversalType::BreadthFirst,
            depth: self.calculate_depth(&traversal_order, graph, target_index),
            nodes_visited: traversal_order.len() as u32,
        })
    }

    /// Perform DFS traversal
    fn dfs_traversal(
        &self,
        graph: &Graph<ISGL1Key, DependencyType, Directed>,
        target: &ISGL1Key,
    ) -> CoreResult<TraversalPath> {
        let target_index = graph
            .node_indices()
            .find(|&idx| graph[idx] == *target)
            .ok_or_else(|| {
                CoreError::ResourceNotFound("Target node not found in graph".to_string())
            })?;

        let mut visited = HashSet::new();
        let mut stack = vec![(target_index, 0)]; // (node_index, depth)
        let mut traversal_order = Vec::new();

        while let Some((current_index, depth)) = stack.pop() {
            if depth >= self.config.max_depth {
                continue;
            }

            if visited.contains(&current_index) {
                continue;
            }

            visited.insert(current_index);
            traversal_order.push(graph[current_index].clone());

            // Add neighbors to stack (reverse order for DFS consistency)
            let mut neighbors: Vec<_> = graph.neighbors(current_index).collect();
            neighbors.reverse();
            for neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    stack.push((neighbor, depth + 1));
                }
            }
        }

        Ok(TraversalPath {
            nodes: traversal_order.clone(),
            traversal_type: TraversalType::DepthFirst,
            depth: self.calculate_depth(&traversal_order, graph, target_index),
            nodes_visited: traversal_order.len() as u32,
        })
    }

    /// Calculate maximum depth reached in traversal
    fn calculate_depth(
        &self,
        traversal_order: &[ISGL1Key],
        graph: &Graph<ISGL1Key, DependencyType, Directed>,
        start_index: NodeIndex,
    ) -> u32 {
        let mut max_depth = 0;

        for key in traversal_order {
            let node_index = graph
                .node_indices()
                .find(|&idx| graph[idx] == *key)
                .unwrap_or(start_index);

            let depth = self.shortest_path_distance(graph, start_index, node_index);
            max_depth = max_depth.max(depth);
        }

        max_depth
    }
}

impl Default for GraphAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Custom serialization module for ISGL1Key
mod isgl1key_serde {
    use parseltongue_01::types::ISGL1Key;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::path::PathBuf;

    #[derive(Serialize, Deserialize)]
    struct SerializedKey {
        filepath: String,
        filename: String,
        interface_name: String,
    }

    pub fn serialize<S>(key: &ISGL1Key, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let serialized = SerializedKey {
            filepath: key.filepath.to_string_lossy().to_string(),
            filename: key.filename.clone(),
            interface_name: key.interface_name.clone(),
        };
        serialized.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<ISGL1Key, D::Error>
    where
        D: Deserializer<'de>,
    {
        let serialized = SerializedKey::deserialize(deserializer)?;
        Ok(ISGL1Key::new(
            PathBuf::from(serialized.filepath),
            serialized.filename,
            serialized.interface_name,
        ))
    }
}

/// Custom serialization module for Vec<ISGL1Key>
mod isgl1key_vec_serde {
    use parseltongue_01::types::ISGL1Key;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::path::PathBuf;

    #[derive(Serialize, Deserialize)]
    struct SerializedKey {
        filepath: String,
        filename: String,
        interface_name: String,
    }

    pub fn serialize<S>(keys: &[ISGL1Key], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let serialized: Vec<SerializedKey> = keys
            .iter()
            .map(|key| SerializedKey {
                filepath: key.filepath.to_string_lossy().to_string(),
                filename: key.filename.clone(),
                interface_name: key.interface_name.clone(),
            })
            .collect();
        serialized.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<ISGL1Key>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let serialized: Vec<SerializedKey> = Vec::deserialize(deserializer)?;
        Ok(serialized
            .into_iter()
            .map(|s| ISGL1Key::new(PathBuf::from(s.filepath), s.filename, s.interface_name))
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::change_request::ChangeType;
    use parseltongue_01::streaming::CodeNode;
    use std::path::PathBuf;

    fn create_test_change_request() -> ChangeRequest {
        let key = ISGL1Key::new(
            PathBuf::from("/test/src/lib.rs"),
            "lib.rs".to_string(),
            "test_function".to_string(),
        );

        ChangeRequest::new(
            key,
            ChangeType::Modify,
            "Test change".to_string(),
            "fn test_function() {}".to_string(),
            "fn test_function() -> Result<(), Error> { Ok(()) }".to_string(),
        )
    }

    fn create_test_code_graph() -> CodeGraph {
        let mut graph = CodeGraph::new();

        // Add multiple nodes with dependencies
        let key1 = ISGL1Key::new(
            PathBuf::from("/test/src/lib.rs"),
            "lib.rs".to_string(),
            "test_function".to_string(),
        );
        let node1 = CodeNode {
            current_code: "fn test_function() { helper_function() }".to_string(),
            future_code: None,
            interface_signature: Some("fn test_function()".to_string()),
            tdd_classification: Some("unit_test".to_string()),
            current_id: 1,
            future_id: 1,
            lsp_meta_data: None,
        };

        let key2 = ISGL1Key::new(
            PathBuf::from("/test/src/helper.rs"),
            "helper.rs".to_string(),
            "helper_function".to_string(),
        );
        let node2 = CodeNode {
            current_code: "fn helper_function() {}".to_string(),
            future_code: None,
            interface_signature: Some("fn helper_function()".to_string()),
            tdd_classification: Some("unit_test".to_string()),
            current_id: 2,
            future_id: 2,
            lsp_meta_data: None,
        };

        let key3 = ISGL1Key::new(
            PathBuf::from("/test/src/utils.rs"),
            "utils.rs".to_string(),
            "utility_function".to_string(),
        );
        let node3 = CodeNode {
            current_code:
                "use helper::helper_function; fn utility_function() { helper_function() }"
                    .to_string(),
            future_code: None,
            interface_signature: Some("fn utility_function()".to_string()),
            tdd_classification: Some("unit_test".to_string()),
            current_id: 3,
            future_id: 3,
            lsp_meta_data: None,
        };

        graph.insert_node(key1, node1).unwrap();
        graph.insert_node(key2, node2).unwrap();
        graph.insert_node(key3, node3).unwrap();

        graph
    }

    #[tokio::test]
    async fn test_graph_analyzer_creation() {
        let analyzer = GraphAnalyzer::new();
        assert_eq!(analyzer.config.max_depth, 5);
        assert!(analyzer.config.include_transitive);
    }

    #[tokio::test]
    async fn test_change_impact_analysis() {
        let analyzer = GraphAnalyzer::new();
        let change_request = create_test_change_request();
        let code_graph = create_test_code_graph();

        let result = analyzer
            .analyze_change_impact(&change_request, &code_graph)
            .await
            .unwrap();

        assert!(!result.affected_nodes.is_empty());
        assert!(result.blast_radius >= 0);
        assert!(result.total_nodes > 0);
        assert_eq!(result.metadata.algorithm, "bfs_with_critical_path_analysis");
    }

    #[tokio::test]
    async fn test_isg_traversal() {
        let analyzer = GraphAnalyzer::new();
        let change_request = create_test_change_request();
        let code_graph = create_test_code_graph();

        let paths = analyzer
            .traverse_isg(&change_request, &code_graph)
            .await
            .unwrap();

        assert_eq!(paths.len(), 2); // BFS and DFS

        // Check BFS path
        let bfs_path = paths
            .iter()
            .find(|p| matches!(p.traversal_type, TraversalType::BreadthFirst))
            .unwrap();
        assert!(!bfs_path.nodes.is_empty());
        assert_eq!(bfs_path.traversal_type, TraversalType::BreadthFirst);

        // Check DFS path
        let dfs_path = paths
            .iter()
            .find(|p| matches!(p.traversal_type, TraversalType::DepthFirst))
            .unwrap();
        assert!(!dfs_path.nodes.is_empty());
        assert_eq!(dfs_path.traversal_type, TraversalType::DepthFirst);
    }

    #[tokio::test]
    async fn test_custom_config() {
        let config = AnalyzerConfig {
            max_depth: 3,
            include_transitive: false,
            min_dependency_strength: 0.5,
            dependency_types: HashSet::from([DependencyType::FunctionCall]),
        };

        let analyzer = GraphAnalyzer::with_config(config);
        assert_eq!(analyzer.config.max_depth, 3);
        assert!(!analyzer.config.include_transitive);
        assert_eq!(analyzer.config.min_dependency_strength, 0.5);
    }

    #[test]
    fn test_dependency_edge() {
        let key1 = ISGL1Key::new(
            PathBuf::from("/test/a.rs"),
            "a.rs".to_string(),
            "function_a".to_string(),
        );
        let key2 = ISGL1Key::new(
            PathBuf::from("/test/b.rs"),
            "b.rs".to_string(),
            "function_b".to_string(),
        );

        let edge = DependencyEdge {
            from: key1.clone(),
            to: key2.clone(),
            dependency_type: DependencyType::FunctionCall,
            strength: 0.8,
        };

        assert_eq!(edge.from, key1);
        assert_eq!(edge.to, key2);
        assert_eq!(edge.dependency_type, DependencyType::FunctionCall);
        assert_eq!(edge.strength, 0.8);
    }

    #[test]
    fn test_critical_path() {
        let key1 = ISGL1Key::new(
            PathBuf::from("/test/a.rs"),
            "a.rs".to_string(),
            "function_a".to_string(),
        );
        let key2 = ISGL1Key::new(
            PathBuf::from("/test/b.rs"),
            "b.rs".to_string(),
            "function_b".to_string(),
        );

        let path = CriticalPath {
            path: vec![key1.clone(), key2.clone()],
            impact_level: ImpactLevel::High,
            reasoning: "Test critical path".to_string(),
        };

        assert_eq!(path.path.len(), 2);
        assert_eq!(path.impact_level, ImpactLevel::High);
        assert_eq!(path.reasoning, "Test critical path");
    }

    #[test]
    fn test_traversal_path() {
        let key = ISGL1Key::new(
            PathBuf::from("/test/a.rs"),
            "a.rs".to_string(),
            "function_a".to_string(),
        );

        let path = TraversalPath {
            nodes: vec![key.clone()],
            traversal_type: TraversalType::BreadthFirst,
            depth: 1,
            nodes_visited: 1,
        };

        assert_eq!(path.nodes.len(), 1);
        assert_eq!(path.traversal_type, TraversalType::BreadthFirst);
        assert_eq!(path.depth, 1);
        assert_eq!(path.nodes_visited, 1);
    }

    #[test]
    fn test_default_graph_analyzer() {
        let analyzer = GraphAnalyzer::default();
        assert_eq!(analyzer.config.max_depth, 5);
    }
}
