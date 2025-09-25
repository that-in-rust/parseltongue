# Refactor Planning Instructions

## Before Making Changes
1. Identify the target entity for refactoring
2. Run impact analysis: `./target/release/parseltongue_20250924231324 blast-radius EntityName`
3. Review all affected entities and their relationships
4. Plan changes in order of increasing risk

## Risk Assessment
- Low Risk (1-5 impacts): Standard refactoring approach
- Medium Risk (6-20 impacts): Requires comprehensive testing
- High Risk (21-50 impacts): Consider incremental approach
- Critical Risk (50+ impacts): Architectural review required

## Available Entities for Refactoring
- ✓ Loaded snapshot: 917 nodes, 1072 edges (1ms)
- 🛡️  PARSELTONGUE ENTITY SCAN COMPLETE
- ═══════════════════════════════════════
- 📊 Discovered 200 entities
- 
- 🔨 Function (52):
-   🎯 add (./src/discovery/performance_metrics.rs:0)
-   🎯 add_discovery_context (./src/discovery/error.rs:0)
-   🎯 add_failure (./src/discovery/performance_regression_tests.rs:0)
-   🎯 add_success (./src/discovery/performance_regression_tests.rs:0)
-   🎯 add_tag (src/models/post.rs:0)
-   🎯 add_tag_to_post (src/handlers/post_handlers.rs:0)
-   🎯 add_test_result (./src/accuracy_validation_report.rs:0)
-   🎯 all_file_paths (./src/discovery/concurrent_discovery_engine.rs:0)
-   🎯 analyze_blast_radius (./src/discovery/blast_radius_analyzer.rs:0)
-   🎯 analyze_feature_impact (./src/discovery/concrete_workflow_orchestrator.rs:0)
-   🎯 arbitrary_codebase_structure (./tests/comprehensive_integration_tests.rs:0)
-   🎯 arbitrary_discovery_query (./tests/comprehensive_integration_tests.rs:0)
-   🎯 arbitrary_entity_type (./tests/comprehensive_integration_tests.rs:0)
-   🎯 as_u32 (./src/discovery/string_interning.rs:0)
-   🎯 assess_refactoring_risks (./src/discovery/concrete_workflow_orchestrator.rs:0)
-   🎯 authenticate (src/services/user_service.rs:0)
-   🎯 available_entity_types (./src/discovery/simple_discovery_engine.rs:0)
-   🎯 available_types (./src/discovery/simple_discovery_engine.rs:0)
-   🎯 average_entities_per_file (./src/discovery/engine.rs:0)
-   🎯 batch_discovery_queries (./src/discovery/concurrent_discovery_engine.rs:0)
-   🎯 batch_discovery_queries_optimized (./src/discovery/concurrent_discovery_engine.rs:0)
-   🎯 batch_discovery_queries_pooled (./src/discovery/concurrent_discovery_engine.rs:0)
-   🎯 batch_discovery_queries_streaming (./src/discovery/concurrent_discovery_engine.rs:0)
-   🎯 batch_entities_by_types (./src/discovery/concurrent_discovery_engine.rs:0)
