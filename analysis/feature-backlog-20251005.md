# Parseltongue Feature Backlog
**Created:** 2025-10-05 12:10:00 UTC
**Type:** Missing CLI Commands
**Priority:** HIGH - Critical for User Workflows

## Overview

The following commands are referenced extensively in the original README workflows but are **not implemented** in the current version. These commands are essential for complete user workflows and should be prioritized for development.

## Missing Commands

### 1. List Entities Command 🔥 CRITICAL
**CLI Pattern:** `parseltongue list-entities [OPTIONS]`
**Impact:** Breaks all 4 main workflows in README
**Mentions in README:** 8+ times
**User Stories:**
- As a developer, I want to see all structs in the codebase
- As a developer, I want to filter entities by type (struct/trait/function)
- As a developer, I want to limit the number of results shown

**Proposed Implementation:**
```rust
ListEntities {
    /// Filter by entity type
    #[arg(long, value_enum)]
    r#type: Option<EntityType>,

    /// Limit number of results
    #[arg(long, default_value = "100")]
    limit: usize,

    /// Output format
    #[arg(long, default_value = "human")]
    format: OutputFormat,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum EntityType {
    Struct,
    Trait,
    Function,
    Impl,
    Module,
}
```

**Example Usage:**
```bash
parseltongue list-entities --type struct
parseltongue list-entities --type trait
parseltongue list-entities --type function --limit 20
parseltongue list-entities --format json
```

**Implementation Notes:**
- Iterate through all nodes in ISG
- Filter by NodeKind based on --type parameter
- Apply limit to results
- Format output as human-readable or JSON
- Should leverage existing isg.node_count() and iteration capabilities

### 2. Entities In File Command 🔥 HIGH
**CLI Pattern:** `parseltongue entities-in-file <FILE_PATH>`
**Impact:** Essential for file-based discovery workflows
**Mentions in README:** 3 times
**User Stories:**
- As a developer, I want to see all entities defined in a specific file
- As a developer, I want to understand file scope and responsibilities
- As a developer, I want to analyze file dependencies

**Proposed Implementation:**
```rust
EntitiesInFile {
    /// Path to the file to analyze
    file: PathBuf,

    /// Output format
    #[arg(long, default_value = "human")]
    format: OutputFormat,
}
```

**Example Usage:**
```bash
parseltongue entities-in-file src/lib.rs
parseltongue entities-in-file src/problem_file.rs
parseltongue entities-in-file src/main.rs --format json
```

**Implementation Notes:**
- Filter nodes by file_path matching the provided path
- Use existing NodeData.file_path field for filtering
- Return list of entities with their types and signatures
- Handle relative and absolute paths
- Validate file exists before processing

### 3. Where Defined Command 🔥 MEDIUM
**CLI Pattern:** `parseltongue where-defined <ENTITY_NAME>`
**Impact:** Important for navigation and code understanding
**Mentions in README:** 2 times
**User Stories:**
- As a developer, I want to find where a specific entity is defined
- As a developer, I want to quickly locate the source file and line number
- As a developer, I want to understand entity context

**Proposed Implementation:**
```rust
WhereDefined {
    /// Entity name to locate
    entity: String,

    /// Output format
    #[arg(long, default_value = "human")]
    format: OutputFormat,
}
```

**Example Usage:**
```bash
parseltongue where-defined UserStruct
parseltongue where-defined Display
parseltongue where-defined main --format json
```

**Implementation Notes:**
- Use existing find_entity_by_name() functionality
- Return file path, line number, and signature
- Handle multiple matches (show all)
- Provide clear location information
- Leverage existing name resolution logic

## Workflow Impact Analysis

### Current Broken Workflows
The missing commands break these README workflows:

1. **Workflow 1: Understand Unfamiliar Codebase**
   - ❌ `list-entities --type struct` - Missing
   - ❌ `list-entities --type trait` - Missing
   - ❌ `list-entities --type function` - Missing

2. **Workflow 2: Plan Feature Changes**
   - ❌ `list-entities --type function --limit 20` - Missing

3. **Workflow 3: Debug Without Breaking Things**
   - ❌ `entities-in-file src/problem_file.rs` - Missing
   - ❌ `where-defined UserStruct` - Missing

4. **File-Based Discovery Section**
   - ❌ `entities-in-file src/lib.rs` - Missing
   - ❌ `where-defined UserStruct` - Missing

### After Implementation Impact
Once these commands are implemented:
- ✅ All 4 workflows will be fully functional
- ✅ Users can complete the full developer experience
- ✅ README workflows will match actual capabilities
- ✅ User satisfaction will increase dramatically

## Technical Implementation Priorities

### Phase 1: Critical Foundation (Week 1)
1. **ListEntities Command** - Highest priority
   - Enables all other workflows
   - Most frequently mentioned in README
   - Builds on existing ISG iteration capabilities

### Phase 2: File Navigation (Week 2)
2. **EntitiesInFile Command**
   - Essential for file-based workflows
   - Simple filtering implementation
   - High user value

3. **WhereDefined Command**
   - Complements entities-in-file functionality
   - Uses existing entity resolution logic
   - Good utility command

### Phase 3: Enhanced Features (Week 3)
- Add advanced filtering options
- Implement pagination for large result sets
- Add file pattern matching
- Export capabilities

## Integration with Existing Codebase

### Reusable Components
- **ParseltongueAIM daemon** - Already has entity resolution
- **ISG graph structure** - Ready for node iteration and filtering
- **CLI framework** - clap is already configured and working
- **Output formatting** - Human/JSON format handling exists
- **Performance monitoring** - Timing infrastructure is in place

### Implementation Strategy
1. **Add command variants to Commands enum** in src/cli.rs
2. **Implement command handlers** following existing patterns
3. **Add tests** for each new command (following TDD approach)
4. **Update README** once commands are working
5. **Add integration tests** for complete workflows

## Testing Strategy

### Unit Tests
- Command parsing and validation
- Entity filtering logic
- File path handling
- Output formatting (human/JSON)

### Integration Tests
- End-to-end workflow testing
- Performance validation (<100ms response time)
- Error handling (invalid files, non-existent entities)
- Large codebase handling (1000+ entities)

### User Acceptance Tests
- Complete workflow execution
- README tutorial walkthrough
- Real-world codebase analysis

## Success Metrics

### Technical Metrics
- Commands respond in <100ms (current: <50μs for queries)
- Handle 1000+ entities without performance degradation
- Zero errors in unit/integration tests
- Memory usage <20MB for typical codebases

### User Experience Metrics
- All README workflows complete successfully
- Users can understand unfamiliar codebase in <15 minutes
- Zero confusion between documentation and reality
- Positive user feedback on workflow effectiveness

## Next Steps

1. **Immediate:** Update README to remove broken command references
2. **Week 1:** Implement list-entities command
3. **Week 2:** Implement entities-in-file and where-defined commands
4. **Week 3:** Update README with restored workflows
5. **Month 1:** Comprehensive user testing and validation

---

**Status:** Ready for Development
**Estimated Effort:** 2-3 weeks total implementation
**Risk Level:** LOW (builds on existing solid foundation)
**Business Impact:** HIGH (enables complete user workflows)

**Last Updated:** 2025-10-05 12:15:00 UTC