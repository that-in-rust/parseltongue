---
inclusion: always
---

# Idiomatic Rust Code Conventions

## THE ESSENCE

**Write Rust code that compiles correctly on the first try by leveraging the type system to prevent bugs at compile time.**

## THE THREE PILLARS

### 1. **SAFETY THROUGH TYPES**
Make invalid states unrepresentable using Rust's type system

### 2. **ZERO-COST ABSTRACTIONS** 
Use high-level patterns that compile to efficient machine code

### 3. **EXPLICIT ERROR HANDLING**
Make failure conditions visible in function signatures

---

## LAYER 1: FUNDAMENTAL PATTERNS

### Ownership Strategy
- **Accept**: `&str`, `&[T]` (borrowed slices for flexibility)
- **Store**: `String`, `Vec<T>` (owned types in structs)
- **Return**: `String`, `Vec<T>` (owned types to transfer ownership)

### Error Strategy
- **Libraries**: `thiserror` for structured, matchable errors
- **Applications**: `anyhow` for contextual error chains
- **Propagation**: `?` operator for clean error bubbling

### Type Safety Strategy
- **Domain Types**: Newtype pattern (`UserId(Uuid)`, `Email(String)`)
- **State Machines**: Enums to model valid states only
- **Validation**: Constructor functions that return `Result<T, E>`

---

## LAYER 2: CORE IMPLEMENTATION PATTERNS

### Function Signatures
```rust
// ✅ GOOD: Flexible input, clear ownership transfer
fn process_content(content: &str) -> ProcessedContent { }
fn create_message(content: String) -> Message { }

// ❌ AVOID: Forces unnecessary cloning
fn bad_process(content: String) -> ProcessedContent { }
```

### Error Handling
```rust
// ✅ Library errors: Structured and matchable
#[derive(Error, Debug)]
pub enum CampfireError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Validation failed: {field}")]
    Validation { field: String },
}

// ✅ Application errors: Contextual chains
pub async fn send_webhook(url: &str) -> anyhow::Result<()> {
    reqwest::get(url)
        .await
        .with_context(|| format!("Failed to connect to {}", url))?;
    Ok(())
}
```

### Type Safety
```rust
// ✅ Newtype pattern prevents ID confusion
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UserId(pub Uuid);

// ✅ State machine prevents invalid states
#[derive(Debug)]
pub enum MessageState {
    Pending { client_id: Uuid },
    Sent { id: MessageId, timestamp: DateTime<Utc> },
    Failed { error: String, retry_count: u8 },
}
```

---

## LAYER 3: ADVANCED PATTERNS

### Async Concurrency
```rust
// ✅ Structured concurrency with JoinSet
use tokio::task::JoinSet;

pub async fn process_batch(messages: Vec<Message>) -> Vec<Result<(), ProcessingError>> {
    let mut tasks = JoinSet::new();
    
    for message in messages {
        tasks.spawn(async move { process_message(message).await });
    }
    
    let mut results = Vec::new();
    while let Some(result) = tasks.join_next().await {
        results.push(result.unwrap_or_else(|e| Err(ProcessingError::TaskPanic(e.to_string()))));
    }
    results
}
```

### Resource Management (RAII)
```rust
// ✅ Automatic cleanup via Drop trait
pub struct ConnectionGuard {
    connection: Option<Connection>,
}

impl Drop for ConnectionGuard {
    fn drop(&mut self) {
        if let Some(conn) = self.connection.take() {
            if let Err(e) = conn.close() {
                eprintln!("Failed to close connection: {}", e);
            }
        }
    }
}
```

### Shared State
```rust
// ✅ Thread-safe shared mutable state
#[derive(Clone)]
pub struct ConnectionManager {
    connections: Arc<RwLock<HashMap<UserId, Vec<ConnectionId>>>>,
}
```

---

## LAYER 4: PERFORMANCE OPTIMIZATION

### Memory Efficiency
```rust
// ✅ Conditional ownership with Cow
use std::borrow::Cow;

pub fn normalize_content(content: &str) -> Cow<str> {
    if content.contains('\r') {
        Cow::Owned(content.replace('\r', ""))
    } else {
        Cow::Borrowed(content)
    }
}

// ✅ Zero-allocation iterator chains
pub fn extract_mentions(content: &str) -> impl Iterator<Item = &str> {
    content
        .split_whitespace()
        .filter_map(|word| word.strip_prefix('@'))
}
```

### Functional Style
```rust
// ✅ Iterator chains compile to efficient loops
pub fn filter_visible_messages(
    messages: impl Iterator<Item = Message>,
    user_id: UserId,
) -> impl Iterator<Item = Message> {
    messages
        .filter(move |msg| msg.is_visible_to(user_id))
        .take(50)
}
```

---

## LAYER 5: DATABASE AND I/O PATTERNS

### Query Safety
```rust
// ✅ Compile-time SQL validation with sqlx
pub async fn get_user_messages(
    db: &Database,
    user_id: UserId,
    limit: u32,
) -> Result<Vec<Message>, sqlx::Error> {
    sqlx::query_as!(
        Message,
        r#"
        SELECT id, content, room_id, creator_id, created_at
        FROM messages 
        WHERE creator_id = $1 
        ORDER BY created_at DESC 
        LIMIT $2
        "#,
        user_id.0,
        limit as i64
    )
    .fetch_all(&db.pool)
    .await
}
```

### Transaction Safety
```rust
// ✅ Proper transaction handling with cleanup
pub async fn create_room_with_membership(
    db: &Database,
    room: CreateRoomRequest,
    creator_id: UserId,
) -> Result<Room, CampfireError> {
    let mut tx = db.pool.begin().await?;
    
    let room = sqlx::query_as!(Room, "INSERT INTO rooms ...")
        .fetch_one(&mut *tx)
        .await?;
    
    sqlx::query!("INSERT INTO memberships ...")
        .execute(&mut *tx)
        .await?;
    
    tx.commit().await?;
    Ok(room)
}
```

---

## LAYER 6: TESTING STRATEGIES

### Contract Testing
```rust
// ✅ Test performance contracts
#[tokio::test]
async fn test_query_performance_contract() {
    let system = create_test_system().await;
    
    let start = Instant::now();
    let result = system.execute_query(test_query()).await.unwrap();
    let elapsed = start.elapsed();
    
    assert!(elapsed < Duration::from_millis(100), 
            "Query took {:?}, expected <100ms", elapsed);
}
```

### Property-Based Testing
```rust
// ✅ Test invariants across input space
use proptest::prelude::*;

proptest! {
    #[test]
    fn user_id_roundtrip(id in any::<u64>()) {
        let user_id = UserId(id);
        let serialized = serde_json::to_string(&user_id)?;
        let deserialized: UserId = serde_json::from_str(&serialized)?;
        prop_assert_eq!(user_id, deserialized);
    }
}
```

---

## LAYER 7: COMPILE-TIME VALIDATION

### Static Assertions
```rust
// ✅ Validate assumptions at compile time
const _: () = assert!(std::mem::size_of::<MessageId>() == 16);

// ✅ Const generics for compile-time constraints
pub struct FixedBuffer<const N: usize> {
    data: [u8; N],
}

impl<const N: usize> FixedBuffer<N> {
    pub fn new() -> Self {
        const { assert!(N > 0, "Buffer size must be positive") };
        Self { data: [0; N] }
    }
}
```

---

## ANTI-PATTERNS TO AVOID

### Critical Mistakes
```rust
// ❌ NEVER: Panic in production
let value = risky_operation().unwrap();

// ❌ NEVER: Ignore errors
let _ = risky_operation();

// ❌ NEVER: Unnecessary cloning
fn bad_process(data: Vec<String>) -> Vec<String> {
    data.clone()
}
```

### Better Alternatives
```rust
// ✅ Handle errors with context
let value = risky_operation()
    .with_context(|| "Failed to perform risky operation")?;

// ✅ Use references when possible
fn good_process(data: &[String]) -> Vec<String> {
    data.iter().map(|s| s.to_uppercase()).collect()
}
```

---

## DECISION FRAMEWORK

When writing Rust code, ask these questions in order:

1. **Can the type system prevent this bug?** → Use newtypes, enums, const generics
2. **Can this fail?** → Return `Result<T, E>`, use `?` operator
3. **Who owns this data?** → Accept `&T`, store owned types, return owned types
4. **Is this zero-cost?** → Prefer iterators, avoid unnecessary allocations
5. **Is this testable?** → Write contracts, use property-based testing

**Remember**: If it compiles and follows these patterns, it's likely correct and performant.