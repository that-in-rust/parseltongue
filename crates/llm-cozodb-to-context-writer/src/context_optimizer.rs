//! Context optimizer implementation for generating optimized CodeGraphContext.json files.

use std::sync::Arc;
use std::time::{Duration, Instant};
use std::path::Path;
use std::collections::HashMap;
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use async_trait::async_trait;
use rayon::prelude::*;

use crate::errors::*;
use crate::llm_client::*;
use parseltongue_core::entities::{CodeEntity, EntityType, InterfaceSignature, Visibility, LineRange};

/// Context optimizer interface
#[async_trait::async_trait]
pub trait ContextOptimizer: Send + Sync {
    /// Generate optimized context from database
    async fn generate_context(&self, output_path: &str) -> Result<ContextResult>;

    /// Generate context for specific entities
    async fn generate_context_for_entities(&self, entities: &[CodeEntity], output_path: &str) -> Result<ContextResult>;

    /// Get current statistics
    fn get_stats(&self) -> ContextOptimizerStats;
}

/// Context generation results
#[derive(Debug, Clone)]
pub struct ContextResult {
    pub context_id: String,
    pub output_path: String,
    pub entities_processed: usize,
    pub entities_optimized: usize,
    pub tokens_generated: usize,
    pub optimization_ratio: f32,
    pub generation_time: Duration,
    pub errors: Vec<String>,
}

/// Context optimizer statistics
#[derive(Debug, Clone, Default)]
pub struct ContextOptimizerStats {
    pub contexts_generated: usize,
    pub entities_processed: usize,
    pub tokens_generated: usize,
    pub optimization_savings: usize,
    pub llm_requests_made: usize,
    pub total_generation_time: Duration,
}

/// Context graph analysis result
#[derive(Debug, Clone)]
pub struct ContextGraph {
    pub entities: Vec<CodeEntity>,
    pub relationships: Vec<EntityRelationship>,
    pub centrality_scores: HashMap<String, f32>,
    pub connectivity_clusters: Vec<Vec<String>>,
    pub metadata: GraphMetadata,
}

/// Graph metadata for analysis
#[derive(Debug, Clone)]
pub struct GraphMetadata {
    pub total_entities: usize,
    pub total_relationships: usize,
    pub graph_density: f32,
    pub average_degree: f32,
    pub max_centrality: f32,
    pub analysis_timestamp: std::time::SystemTime,
}

/// Context optimizer implementation
pub struct ContextOptimizerImpl {
    config: crate::ContextWriterConfig,
    llm_client: Arc<ContextLlmClientImpl>,
    stats: std::sync::Mutex<ContextOptimizerStats>,
}

impl ContextOptimizerImpl {
    /// Create new context optimizer
    pub fn new(
        config: crate::ContextWriterConfig,
        llm_client: Arc<ContextLlmClientImpl>,
    ) -> Self {
        Self {
            config,
            llm_client,
            stats: std::sync::Mutex::new(ContextOptimizerStats::default()),
        }
    }

    /// Query entity graph from database
    async fn query_entity_graph(&self) -> Result<ContextGraph> {
        // TODO: Implement actual database query
        // For now, return sample graph for testing
        let sample_entities = self.create_sample_entities()?;
        let sample_relationships = self.create_sample_relationships(&sample_entities);

        let centrality_scores = self.calculate_centrality_scores(&sample_entities, &sample_relationships);
        let connectivity_clusters = self.identify_connectivity_clusters(&sample_entities, &sample_relationships);

        let metadata = GraphMetadata {
            total_entities: sample_entities.len(),
            total_relationships: sample_relationships.len(),
            graph_density: self.calculate_graph_density(&sample_entities, &sample_relationships),
            average_degree: self.calculate_average_degree(&sample_entities, &sample_relationships),
            max_centrality: centrality_scores.values().cloned().fold(0.0, f32::max),
            analysis_timestamp: std::time::SystemTime::now(),
        };

        Ok(ContextGraph {
            entities: sample_entities,
            relationships: sample_relationships,
            centrality_scores,
            connectivity_clusters,
            metadata,
        })
    }

    /// Create sample entities for testing
    fn create_sample_entities(&self) -> Result<Vec<CodeEntity>> {
        let interface_signature = InterfaceSignature {
            entity_type: EntityType::Function,
            name: "sample_function".to_string(),
            visibility: Visibility::Public,
            file_path: "src/lib.rs".into(),
            line_range: LineRange::new(1, 10).unwrap(),
            module_path: vec!["sample_module".to_string()],
            documentation: None,
            language_specific: parseltongue_core::entities::LanguageSpecificSignature::Rust(
                parseltongue_core::entities::RustSignature {
                    generics: vec![],
                    lifetimes: vec![],
                    where_clauses: vec![],
                    attributes: vec![],
                    trait_impl: None,
                }
            ),
        };

        let sample_entities = vec![
            CodeEntity::new(
                "rust:fn:sample_function:src/lib.rs:1-10".to_string(),
                interface_signature,
            ).unwrap(),
        ];

        Ok(sample_entities)
    }

    /// Create sample relationships for testing
    fn create_sample_relationships(&self, entities: &[CodeEntity]) -> Vec<EntityRelationship> {
        if entities.is_empty() {
            return vec![];
        }

        vec![
            EntityRelationship {
                source_id: entities[0].isgl1_key.clone(),
                target_id: entities[0].isgl1_key.clone(),
                relationship_type: RelationshipType::RelatedTo,
                strength: 1.0,
                context: Some("sample relationship".to_string()),
            }
        ]
    }

    /// Calculate centrality scores for entities
    fn calculate_centrality_scores(&self, entities: &[CodeEntity], relationships: &[EntityRelationship]) -> HashMap<String, f32> {
        let mut scores = HashMap::new();

        // Simple degree centrality calculation
        for entity in entities {
            let degree = relationships.iter()
                .filter(|r| r.source_id == entity.isgl1_key || r.target_id == entity.isgl1_key)
                .count() as f32;
            scores.insert(entity.isgl1_key.clone(), degree);
        }

        scores
    }

    /// Identify connectivity clusters in the graph
    fn identify_connectivity_clusters(&self, entities: &[CodeEntity], relationships: &[EntityRelationship]) -> Vec<Vec<String>> {
        if entities.is_empty() {
            return vec![];
        }

        // Simple clustering - put all entities in one cluster for now
        vec![entities.iter().map(|e| e.isgl1_key.clone()).collect()]
    }

    /// Calculate graph density
    fn calculate_graph_density(&self, entities: &[CodeEntity], relationships: &[EntityRelationship]) -> f32 {
        if entities.len() < 2 {
            return 0.0;
        }

        let possible_edges = entities.len() * (entities.len() - 1) / 2;
        relationships.len() as f32 / possible_edges as f32
    }

    /// Calculate average degree
    fn calculate_average_degree(&self, entities: &[CodeEntity], relationships: &[EntityRelationship]) -> f32 {
        if entities.is_empty() {
            return 0.0;
        }

        relationships.len() as f32 * 2.0 / entities.len() as f32
    }

    /// Write optimized context to file
    async fn write_context_file(&self, response: &ContextOptimizationResponse, output_path: &str) -> Result<()> {
        // Create output directory if it doesn't exist
        if let Some(parent) = Path::new(output_path).parent() {
            tokio::fs::create_dir_all(parent).await.map_err(|e| ContextWriterError::FileError {
                path: parent.to_string_lossy().to_string(),
                reason: format!("Failed to create directory: {}", e),
            })?;
        }

        let context_json = serde_json::to_string_pretty(response).map_err(|e| {
            ContextWriterError::SerializationError {
                data: "optimized context".to_string(),
                reason: format!("Failed to serialize context: {}", e),
            }
        })?;

        tokio::fs::write(output_path, context_json).await.map_err(|e| {
            ContextWriterError::FileError {
                path: output_path.to_string(),
                reason: format!("Failed to write context file: {}", e),
            }
        })?;

        Ok(())
    }

    /// Update statistics
    fn update_stats(&self, entities_count: usize, tokens_generated: usize, optimization_ratio: f32, generation_time: Duration) {
        if let Ok(mut stats) = self.stats.lock() {
            stats.contexts_generated += 1;
            stats.entities_processed += entities_count;
            stats.tokens_generated += tokens_generated;
            stats.optimization_savings += (entities_count as f32 * optimization_ratio) as usize;
            stats.llm_requests_made += 1;
            stats.total_generation_time += generation_time;
        }
    }
}

#[async_trait::async_trait]
impl ContextOptimizer for ContextOptimizerImpl {
    async fn generate_context(&self, output_path: &str) -> Result<ContextResult> {
        let start_time = Instant::now();
        let mut errors = Vec::new();

        println!(
            "{}",
            style("Starting context generation...").blue().bold()
        );

        // Setup progress bar
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} [{elapsed_precise}] {msg}")
                .unwrap()
        );
        pb.set_message("Querying entity graph...");

        // Query entity graph from database
        let graph = self.query_entity_graph().await.map_err(|e| {
            let error_msg = format!("Failed to query entity graph: {}", e);
            pb.println(format!("{} {}", style("⚠").yellow().for_stderr().to_string(), error_msg.clone()));
            errors.push(error_msg.clone());
            e
        })?;

        pb.set_message(format!("Processing {} entities...", graph.entities.len()));

        if graph.entities.is_empty() {
            pb.finish_with_message("No entities found for context generation");
            return Ok(ContextResult {
                context_id: uuid::Uuid::new_v4().to_string(),
                output_path: output_path.to_string(),
                entities_processed: 0,
                entities_optimized: 0,
                tokens_generated: 0,
                optimization_ratio: 0.0,
                generation_time: start_time.elapsed(),
                errors,
            });
        }

        // Create context optimization request
        let request = ContextOptimizationRequest {
            entities: graph.entities.clone(),
            relationships: graph.relationships.clone(),
            target_context_size: self.config.max_context_tokens,
            focus_areas: vec!["core_types".to_string(), "implementations".to_string()],
            optimization_goals: vec![
                OptimizationGoal::MinimizeSize,
                OptimizationGoal::MaximizeRelevance,
                OptimizationGoal::PreserveConnectivity,
            ],
        };

        pb.set_message("Optimizing context with LLM...");

        // Generate optimized context
        let response = self.llm_client.optimize_context(request).await.map_err(|e| {
            let error_msg = format!("Context optimization failed: {}", e);
            pb.println(format!("{} {}", style("⚠").yellow().for_stderr().to_string(), error_msg.clone()));
            errors.push(error_msg.clone());
            e
        })?;

        pb.set_message("Writing context file...");

        // Write optimized context to file
        self.write_context_file(&response, output_path).await.map_err(|e| {
            let error_msg = format!("Failed to write context file: {}", e);
            pb.println(format!("{} {}", style("⚠").yellow().for_stderr().to_string(), error_msg.clone()));
            errors.push(error_msg.clone());
            e
        })?;

        pb.finish_with_message("Context generation completed");

        let generation_time = start_time.elapsed();
        let entities_optimized = response.optimized_entities.len();
        let tokens_generated = response.context_metadata.token_count;
        let optimization_ratio = response.pruning_summary.pruning_ratio;

        // Print summary
        println!("\n{}", style("Context Generation Summary:").green().bold());
        println!("Context ID: {}", response.context_metadata.context_id);
        println!("Entities processed: {}", graph.metadata.total_entities);
        println!("Entities optimized: {}", entities_optimized);
        println!("Tokens generated: {}", tokens_generated);
        println!("Optimization ratio: {:.2}%", optimization_ratio * 100.0);
        println!("Confidence score: {:.2}", response.confidence_score);
        println!("Output file: {}", output_path);
        println!("Generation time: {:?}", generation_time);

        // Update statistics
        self.update_stats(graph.metadata.total_entities, tokens_generated, optimization_ratio, generation_time);

        Ok(ContextResult {
            context_id: response.context_metadata.context_id,
            output_path: output_path.to_string(),
            entities_processed: graph.metadata.total_entities,
            entities_optimized,
            tokens_generated,
            optimization_ratio,
            generation_time,
            errors,
        })
    }

    async fn generate_context_for_entities(&self, entities: &[CodeEntity], output_path: &str) -> Result<ContextResult> {
        let start_time = Instant::now();
        let mut errors = Vec::new();

        // Create simple relationships for provided entities
        let relationships = entities.iter().enumerate().map(|(i, entity)| {
            EntityRelationship {
                source_id: entity.isgl1_key.clone(),
                target_id: entity.isgl1_key.clone(),
                relationship_type: RelationshipType::RelatedTo,
                strength: 1.0,
                context: None,
            }
        }).collect();

        // Create context optimization request
        let request = ContextOptimizationRequest {
            entities: entities.to_vec(),
            relationships,
            target_context_size: self.config.max_context_tokens,
            focus_areas: vec!["user_provided".to_string()],
            optimization_goals: vec![OptimizationGoal::MaximizeRelevance],
        };

        // Generate optimized context
        let response = self.llm_client.optimize_context(request).await.map_err(|e| {
            let error_msg = format!("Context optimization failed: {}", e);
            errors.push(error_msg.clone());
            e
        })?;

        // Write optimized context to file
        self.write_context_file(&response, output_path).await.map_err(|e| {
            let error_msg = format!("Failed to write context file: {}", e);
            errors.push(error_msg.clone());
            e
        })?;

        let generation_time = start_time.elapsed();
        let entities_optimized = response.optimized_entities.len();
        let tokens_generated = response.context_metadata.token_count;
        let optimization_ratio = response.pruning_summary.pruning_ratio;

        // Update statistics
        self.update_stats(entities.len(), tokens_generated, optimization_ratio, generation_time);

        Ok(ContextResult {
            context_id: response.context_metadata.context_id,
            output_path: output_path.to_string(),
            entities_processed: entities.len(),
            entities_optimized,
            tokens_generated,
            optimization_ratio,
            generation_time,
            errors,
        })
    }

    fn get_stats(&self) -> ContextOptimizerStats {
        self.stats.lock().unwrap_or_else(|poisoned| poisoned.into_inner()).clone()
    }
}

/// Factory for creating context optimizers
pub struct ContextOptimizerFactory;

impl ContextOptimizerFactory {
    /// Create new context optimizer instance
    pub fn new(
        config: crate::ContextWriterConfig,
        llm_client: Arc<ContextLlmClientImpl>,
    ) -> Arc<ContextOptimizerImpl> {
        Arc::new(ContextOptimizerImpl::new(config, llm_client))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_context_generation() {
        let config = crate::ContextWriterConfig::default();
        let llm_client = crate::ToolFactory::create_llm_client(config.clone());
        let optimizer = ContextOptimizerImpl::new(config, llm_client);

        let temp_dir = tempfile::tempdir().unwrap();
        let output_path = temp_dir.path().join("test_context.json").to_string_lossy().to_string();

        // Note: This test would require a mock LLM client for real testing
        // For now, it demonstrates the interface
        let _result = optimizer.generate_context(&output_path).await;
    }

    #[test]
    fn test_graph_analysis() {
        let config = crate::ContextWriterConfig::default();
        let llm_client = crate::ToolFactory::create_llm_client(config.clone());
        let optimizer = ContextOptimizerImpl::new(config, llm_client);

        let entities = optimizer.create_sample_entities().unwrap();
        let relationships = optimizer.create_sample_relationships(&entities);

        let density = optimizer.calculate_graph_density(&entities, &relationships);
        assert!(density >= 0.0 && density <= 1.0);

        let avg_degree = optimizer.calculate_average_degree(&entities, &relationships);
        assert!(avg_degree >= 0.0);
    }

    #[test]
    fn test_statistics_tracking() {
        let config = crate::ContextWriterConfig::default();
        let llm_client = crate::ToolFactory::create_llm_client(config.clone());
        let optimizer = ContextOptimizerImpl::new(config, llm_client);

        let initial_stats = optimizer.get_stats();
        assert_eq!(initial_stats.contexts_generated, 0);

        optimizer.update_stats(10, 1000, 0.3, Duration::from_secs(5));
        let updated_stats = optimizer.get_stats();
        assert_eq!(updated_stats.contexts_generated, 1);
        assert_eq!(updated_stats.entities_processed, 10);
        assert_eq!(updated_stats.tokens_generated, 1000);
    }
}