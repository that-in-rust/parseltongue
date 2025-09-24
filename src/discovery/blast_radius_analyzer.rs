//! Blast Radius Analysis for Parseltongue v2
//! 
//! Provides human-readable blast radius analysis with proper categorization
//! and separation of test files from production code.

use crate::discovery::{DiscoveryError, Result};
use crate::isg::{OptimizedISG, SigHash, EdgeKind, NodeData};
use petgraph::{Direction, visit::EdgeRef};
use std::collections::{HashMap, HashSet};
use serde::{Serialize, Deserialize};

/// Risk level categorization for blast radius analysis
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,      // 1-5 impacted entities
    Medium,   // 6-20 impacted entities  
    High,     // 21-50 impacted entities
    Critical, // 50+ impacted entities
}

impl RiskLevel {
    /// Categorize based on impact count
    pub fn from_impact_count(count: usize) -> Self {
        match count {
            0..=5 => Self::Low,
            6..=20 => Self::Medium,
            21..=50 => Self::High,
            _ => Self::Critical,
        }
    }
    
    /// Get human-readable description
    pub fn description(&self) -> &'static str {
        match self {
            Self::Low => "Low Risk (1-5 entities)",
            Self::Medium => "Medium Risk (6-20 entities)",
            Self::High => "High Risk (21-50 entities)",
            Self::Critical => "Critical Risk (50+ entities)",
        }
    }
}

/// Impact group categorized by relationship type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImpactGroup {
    /// Type of relationship (CALLS, USES, IMPLEMENTS)
    pub relationship_type: EdgeKind,
    /// Impacted entities in this group
    pub entities: Vec<ImpactedEntity>,
}

/// Individual impacted entity with human-readable information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImpactedEntity {
    /// Human-readable entity name
    pub name: String,
    /// File path with proper context
    pub file_path: String,
    /// Line number for navigation
    pub line_number: u32,
    /// Entity signature for context
    pub signature: String,
    /// Whether this is a test file
    pub is_test_file: bool,
}

/// Complete blast radius analysis result
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlastRadiusAnalysis {
    /// The entity that was analyzed
    pub target_entity: String,
    /// Risk level based on total impact
    pub risk_level: RiskLevel,
    /// Total number of impacted entities
    pub total_impact_count: usize,
    /// Impact groups organized by relationship type
    pub impact_groups: Vec<ImpactGroup>,
    /// Production code impacts (excluding tests)
    pub production_impacts: Vec<ImpactedEntity>,
    /// Test file impacts (separated for clarity)
    pub test_impacts: Vec<ImpactedEntity>,
}

impl BlastRadiusAnalysis {
    /// Format as human-readable summary
    pub fn format_summary(&self) -> String {
        let mut summary = String::new();
        
        summary.push_str(&format!("🎯 Blast Radius Analysis for: {}\n", self.target_entity));
        summary.push_str(&format!("📊 Risk Level: {}\n", self.risk_level.description()));
        summary.push_str(&format!("📈 Total Impact: {} entities\n", self.total_impact_count));
        summary.push_str(&format!("🏭 Production Impact: {} entities\n", self.production_impacts.len()));
        summary.push_str(&format!("🧪 Test Impact: {} entities\n\n", self.test_impacts.len()));
        
        // Group summary
        summary.push_str("📋 Impact by Relationship Type:\n");
        for group in &self.impact_groups {
            summary.push_str(&format!("  {:?}: {} entities\n", group.relationship_type, group.entities.len()));
        }
        
        summary.push_str("\n");
        
        // Detailed breakdown
        if !self.production_impacts.is_empty() {
            summary.push_str("🏭 Production Code Impacts:\n");
            for entity in &self.production_impacts {
                summary.push_str(&format!("  • {} ({}:{})\n", 
                    entity.name, entity.file_path, entity.line_number));
            }
            summary.push_str("\n");
        }
        
        if !self.test_impacts.is_empty() {
            summary.push_str("🧪 Test Code Impacts:\n");
            for entity in &self.test_impacts {
                summary.push_str(&format!("  • {} ({}:{})\n", 
                    entity.name, entity.file_path, entity.line_number));
            }
        }
        
        summary
    }
    
    /// Get production impact percentage
    pub fn production_impact_percentage(&self) -> f64 {
        if self.total_impact_count == 0 {
            return 0.0;
        }
        (self.production_impacts.len() as f64 / self.total_impact_count as f64) * 100.0
    }
    
    /// Check if this change would be high risk for production
    pub fn is_high_risk_for_production(&self) -> bool {
        matches!(self.risk_level, RiskLevel::High | RiskLevel::Critical) && 
        self.production_impacts.len() > 10
    }
}

/// Blast radius analyzer with human-readable output
pub struct BlastRadiusAnalyzer {
    isg: OptimizedISG,
}

impl BlastRadiusAnalyzer {
    /// Create new analyzer with ISG reference
    pub fn new(isg: OptimizedISG) -> Self {
        Self { isg }
    }
    
    /// Analyze blast radius for a given entity
    /// 
    /// # Arguments
    /// * `entity_name` - Human-readable entity name to analyze
    /// 
    /// # Returns
    /// * `BlastRadiusAnalysis` with human-readable output and proper categorization
    /// 
    /// # Performance Contract
    /// * Must complete in <1ms for simple analysis
    /// * Must provide 100% readable output with no hash values
    pub fn analyze_blast_radius(&self, entity_name: &str) -> Result<BlastRadiusAnalysis> {
        // Find the entity by name
        let entity_hashes = self.isg.find_by_name(entity_name);
        if entity_hashes.is_empty() {
            return Err(DiscoveryError::entity_not_found(entity_name));
        }
        
        // Use the first matching entity (for now)
        let target_hash = entity_hashes[0];
        
        // Calculate blast radius using ISG
        let blast_radius = self.isg.calculate_blast_radius(target_hash)
            .map_err(|e| DiscoveryError::internal(format!("ISG error: {}", e)))?;
        
        // Convert hash set to detailed impact information with proper relationship detection
        let impacts_with_relationships = self.detect_relationship_types(target_hash, &blast_radius)?;
        
        // Group impacts by relationship type
        let impact_groups = self.group_impacts_by_relationship(impacts_with_relationships);
        
        // Separate production and test impacts
        let mut production_impacts = Vec::new();
        let mut test_impacts = Vec::new();
        
        for group in &impact_groups {
            for entity in &group.entities {
                if entity.is_test_file {
                    test_impacts.push(entity.clone());
                } else {
                    production_impacts.push(entity.clone());
                }
            }
        }
        
        let total_impact_count = blast_radius.len();
        let risk_level = RiskLevel::from_impact_count(total_impact_count);
        
        Ok(BlastRadiusAnalysis {
            target_entity: entity_name.to_string(),
            risk_level,
            total_impact_count,
            impact_groups,
            production_impacts,
            test_impacts,
        })
    }
    
    /// Check if a file path represents a test file
    fn is_test_file(file_path: &str) -> bool {
        file_path.contains("test") || 
        file_path.contains("spec") ||
        file_path.ends_with("_test.rs") ||
        file_path.starts_with("tests/") ||
        file_path.starts_with("benches/")
    }
    
    /// Convert NodeData to ImpactedEntity with readable information
    fn node_to_impacted_entity(&self, node: &NodeData) -> ImpactedEntity {
        let file_path = node.file_path.to_string();
        let is_test_file = Self::is_test_file(&file_path);
        
        ImpactedEntity {
            name: node.name.to_string(),
            file_path,
            line_number: node.line,
            signature: node.signature.to_string(),
            is_test_file,
        }
    }
    
    /// Detect relationship types between target entity and impacted entities
    fn detect_relationship_types(
        &self, 
        target_hash: SigHash, 
        blast_radius: &HashSet<SigHash>
    ) -> Result<Vec<(NodeData, EdgeKind)>> {
        let mut impacts_with_relationships = Vec::new();
        
        // Get the ISG state to examine edges
        let state = self.isg.state.read();
        
        // Get target node index
        let target_idx = state.id_map.get(&target_hash)
            .ok_or_else(|| DiscoveryError::internal("Target node not found in ISG"))?;
        
        for &impact_hash in blast_radius {
            if let Ok(node) = self.isg.get_node(impact_hash) {
                // Find the relationship type by examining edges
                let impact_idx = state.id_map.get(&impact_hash);
                
                let relationship_type = if let Some(&impact_idx) = impact_idx {
                    // Check for direct edges between target and impact
                    self.find_edge_type(&state, *target_idx, impact_idx)
                        .unwrap_or(EdgeKind::Uses) // Default to Uses if no direct edge found
                } else {
                    EdgeKind::Uses // Default fallback
                };
                
                impacts_with_relationships.push((node, relationship_type));
            }
        }
        
        Ok(impacts_with_relationships)
    }
    
    /// Find the edge type between two nodes
    fn find_edge_type(
        &self,
        state: &crate::isg::ISGState,
        from_idx: petgraph::graph::NodeIndex,
        to_idx: petgraph::graph::NodeIndex,
    ) -> Option<EdgeKind> {
        // Check outgoing edges from target
        for edge_ref in state.graph.edges_directed(from_idx, Direction::Outgoing) {
            if edge_ref.target() == to_idx {
                return Some(*edge_ref.weight());
            }
        }
        
        // Check incoming edges to target (reverse relationship)
        for edge_ref in state.graph.edges_directed(from_idx, Direction::Incoming) {
            if edge_ref.source() == to_idx {
                return Some(*edge_ref.weight());
            }
        }
        
        None
    }

    /// Group impacts by relationship type
    fn group_impacts_by_relationship(
        &self, 
        impacts: Vec<(NodeData, EdgeKind)>
    ) -> Vec<ImpactGroup> {
        let mut groups: HashMap<EdgeKind, Vec<ImpactedEntity>> = HashMap::new();
        
        for (node, edge_kind) in impacts {
            let impacted_entity = self.node_to_impacted_entity(&node);
            groups.entry(edge_kind)
                .or_insert_with(Vec::new)
                .push(impacted_entity);
        }
        
        groups.into_iter()
            .map(|(relationship_type, entities)| ImpactGroup {
                relationship_type,
                entities,
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::isg::{NodeKind, NodeData};
    use std::sync::Arc;

    // Test helper to create sample ISG with known relationships
    fn create_test_isg() -> OptimizedISG {
        let isg = OptimizedISG::new();
        
        // Create sample nodes representing a realistic codebase
        let main_fn = NodeData {
            hash: SigHash::from_signature("fn main"),
            kind: NodeKind::Function,
            name: Arc::from("main"),
            signature: Arc::from("fn main()"),
            file_path: Arc::from("src/main.rs"),
            line: 1,
        };
        
        let user_service = NodeData {
            hash: SigHash::from_signature("struct UserService"),
            kind: NodeKind::Struct,
            name: Arc::from("UserService"),
            signature: Arc::from("struct UserService { db: Database }"),
            file_path: Arc::from("src/services/user.rs"),
            line: 10,
        };
        
        let database_trait = NodeData {
            hash: SigHash::from_signature("trait Database"),
            kind: NodeKind::Trait,
            name: Arc::from("Database"),
            signature: Arc::from("trait Database { fn connect(&self) -> Result<Connection>; }"),
            file_path: Arc::from("src/database/mod.rs"),
            line: 5,
        };
        
        let test_user_service = NodeData {
            hash: SigHash::from_signature("fn test_user_creation"),
            kind: NodeKind::Function,
            name: Arc::from("test_user_creation"),
            signature: Arc::from("fn test_user_creation()"),
            file_path: Arc::from("tests/user_service_test.rs"),
            line: 15,
        };
        
        // Add nodes to ISG
        isg.upsert_node(main_fn.clone());
        isg.upsert_node(user_service.clone());
        isg.upsert_node(database_trait.clone());
        isg.upsert_node(test_user_service.clone());
        
        // Add relationships
        // main() calls UserService
        isg.upsert_edge(main_fn.hash, user_service.hash, EdgeKind::Calls).unwrap();
        
        // UserService implements Database trait
        isg.upsert_edge(user_service.hash, database_trait.hash, EdgeKind::Implements).unwrap();
        
        // Test uses UserService
        isg.upsert_edge(test_user_service.hash, user_service.hash, EdgeKind::Uses).unwrap();
        
        isg
    }

    #[test]
    fn test_risk_level_categorization() {
        // Test risk level boundaries according to requirements
        assert_eq!(RiskLevel::from_impact_count(1), RiskLevel::Low);
        assert_eq!(RiskLevel::from_impact_count(5), RiskLevel::Low);
        assert_eq!(RiskLevel::from_impact_count(6), RiskLevel::Medium);
        assert_eq!(RiskLevel::from_impact_count(20), RiskLevel::Medium);
        assert_eq!(RiskLevel::from_impact_count(21), RiskLevel::High);
        assert_eq!(RiskLevel::from_impact_count(50), RiskLevel::High);
        assert_eq!(RiskLevel::from_impact_count(51), RiskLevel::Critical);
        assert_eq!(RiskLevel::from_impact_count(100), RiskLevel::Critical);
    }

    #[test]
    fn test_risk_level_descriptions() {
        assert_eq!(RiskLevel::Low.description(), "Low Risk (1-5 entities)");
        assert_eq!(RiskLevel::Medium.description(), "Medium Risk (6-20 entities)");
        assert_eq!(RiskLevel::High.description(), "High Risk (21-50 entities)");
        assert_eq!(RiskLevel::Critical.description(), "Critical Risk (50+ entities)");
    }

    #[test]
    fn test_blast_radius_analyzer_creation() {
        let isg = create_test_isg();
        let analyzer = BlastRadiusAnalyzer::new(isg);
        
        // Verify analyzer can be created
        // This test will pass once we implement the constructor
    }

    #[test]
    fn test_analyze_blast_radius_returns_readable_output() {
        let isg = create_test_isg();
        let analyzer = BlastRadiusAnalyzer::new(isg);
        
        // RED: This test should fail until we implement the method
        let result = analyzer.analyze_blast_radius("UserService");
        
        // Verify the result contains human-readable information
        assert!(result.is_ok(), "Analysis should succeed for valid entity");
        
        let analysis = result.unwrap();
        
        // Verify no hash values in output - all should be human-readable
        assert_eq!(analysis.target_entity, "UserService");
        assert!(analysis.total_impact_count > 0, "Should find some impacts");
        
        // Verify all entities have readable names (no hash values)
        for group in &analysis.impact_groups {
            for entity in &group.entities {
                assert!(!entity.name.contains("SigHash"), "Entity name should not contain hash: {}", entity.name);
                assert!(!entity.file_path.is_empty(), "File path should not be empty");
                assert!(entity.line_number > 0, "Line number should be valid");
            }
        }
    }

    #[test]
    fn test_impact_group_structure_by_relationship_type() {
        let isg = create_test_isg();
        let analyzer = BlastRadiusAnalyzer::new(isg);
        
        // RED: This test should fail until we implement grouping
        let result = analyzer.analyze_blast_radius("UserService");
        assert!(result.is_ok());
        
        let analysis = result.unwrap();
        
        // Verify impact groups are organized by relationship type
        let relationship_types: HashSet<EdgeKind> = analysis.impact_groups
            .iter()
            .map(|group| group.relationship_type)
            .collect();
        
        // Should have different relationship types (CALLS, USES, IMPLEMENTS)
        assert!(!relationship_types.is_empty(), "Should have relationship types");
        
        // Each group should contain entities of the same relationship type
        for group in &analysis.impact_groups {
            assert!(!group.entities.is_empty(), "Each group should have entities");
        }
    }

    #[test]
    fn test_separation_of_test_files_from_production() {
        let isg = create_test_isg();
        let analyzer = BlastRadiusAnalyzer::new(isg);
        
        // RED: This test should fail until we implement test file separation
        let result = analyzer.analyze_blast_radius("UserService");
        assert!(result.is_ok());
        
        let analysis = result.unwrap();
        
        // Verify test impacts are separated from production impacts
        let total_impacts = analysis.production_impacts.len() + analysis.test_impacts.len();
        assert_eq!(total_impacts, analysis.total_impact_count, "All impacts should be categorized");
        
        // Verify test files are properly identified
        for test_entity in &analysis.test_impacts {
            assert!(test_entity.is_test_file, "Test entity should be marked as test file");
            assert!(
                test_entity.file_path.contains("test") || 
                test_entity.file_path.contains("spec") ||
                test_entity.file_path.ends_with("_test.rs"),
                "Test file path should indicate it's a test: {}", test_entity.file_path
            );
        }
        
        // Verify production files are not marked as tests
        for prod_entity in &analysis.production_impacts {
            assert!(!prod_entity.is_test_file, "Production entity should not be marked as test file");
        }
    }

    #[test]
    fn test_is_test_file_detection() {
        // RED: This test should fail until we implement test file detection
        assert!(BlastRadiusAnalyzer::is_test_file("tests/user_test.rs"));
        assert!(BlastRadiusAnalyzer::is_test_file("src/lib_test.rs"));
        assert!(BlastRadiusAnalyzer::is_test_file("tests/integration/api_test.rs"));
        assert!(BlastRadiusAnalyzer::is_test_file("benches/benchmark_test.rs"));
        
        assert!(!BlastRadiusAnalyzer::is_test_file("src/main.rs"));
        assert!(!BlastRadiusAnalyzer::is_test_file("src/services/user.rs"));
        assert!(!BlastRadiusAnalyzer::is_test_file("src/database/mod.rs"));
    }

    #[test]
    fn test_100_percent_readable_output_no_hash_values() {
        let isg = create_test_isg();
        let analyzer = BlastRadiusAnalyzer::new(isg);
        
        // RED: This test should fail until we ensure no hash values in output
        let result = analyzer.analyze_blast_radius("UserService");
        assert!(result.is_ok());
        
        let analysis = result.unwrap();
        
        // Serialize to JSON and verify no hash representations
        let json_output = serde_json::to_string(&analysis).unwrap();
        
        // Should not contain any hash-like patterns
        assert!(!json_output.contains("SigHash"), "Output should not contain SigHash");
        assert!(!json_output.contains("0x"), "Output should not contain hex values");
        
        // Should contain human-readable information
        assert!(json_output.contains("UserService"), "Should contain target entity name");
        assert!(json_output.contains("src/"), "Should contain readable file paths");
    }

    #[test]
    fn test_performance_contract_under_1ms() {
        let isg = create_test_isg();
        let analyzer = BlastRadiusAnalyzer::new(isg);
        
        // RED: This test should fail until we optimize performance
        let start = std::time::Instant::now();
        let result = analyzer.analyze_blast_radius("UserService");
        let elapsed = start.elapsed();
        
        assert!(result.is_ok(), "Analysis should succeed");
        assert!(elapsed.as_millis() < 1, "Analysis should complete in <1ms, took {:?}", elapsed);
    }

    #[test]
    fn test_human_readable_summary_formatting() {
        let isg = create_test_isg();
        let analyzer = BlastRadiusAnalyzer::new(isg);
        
        let result = analyzer.analyze_blast_radius("UserService");
        assert!(result.is_ok());
        
        let analysis = result.unwrap();
        let summary = analysis.format_summary();
        
        // Verify summary contains expected sections
        assert!(summary.contains("🎯 Blast Radius Analysis for: UserService"));
        assert!(summary.contains("📊 Risk Level:"));
        assert!(summary.contains("📈 Total Impact:"));
        assert!(summary.contains("🏭 Production Impact:"));
        assert!(summary.contains("🧪 Test Impact:"));
        assert!(summary.contains("📋 Impact by Relationship Type:"));
        
        // Should be human-readable with no technical jargon
        assert!(!summary.contains("SigHash"));
        assert!(!summary.contains("NodeIndex"));
    }

    #[test]
    fn test_production_impact_percentage_calculation() {
        let isg = create_test_isg();
        let analyzer = BlastRadiusAnalyzer::new(isg);
        
        let result = analyzer.analyze_blast_radius("UserService");
        assert!(result.is_ok());
        
        let analysis = result.unwrap();
        let percentage = analysis.production_impact_percentage();
        
        // Should be a valid percentage
        assert!(percentage >= 0.0 && percentage <= 100.0);
        
        // If we have impacts, percentage should be meaningful
        if analysis.total_impact_count > 0 {
            let expected = (analysis.production_impacts.len() as f64 / analysis.total_impact_count as f64) * 100.0;
            assert!((percentage - expected).abs() < 0.001);
        }
    }

    #[test]
    fn test_high_risk_production_detection() {
        let isg = create_test_isg();
        let analyzer = BlastRadiusAnalyzer::new(isg);
        
        let result = analyzer.analyze_blast_radius("UserService");
        assert!(result.is_ok());
        
        let analysis = result.unwrap();
        
        // For our small test ISG, this should not be high risk
        // (we only have a few entities)
        assert!(!analysis.is_high_risk_for_production());
    }

    #[test]
    fn test_entity_not_found_error_handling() {
        let isg = create_test_isg();
        let analyzer = BlastRadiusAnalyzer::new(isg);
        
        let result = analyzer.analyze_blast_radius("NonExistentEntity");
        
        // Should return appropriate error for missing entity
        assert!(result.is_err());
        
        if let Err(error) = result {
            assert!(matches!(error, DiscoveryError::EntityNotFound { .. }));
        }
    }
}