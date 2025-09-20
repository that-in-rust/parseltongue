---
inclusion: always
---

# Campfire-on-Rust: Code Conventions

## CORE PRINCIPLES

- **Compile-First Success**: Write idiomatic patterns that leverage Rust's type system
- **Zero-Cost Abstractions**: Use high-level patterns that compile to efficient code
- **Explicit Over Implicit**: Make error conditions and ownership clear in types
- **Safety Through Types**: Use the type system to prevent bugs at compile time

## OWNERSHIP AND BORROWING PATTERNS

### Function Signatures
```rust
// ✅ Accept slices for maximum flexibility
fn process_content(content: &str) -> ProcessedContent { }
fn analyze_messages(messages: &[Message]) -> Analysis { }

// ✅ Return owned types to transfer ownership
fn create_message(content: String) -> Message { }

// ❌ Avoid taking ownership unless consuming
fn bad_process(content: String) -> ProcessedContent { } // Forces caller to clone
```

### Struct Fields
```rust
// ✅ Use owned types in structs to avoid lifetime complexity
pub struct Message {
    pub id: MessageId,
    pub content: String,        // Owned, not &str
    pub room_id: RoomId,
    pub created_at: DateTime<Utc>,
}

// ❌ Avoid references in structs unless absolutely necessary
pub struct BadMessage<'a> {
    pub content: &'a str,      // Complicates usage
}
```

## ERROR HANDLING PATTERNS

### Library vs Application Errors
```rust
// ✅ Use thiserror for library errors (structured)
#[derive(Error, Debug)]
pub enum CampfireError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Validation error: {field} - {message}")]
    Validation { field: String, message: String },
}

// ✅ Use anyhow for application errors (contextual)
pub async fn send_webhook(url: &str) -> anyhow::Result<()> {
    let response = reqwest::get(url)
        .await
        .with_context(|| format!("Failed to connect to {}", url))?;
    Ok(())
}
```

### Option and Result Combinators
```rust
// ✅ Use combinators over explicit matching
fn extract_mentions(&self) -> Vec<UserId> {
    self.content
        .split_whitespace()
        .filter_map(|word| {
            word.strip_prefix('@')
                .and_then(|username| self.room.find_user_by_name(username))
        })
        .collect()
}

// ✅ Chain operations safely
let processed = input
    .filter(|s| !s.trim().is_empty())
    .map(|s| s.trim().to_string())
    .and_then(|content| validate_content(&content).ok())
    .map(ProcessedMessage::new);
```

## TYPE SAFETY PATTERNS

### Newtype Pattern for Domain Safety
```rust
// ✅ Prevent ID confusion with newtypes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UserId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RoomId(pub Uuid);

// ✅ Validation in constructor
impl Email {
    pub fn new(s: String) -> Result<Self, ValidationError> {
        if s.contains('@') && s.len() <= 254 {
            Ok(Self(s))
        } else {
            Err(ValidationError::InvalidEmail)
        }
    }
}
```

### Making Invalid States Unrepresentable
```rust
// ✅ Use enums to model state machines
#[derive(Debug, Clone)]
pub enum RoomType {
    Open,
    Closed { invited_users: Vec<UserId> },
    Direct { participants: [UserId; 2] }, // Exactly 2
}

#[derive(Debug)]
pub enum MessageState {
    Pending { client_id: Uuid },
    Sent { id: MessageId, timestamp: DateTime<Utc> },
    Failed { error: String, retry_count: u8 },
}
```

## ASYNC PATTERNS

### Structured Concurrency
```rust
// ✅ Use JoinSet for managing multiple tasks
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

### Resource Management
```rust
// ✅ Use RAII patterns for cleanup
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

## CONCURRENCY PATTERNS

### Shared State Management
```rust
// ✅ Arc<Mutex<T>> for shared mutable state
#[derive(Clone)]
pub struct ConnectionManager {
    connections: Arc<RwLock<HashMap<UserId, Vec<ConnectionId>>>>,
}

// ✅ Actor pattern for complex state management
pub enum RoomMessage {
    UserJoined { user_id: UserId, connection: ConnectionHandle },
    NewMessage { message: Message, sender: oneshot::Sender<Result<(), MessageError>> },
}
```

## DATABASE PATTERNS

### Query Safety
```rust
// ✅ Use sqlx::query! for compile-time validation
pub async fn get_user_messages(
    db: &Database,
    user_id: UserId,
    limit: u32,
) -> Result<Vec<Message>, sqlx::Error> {
    let messages = sqlx::query_as!(
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
    .await?;
    
    Ok(messages)
}
```

### Transaction Patterns
```rust
// ✅ Safe transaction handling with proper cleanup
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

## PERFORMANCE PATTERNS

### Memory Efficiency
```rust
// ✅ Use Cow for conditional ownership
use std::borrow::Cow;

pub fn normalize_content(content: &str) -> Cow<str> {
    if content.contains('\r') {
        Cow::Owned(content.replace('\r', ""))
    } else {
        Cow::Borrowed(content)
    }
}

// ✅ Zero-allocation string processing
pub fn extract_mentions(content: &str) -> impl Iterator<Item = &str> {
    content
        .split_whitespace()
        .filter_map(|word| word.strip_prefix('@'))
}
```

### Iterator Patterns
```rust
// ✅ Functional style with zero runtime cost
pub fn filter_visible_messages(
    messages: impl Iterator<Item = Message>,
    user_id: UserId,
) -> impl Iterator<Item = Message> {
    messages
        .filter(move |msg| msg.is_visible_to(user_id))
        .take(50)
}
```

## TESTING PATTERNS

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

// ✅ Property-based testing
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

## ANTI-PATTERNS TO AVOID

### Memory and Ownership
```rust
// ❌ Don't use .unwrap() in production
let value = risky_operation().unwrap(); // Can panic

// ✅ Handle errors properly
let value = risky_operation()
    .with_context(|| "Failed to perform risky operation")?;

// ❌ Don't clone unnecessarily
fn bad_process(data: Vec<String>) -> Vec<String> {
    data.clone() // Unnecessary allocation
}

// ✅ Use references when possible
fn good_process(data: &[String]) -> Vec<String> {
    data.iter().map(|s| s.to_uppercase()).collect()
}
```

### Error Handling
```rust
// ❌ Don't ignore Result types
let _ = risky_operation(); // Silently ignores errors

// ✅ Handle or propagate errors
risky_operation().with_context(|| "Context for debugging")?;
```

## COMPILE-TIME VALIDATION

### Static Assertions
```rust
// ✅ Validate assumptions at compile time
const _: () = assert!(std::mem::size_of::<MessageId>() == 16);

// ✅ Use const generics for compile-time constraints
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

## SUMMARY

These patterns leverage Rust's type system to:
- **Prevent bugs at compile time** through ownership and borrowing
- **Make invalid states unrepresentable** through careful type design
- **Provide zero-cost abstractions** that compile to efficient code
- **Handle errors explicitly** without runtime exceptions
- **Manage resources safely** through RAII and Drop

When complex patterns are suggested, respond with: "This violates our simplicity constraints. Here's the idiomatic Rust approach that leverages the type system for safety and performance..."