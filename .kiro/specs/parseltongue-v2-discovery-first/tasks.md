# Implementation Plan

- [x] 1. Set up discovery infrastructure and core interfaces
  - Create discovery module structure with trait definitions
  - Implement string interning system for file paths (FileId, FileInterner)
  - Define DiscoveryEngine trait with async methods for entity exploration
  - Write unit tests for string interning efficiency and memory usage
  - _Requirements: 1.1, 1.2, 2.1_

- [x] 2. Enhance ISG node structure with file location attributes
  - Extend existing ISG node structure to include file_path, line_number, column fields
  - Implement EnhancedIsgNode with embedded file location data (not separate nodes)
  - Create conversion utilities between existing and enhanced node formats
  - Write unit tests validating O(1) file location access performance
  - _Requirements: 2.1, 2.2, 2.3_

- [x] 3. Implement simple entity listing (core constraint solver)
  - Create sorted entity list for simple browsing (all_entities: Vec<EntityInfo>)
  - Implement entity type filtering for focused discovery
  - Build efficient entity listing with pagination support
  - Write unit tests for complete entity enumeration
  - Write performance tests ensuring <100ms response time for entity listing
  - _Requirements: 1.1, 1.2 (MVP focus)_

- [x] 4. Create discovery query interface and command handlers
  - Implement DiscoveryEngine trait with list_all_entities, entities_in_file, where_defined methods
  - Create DiscoveryQuery enum for simple query types (ListAll, EntitiesInFile, WhereDefinedExact)
  - Build query result structures (EntityInfo, FileLocation) with proper serialization
  - Write integration tests for complete discovery workflows
  - _Requirements: 1.1, 1.2 (MVP focus)_

- [x] 5. Implement file-based entity navigation
  - Create file-to-entities index (HashMap<FileId, Vec<SigHash>>) for O(n) file queries
  - Implement entities_in_file query with entity type filtering
  - Build where_defined functionality returning exact file locations
  - Write unit tests validating file location accuracy and completeness
  - _Requirements: 2.2, 2.4, 2.5_

- [x] 6. Build entity type filtering and organization
  - Implement type_index for efficient entity type filtering
  - Create organized entity listing by type (Functions, Structs, Traits, etc.)
  - Add entity count summaries by type for overview
  - Write unit tests for type-based organization and filtering
  - _Requirements: 1.1, 1.2 (MVP focus)_

- [x] 7. Implement readable blast radius analysis
  - Create BlastRadiusAnalyzer with human-readable output (no hash values)
  - Implement ImpactGroup structure grouping by relationship type (CALLS, USES, IMPLEMENTS)
  - Build risk categorization logic (Low: 1-5, Medium: 6-20, High: 21-50, Critical: 50+)
  - Add separation of test files from production code in impact results
  - Write unit tests ensuring 100% readable output with proper file context
  - _Requirements: 3.1, 3.2, 3.3, 3.4, 3.5_

- [x] 8. Create simple discovery indexes
  - Implement DiscoveryIndexes struct with all_entities, file_index, type_index
  - Build CompactEntityInfo with optimized memory layout (24 bytes per entity)
  - Create efficient index rebuild mechanism for ISG updates
  - Write performance tests validating <5 second index rebuild for large codebases
  - _Requirements: 1.1, 1.2 (MVP focus)_

- [x] 9. Implement concurrent discovery engine with thread safety
  - Create ConcurrentDiscoveryEngine with Arc<RwLock<>> for thread-safe access
  - Implement read-optimized locking strategy for simple entity listing
  - Add efficient concurrent access to sorted entity lists
  - Write concurrency tests validating thread safety under concurrent load
  - _Requirements: Performance preservation constraint_

- [x] 10. Add comprehensive error handling and structured errors
  - Implement DiscoveryError enum with all possible failure conditions
  - Create context-rich error messages for CLI user experience
  - Add performance contract violation detection and reporting
  - Write error handling tests covering all error conditions
  - _Requirements: 1.1, 1.2, 1.3, 1.4, 1.5_

- [x] 11. Build CLI command interface for discovery operations
  - Implement CLI commands: list-entities, entities-in-file, where-defined
  - Add entity type filtering (--type) and result limiting (--limit) options
  - Create formatted output for discovery results with file locations
  - Write CLI integration tests for all discovery command variations
  - _Requirements: 1.1, 1.2 (MVP focus)_

- [x] 12. Implement performance monitoring and contract validation
  - Read thoroughly to keep in mind solving this task:.kiro/steering/design101-tdd-architecture-principles.md AND .kiro/steering/code-conventions.md AND .kiro/steering/A01-README-MOSTIMP.md
  - Create DiscoveryMetrics struct with Histogram and Counter metrics
  - Add automated performance contract validation (<100ms discovery, <50μs existing queries)
  - Implement memory usage monitoring with <20% increase constraint
  - Write performance regression tests for all critical paths
  - _Requirements: Performance preservation constraint, 1.1, 1.2, 1.3_



- [x] 14. Build comprehensive integration and end-to-end tests
  - Read thoroughly to keep in mind solving this task:.kiro/steering/design101-tdd-architecture-principles.md AND .kiro/steering/code-conventions.md AND .kiro/steering/A01-README-MOSTIMP.md
  - Create test scenarios covering discovery-to-analysis workflows
  - Implement property-based tests for discovery query invariants
  - Add stress tests with realistic codebase sizes (Iggy: 983 files, Axum: 295 files)
  - Write success metrics validation tests (discovery time <30s, success rate >90%)
  - _Requirements: All requirements validation_

- [x] 15. Optimize memory layout and performance critical paths
  - Read thoroughly to keep in mind solving this task:.kiro/steering/design101-tdd-architecture-principles.md AND .kiro/steering/code-conventions.md AND .kiro/steering/A01-README-MOSTIMP.md
  - Implement zero-allocation entity filtering with iterator patterns
  - Add batch processing for multiple discovery queries with bounded concurrency
  - Optimize string interning and trigram index memory usage
  - Write micro-benchmarks for all performance-critical operations
  - _Requirements: Performance preservation constraint, memory efficiency_

- [x] 16. Add JSON output support for tooling integration
  - Read thoroughly to keep in mind solving this task:.kiro/steering/design101-tdd-architecture-principles.md AND .kiro/steering/code-conventions.md AND .kiro/steering/A01-README-MOSTIMP.md
  - Implement `--json` flag for all discovery commands (list-entities, entities-in-file, where-defined)
  - Create structured JSON schemas with metadata (timestamps, file paths, confidence scores)
  - Add JSON output for blast-radius analysis with machine-readable impact data
  - Write unit tests validating JSON schema stability and backward compatibility
  - _Requirements: 5.1, 5.2, 5.3, 5.4, 5.5_

- [x] 17. Build workspace state management system
  - Read thoroughly to keep in mind solving this task:.kiro/steering/design101-tdd-architecture-principles.md AND .kiro/steering/code-conventions.md AND .kiro/steering/A01-README-MOSTIMP.md
  - Implement WorkspaceManager for persistent analysis sessions in `./parseltongue_workspace/`
  - Create AnalysisSession tracking with timestamps and automatic latest linking
  - Add workspace cleanup commands and stale analysis detection
  - Write integration tests for workspace isolation and state persistence
  - _Requirements: 6.1, 6.2, 6.3, 6.4, 6.5_

- [x] 18. Create workflow orchestration layer (shell toolkit)
  - Read thoroughly to keep in mind solving this task:.kiro/steering/design101-tdd-architecture-principles.md AND .kiro/steering/code-conventions.md AND .kiro/steering/A01-README-MOSTIMP.md
  - Build `pt` shell script with subcommand architecture (onboard, feature-start, debug, refactor-check)
  - Implement WorkflowOrchestrator trait combining discovery commands into complete user journeys
  - Create workflow result structures (OnboardingResult, FeaturePlanResult, DebugResult)
  - Write workflow integration tests validating complete JTBD user journeys
  - _Requirements: 4.1, 4.2, 4.3, 4.4, 4.5_

- [x] 19. Implement JTBD workflow commands
  - Read thoroughly to keep in mind solving this task:.kiro/steering/design101-tdd-architecture-principles.md AND .kiro/steering/code-conventions.md AND .kiro/steering/A01-README-MOSTIMP.md
  - Create `pt onboard` workflow: ingest → overview → routes → key contexts (complete in <15 minutes)
  - Build `pt feature-start` workflow: impact analysis → scope guidance → test recommendations
  - Implement `pt debug` workflow: caller traces → usage sites → minimal change scope
  - Add `pt refactor-check` workflow: risk assessment → change checklist → reviewer guidance
  - Write end-to-end tests for each workflow meeting success criteria timelines
  - _Requirements: JTBD 1, 2, 3, 4 workflows_

- [x] 20. Build output integration and formatting system
  - Read thoroughly to keep in mind solving this task:.kiro/steering/design101-tdd-architecture-principles.md AND .kiro/steering/code-conventions.md AND .kiro/steering/A01-README-MOSTIMP.md
  - Implement OutputFormatter trait with human, JSON, PR summary, and CI output formats
  - Create PR-ready markdown summaries with architectural context and impact analysis
  - Add CI/CD integration outputs with risk levels and actionable recommendations
  - Write formatting tests ensuring consistent, copy-pastable outputs across all formats
  - _Requirements: 5.1, 5.2, 5.3, 5.4, 5.5_

- [x] 21. Integration with existing ISG engine and final wiring
  - Read thoroughly to keep in mind solving this task:.kiro/steering/design101-tdd-architecture-principles.md AND .kiro/steering/code-conventions.md AND .kiro/steering/A01-README-MOSTIMP.md
  - Integrate discovery layer with existing InMemoryIsg without modifications
  - Wire discovery commands through existing CLI infrastructure
  - Connect workflow orchestration layer to core discovery primitives
  - Write full system integration tests validating complete feature functionality and workflows
  - _Requirements: All requirements integration and validation_



- [x] 22. Create comprehensive user journey documentation and demo materials
  - Make sure the final parseltongue binary has YYYYMMDDHHSS suffix to ensure we always know which version is which
  - Read thoroughly to keep in mind solving this task:.kiro/steering/design101-tdd-architecture-principles.md AND .kiro/steering/code-conventions.md AND .kiro/steering/A01-README-MOSTIMP.md
  - Create Demo 1: Axum codebase exploration journey using existing test data (zzzzArchive/_refTestDataAsLibraryTxt/tokio-rs-axum-8a5edab282632443.txt)
  - Create Demo 2: Self-exploration journey documenting Parseltongue's own codebase analysis
  - Document each workflow step with actual command outputs and timing measurements
  - Create parseltongue_dungeon/ folder with ready-to-use scripts and LLM instruction files
  - Update README.md with Mermaid diagrams using Minto Pyramid Principle (PMF features at top, details layered below)
  - Ensure all documentation uses low-drama, technical language focused on practical value
  - _Requirements: All requirements validation, user onboarding, workflow demonstration_

- [x] 23. Validate end-to-end system integration and performance contracts
  - Read thoroughly to keep in mind solving this task:.kiro/steering/design101-tdd-architecture-principles.md AND .kiro/steering/code-conventions.md AND .kiro/steering/A01-README-MOSTIMP.md
  - Run comprehensive end-to-end tests with realistic codebases - - Create Demo 1: Axum codebase exploration journey using existing test data (zzzzArchive/_refTestDataAsLibraryTxt/tokio-rs-axum-8a5edab282632443.txt)
  - Create Demo 2: Self-exploration journey documenting Parseltongue's own codebase analysis
  - Document each workflow step with actual command outputs and timing measurements
  - Validate all performance contracts: discovery <30s, queries <100ms, existing queries <50μs
  - Test all JTBD workflows meet timing requirements: onboard <15min, feature-start <5min, debug <2min, refactor-check <3min
  - Verify memory usage increase <20% from baseline ISG implementation
  - Document any performance contract violations and optimization recommendations
  - _Requirements: Performance preservation constraint, success metrics validation_


- [x] 24. Create a special LLM aligned document to work in .kiro as per your judgement + Use Parseltongue latest build to clean up the warnings in our code itself + when you quote loading time - avoid 0 seconds its confusing whenever it approximates to 0 then tell the milliseconds - ideally post in seconds + milliseconds as often as you can - it is impressive  -- do this using parseltongue_dungeon/scripts and then 
  - [x] Document all that you learn in previous task as a use case how you used the tool to improve the tool


- [-] 25. Using the Kiro steering document which uses pt parseltongue best practices, try to make some visually nice changes in the CLI experience putting emojis in the style of avengers theme, the idea is the workflows should replace some of the existing agentic workflows of search - or at least in part be better at helping navigate the codebase

- [ ] 26. Do amazing documentation using Mermaid diagrams - Use .kiro/trun_c928898c8ef7483eb8257cb7dc52ac9a.json +  .kiro/Bullet-Proof Mermaid Prompts_ Square-Perfect Diagrams from Any LLM.md -- And finally make an approrpirate commit and push to origin