# Temporal Versioning Logic Specification

## Overview

The temporal versioning system is the core innovation of Parseltongue, enabling safe code modifications through time-based state tracking. This specification defines the exact logic for state transitions, consistency rules, conflict resolution, and validation procedures.

## State Model

### Temporal Indicators

Each entity in the CodeGraph has three temporal attributes:

1. **current_ind (boolean)**: Entity exists in the current state
2. **future_ind (boolean)**: Entity will exist in the future state
3. **Future_Action (string)**: Action to perform during state transition

### Valid State Combinations

| current_ind | future_ind | Future_Action | State Name | Description |
|-------------|------------|---------------|------------|-------------|
| 1 | 1 | null | **EXISTS_UNCHANGED** | Entity exists and continues unchanged |
| 1 | 0 | "Delete" | **EXISTS_DELETE** | Entity exists but will be deleted |
| 0 | 1 | "Create" | **CREATE_EXISTS** | Entity doesn't exist but will be created |
| 1 | 1 | "Edit" | **EXISTS_MODIFIED** | Entity exists and will be modified |

### Invalid State Combinations

| current_ind | future_ind | Future_Action | Reason |
|-------------|------------|---------------|---------|
| 0 | 0 | null | Entity doesn't exist and won't exist (no point storing) |
| 0 | 0 | any | Invalid temporal combination |
| 1 | 1 | "Create" | Can't create entity that already exists |
| 1 | 1 | "Delete" | Use current_ind=0, future_ind=0 for deletion |
| 0 | 1 | null | Future action required for creation |
| 1 | 0 | null | Future action required for deletion |
| 0 | 1 | "Edit" or "Delete" | Invalid actions for non-existent entity |

## State Transition Logic

### Create Operation

**Initial State**: Entity does not exist in database
**Target State**: `current_ind=0, future_ind=1, Future_Action="Create"`

**Prerequisites**:
- ISGL1_key must be unique
- Future_Code must be provided
- Interface signature must be extracted

**Transition Process**:
```rust
fn create_entity(
    isgl1_key: String,
    future_code: String,
    interface_signature: InterfaceSignature,
    tdd_classification: TDDClassification,
    lsp_metadata: Option<LspMetadata>
) -> Result<Entity, TemporalError> {
    // 1. Validate ISGL1 key uniqueness
    if entity_exists(&isgl1_key)? {
        return Err(TemporalError::EntityAlreadyExists(isgl1_key));
    }

    // 2. Create new entity with temporal state
    Ok(Entity {
        isgl1_key,
        current_code: None,           // No current code for new entity
        future_code: Some(future_code),
        current_ind: false,
        future_ind: true,
        future_action: Some("Create".to_string()),
        interface_signature,
        tdd_classification,
        lsp_metadata,
        // ... other metadata
    })
}
```

### Edit Operation

**Initial State**: `current_ind=1, future_ind=1, Future_Action=null`
**Target State**: `current_ind=1, future_ind=1, Future_Action="Edit"`

**Prerequisites**:
- Entity must exist in current state
- Future_Code must be provided
- Interface signature must be updated

**Transition Process**:
```rust
fn edit_entity(
    isgl1_key: String,
    future_code: String,
    updated_signature: InterfaceSignature
) -> Result<Entity, TemporalError> {
    // 1. Validate entity exists in current state
    let mut entity = get_entity(&isgl1_key)?;
    if !entity.current_ind {
        return Err(TemporalError::EntityNotExists(isgl1_key));
    }

    // 2. Validate entity is not already modified
    if entity.future_action.is_some() {
        return Err(TemporalError::EntityAlreadyModified(isgl1_key));
    }

    // 3. Apply edit operation
    entity.future_code = Some(future_code);
    entity.interface_signature = updated_signature;
    entity.future_action = Some("Edit".to_string());

    Ok(entity)
}
```

### Delete Operation

**Initial State**: `current_ind=1, future_ind=1, Future_Action=null`
**Target State**: `current_ind=1, future_ind=0, Future_Action="Delete"`

**Prerequisites**:
- Entity must exist in current state
- No dependent entities can reference this entity
- Future_Code must be set to null

**Transition Process**:
```rust
fn delete_entity(isgl1_key: String) -> Result<Entity, TemporalError> {
    // 1. Validate entity exists in current state
    let mut entity = get_entity(&isgl1_key)?;
    if !entity.current_ind {
        return Err(TemporalError::EntityNotExists(isgl1_key));
    }

    // 2. Check for active dependencies
    let dependents = get_dependent_entities(&isgl1_key)?;
    if !dependents.is_empty() {
        return Err(TemporalError::HasActiveDependents {
            entity: isgl1_key,
            dependents,
        });
    }

    // 3. Apply delete operation
    entity.future_ind = false;
    entity.future_action = Some("Delete".to_string());
    entity.future_code = None;

    Ok(entity)
}
```

## State Application Process

### Phase 4: Apply Changes (LLM-cozoDB-to-code-writer)

The state application process converts future state to actual file changes:

```rust
fn apply_temporal_changes() -> Result<Vec<FileOperation>, TemporalError> {
    let mut operations = Vec::new();

    // 1. Get all entities with future actions
    let changed_entities = query_changed_entities()?;

    // 2. Sort entities by dependency order
    let sorted_entities = sort_by_dependencies(changed_entities)?;

    // 3. Apply changes in dependency order
    for entity in sorted_entities {
        match entity.future_action.as_deref() {
            Some("Create") => {
                operations.push(FileOperation::CreateFile {
                    path: extract_file_path(&entity.isgl1_key),
                    content: entity.future_code.unwrap(),
                });
            }
            Some("Edit") => {
                operations.push(FileOperation::ModifyFile {
                    path: extract_file_path(&entity.isgl1_key),
                    content: entity.future_code.unwrap(),
                    entity_name: extract_entity_name(&entity.isgl1_key),
                });
            }
            Some("Delete") => {
                operations.push(FileOperation::DeleteEntity {
                    path: extract_file_path(&entity.isgl1_key),
                    entity_name: extract_entity_name(&entity.isgl1_key),
                });
            }
            _ => return Err(TemporalError::InvalidFutureAction),
        }
    }

    Ok(operations)
}
```

### File Operation Types

```rust
enum FileOperation {
    CreateFile { path: PathBuf, content: String },
    ModifyFile {
        path: PathBuf,
        content: String,
        entity_name: String
    },
    DeleteEntity {
        path: PathBuf,
        entity_name: String
    },
}
```

## Phase 6: State Reset Process

### Reset Logic (cozoDB-make-future-code-current)

The reset process makes the future state the new current state:

```rust
fn reset_temporal_state() -> Result<(), TemporalError> {
    // 1. Get all entities with future actions
    let changed_entities = query_changed_entities()?;

    // 2. Apply state transitions
    for entity in changed_entities {
        let mut updated_entity = entity.clone();

        match entity.future_action.as_deref() {
            Some("Create") => {
                // Create → Exists: set current_ind=1, clear future action
                updated_entity.current_ind = true;
                updated_entity.current_code = entity.future_code.clone();
                updated_entity.future_action = None;
            }
            Some("Edit") => {
                // Exists → Modified: update current_code, clear future action
                updated_entity.current_code = entity.future_code.clone();
                updated_entity.future_action = None;
            }
            Some("Delete") => {
                // Exists → Delete: remove entity entirely
                delete_entity_from_database(&entity.isgl1_key)?;
                continue; // Skip update for deleted entities
            }
            _ => return Err(TemporalError::InvalidFutureAction),
        }

        // 3. Update entity in database
        update_entity_in_database(updated_entity)?;
    }

    // 4. Delete CodeGraph table and re-index
    delete_codegraph_table()?;
    trigger_reindexing()?;

    Ok(())
}
```

## Consistency Validation

### Temporal Consistency Rules

```rust
fn validate_temporal_consistency() -> Result<Vec<ConsistencyError>, TemporalError> {
    let mut errors = Vec::new();

    // 1. Check for invalid state combinations
    let invalid_states = query_invalid_temporal_states()?;
    for entity in invalid_states {
        errors.push(ConsistencyError::InvalidTemporalState {
            isgl1_key: entity.isgl1_key,
            current_ind: entity.current_ind,
            future_ind: entity.future_ind,
            future_action: entity.future_action,
        });
    }

    // 2. Check for orphaned dependencies
    let orphaned_deps = find_orphaned_dependencies()?;
    for dep in orphaned_deps {
        errors.push(ConsistencyError::OrphanedDependency {
            dependent: dep.dependent,
            dependency: dep.dependency,
        });
    }

    // 3. Check for dependency cycles
    let cycles = find_dependency_cycles()?;
    for cycle in cycles {
        errors.push(ConsistencyError::DependencyCycle { cycle });
    }

    Ok(errors)
}
```

### Dependency Consistency

**Rules for Dependency Management**:

1. **Creating Entities**: Can depend on existing entities or other entities being created
2. **Editing Entities**: Can modify dependencies to any existing entities
3. **Deleting Entities**: Cannot delete entities that have active dependents
4. **Modified Dependencies**: If dependency is modified, dependents must be revalidated

```rust
fn validate_dependency_consistency(
    entity: &Entity,
    operation: &TemporalOperation
) -> Result<(), DependencyError> {
    match operation {
        TemporalOperation::Create => {
            // New entities can depend on existing or other new entities
            validate_dependencies_exist_or_being_created(&entity.dependencies)?;
        }
        TemporalOperation::Edit => {
            // Edited entities can only depend on existing entities
            validate_dependencies_exist(&entity.dependencies)?;
        }
        TemporalOperation::Delete => {
            // Cannot delete entities with active dependents
            let dependents = get_active_dependents(&entity.isgl1_key)?;
            if !dependents.is_empty() {
                return Err(DependencyError::HasActiveDependents {
                    entity: entity.isgl1_key.clone(),
                    dependents,
                });
            }
        }
    }

    Ok(())
}
```

## Conflict Resolution

### Concurrent Modification Detection

```rust
fn detect_concurrent_modifications(
    proposed_changes: &[Entity]
) -> Result<Vec<Conflict>, ConflictError> {
    let mut conflicts = Vec::new();

    for proposed in proposed_changes {
        // Check if entity was modified since last read
        let current_entity = get_current_entity(&proposed.isgl1_key)?;

        if let Some(current) = current_entity {
            if current.last_modified > proposed.read_timestamp {
                conflicts.push(Conflict::ConcurrentModification {
                    isgl1_key: proposed.isgl1_key.clone(),
                    current_version: current.last_modified,
                    read_version: proposed.read_timestamp,
                });
            }
        }
    }

    Ok(conflicts)
}
```

### Resolution Strategies

1. **Fail Fast**: Reject entire operation if conflicts detected (MVP approach)
2. **Merge Strategies**: For future enhancement (automatic merging)
3. **User Resolution**: Prompt user to resolve conflicts manually

```rust
fn resolve_conflicts(conflicts: &[Conflict]) -> Result<(), ConflictError> {
    if !conflicts.is_empty() {
        return Err(ConflictError::UnresolvableConflicts {
            conflicts: conflicts.to_vec(),
            message: "Concurrent modifications detected. Please refresh and retry.".to_string(),
        });
    }
    Ok(())
}
```

## Performance Optimizations

### Query Optimization

```rust
// Efficient query for changed entities
const QUERY_CHANGED_ENTITIES: &str = r#"
    ?[isgl1_key, current_code, future_code, future_action] :=
        *CodeGraph{
            isgl1_key,
            current_code,
            future_code,
            future_ind: true,
            future_action
        },
        future_action != null
"#;

// Efficient query for dependency validation
const QUERY_DEPENDENT_ENTITIES: &str = r#"
    ?[dependent_isgl1] :=
        *Dependencies{
            dependency_isgl1: $target_entity,
            dependent_isgl1,
            current_ind: true
        }
"#;
```

### Batch Operations

```rust
fn batch_update_entities(entities: Vec<Entity>) -> Result<(), TemporalError> {
    // Use CozoDB transaction for atomic batch updates
    let transaction = cozo_db.transaction()?;

    for entity in entities {
        validate_entity_consistency(&entity)?;
        transaction.update_entity(entity)?;
    }

    transaction.commit()?;
    Ok(())
}
```

## Error Handling

### Error Types

```rust
#[derive(Debug, thiserror::Error)]
pub enum TemporalError {
    #[error("Entity already exists: {0}")]
    EntityAlreadyExists(String),

    #[error("Entity does not exist: {0}")]
    EntityNotExists(String),

    #[error("Entity already modified: {0}")]
    EntityAlreadyModified(String),

    #[error("Entity has active dependents: {entity:?}, dependents: {dependents:?}")]
    HasActiveDependents { entity: String, dependents: Vec<String> },

    #[error("Invalid temporal state: {entity:?}")]
    InvalidTemporalState { entity: Entity },

    #[error("Invalid future action: {0}")]
    InvalidFutureAction(String),

    #[error("Dependency cycle detected: {cycle:?}")]
    DependencyCycle { cycle: Vec<String> },

    #[error("Concurrent modification conflict: {conflicts:?}")]
    ConcurrentModification { conflicts: Vec<Conflict> },
}
```

### Recovery Strategies

1. **Transaction Rollback**: Rollback entire operation on any error
2. **State Reset**: Use Tool 6 to reset to known good state
3. **Partial Recovery**: Attempt to recover from non-critical errors
4. **User Notification**: Clear error messages with actionable guidance

## MVP Constraints

Following ultra-minimalist principles for MVP (~10 users):

1. **Simple State Model**: Only 4 valid state combinations
2. **Fail-Fast Conflict Resolution**: No automatic merging or conflict resolution
3. **Basic Dependency Checking**: Essential dependency validation only
4. **Single Transaction**: All operations in single database transaction
5. **Complete Reset**: Reset entire CodeGraph on state application (Tool 6)

This temporal versioning specification provides a robust foundation for safe code modifications while maintaining simplicity and reliability for the MVP target audience.