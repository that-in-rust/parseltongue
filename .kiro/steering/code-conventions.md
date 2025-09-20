---
inclusion: always
---

# Idiomatic Rust Code Conventions

## THE ESSENCE

**Write Rust code that compiles correctly on the first try by leveraging the type system to prevent bugs at compile time.**

*"Once the code compiles, developers can have a high degree of confidence that it is free from data races, null pointer dereferences, and a host of other pernicious issues that have plagued systems programming for decades."*

## THE THREE PILLARS

### 1. **SAFETY THROUGH TYPES**
Make invalid states unrepresentable using Rust's type system

### 2. **ZERO-COST ABSTRACTIONS** 
Use high-level patterns that compile to efficient machine code

### 3. **FEARLESS CONCURRENCY**
Extend ownership and borrowing rules to multi-threaded contexts

---

## LAYER 1: FUNDAMENTAL PATTERNS

### The Ownership Model (Core Rules)
1. **Each value has a single owner**
2. **Only one owner at a time**  
3. **When owner goes out of scope, value is dropped**

### Borrowing Rules (Data Race Prevention)
1. **Either one mutable reference (`&mut T`) OR any number of immutable references (`&T`)**
2. **Never both simultaneously**

### API Design Strategy
- **Accept**: `&str`, `&[T]` (borrowed slices for maximum flexibility)
- **Store**: `String`, `Vec<T>` (owned types in structs to avoid lifetime complexity)
- **Return**: `String`, `Vec<T>` (owned types to transfer ownership cleanly)

### Error Handling Philosophy
- **Libraries**: `thiserror` for structured, matchable errors that consumers can handle programmatically
- **Applications**: `anyhow` for contextual error chains with human-readable context
- **Propagation**: `?` operator for clean error bubbling without verbose match blocks

### Type Safety Through Design
- **Parse, Don't Validate**: Newtype pattern (`UserId(Uuid)`, `Email(String)`) with validation in constructors
- **Make Invalid States Unrepresentable**: Enums to model only valid states
- **Compile-Time Guarantees**: Move validation from runtime to type system

---

## LAYER 2: CORE IMPLEMENTATION PATTERNS

### Smart Pointer Decision Matrix

| Scenario | Single-Threaded | Multi-Threaded | Use Case |
|----------|------------------|----------------|----------|
| **Unique Ownership** | `Box<T>` | `Box<T>` | Heap allocation, trait objects |
| **Shared Ownership** | `Rc<T>` | `Arc<T>` | Multiple owners, reference counting |
| **Interior Mutability** | `RefCell<T>` | `Mutex<T>` / `RwLock<T>` | Modify through shared reference |
| **Combined** | `Rc<RefCell<T>>` | `Arc<Mutex<T>>` | Shared mutable state |

### Send and Sync Traits (Auto-Implemented)
- **Send**: Safe to transfer ownership to another thread
- **Sync**: Safe to share references across threads  
- **Auto Traits**: Compiler automatically implements for structs if all fields are Send/Sync

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

### Builder Pattern Variations
```rust
// ✅ Standard Builder (consuming methods)
impl MessageBuilder {
    pub fn content(mut self, content: String) -> Self {
        self.content = Some(content);
        self
    }
    
    pub fn build(self) -> Result<Message, BuildError> {
        // Validate and construct
    }
}

// ✅ Type-State Builder (compile-time validation)
pub struct MessageBuilder<State> {
    content: String,
    _state: PhantomData<State>,
}

pub struct HasContent;
pub struct NoContent;

impl MessageBuilder<NoContent> {
    pub fn content(self, content: String) -> MessageBuilder<HasContent> {
        MessageBuilder {
            content,
            _state: PhantomData,
        }
    }
}

impl MessageBuilder<HasContent> {
    pub fn build(self) -> Message {
        // Can only build when content is set
        Message { content: self.content }
    }
}
```

### Extension Traits for API Ergonomics
```rust
// ✅ Add methods to existing types
pub trait StringExt {
    fn is_email(&self) -> bool;
    fn extract_mentions(&self) -> Vec<&str>;
}

impl StringExt for str {
    fn is_email(&self) -> bool {
        self.contains('@') && self.contains('.')
    }
    
    fn extract_mentions(&self) -> Vec<&str> {
        self.split_whitespace()
            .filter_map(|word| word.strip_prefix('@'))
            .collect()
    }
}
```

### Sealed Traits (Prevent External Implementation)
```rust
// ✅ Control trait implementation
mod sealed {
    pub trait Sealed {}
}

pub trait ProcessingState: sealed::Sealed {
    fn process(&self) -> String;
}

pub struct Pending;
pub struct Complete;

impl sealed::Sealed for Pending {}
impl sealed::Sealed for Complete {}

impl ProcessingState for Pending {
    fn process(&self) -> String { "processing...".to_string() }
}

impl ProcessingState for Complete {
    fn process(&self) -> String { "done".to_string() }
}
```

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

### Memory Layout Optimization
```rust
// ✅ Packed structures for cache efficiency
#[repr(packed)]
pub struct PackedMessage {
    id: u64,
    timestamp: u32,
    user_id: u32,
}

// ✅ Alignment for SIMD operations
#[repr(align(16))]
pub struct AlignedBuffer {
    data: [u8; 64],
}

// ✅ Small string optimization pattern
pub enum SmallString {
    Inline([u8; 23], u8), // 23 bytes + length
    Heap(String),
}
```

### Arena Allocation for Batch Processing
```rust
// ✅ Allocate many objects in single allocation
use typed_arena::Arena;

pub fn process_batch(messages: &[RawMessage]) -> Vec<ProcessedMessage> {
    let arena = Arena::new();
    
    messages.iter()
        .map(|raw| {
            let processed = arena.alloc(ProcessedMessage::from(raw));
            // All arena allocations freed together at end of scope
            processed.clone()
        })
        .collect()
}
```

### Lock-Free Patterns
```rust
// ✅ Atomic operations for high-performance concurrent access
use std::sync::atomic::{AtomicU64, Ordering};

pub struct Counter {
    value: AtomicU64,
}

impl Counter {
    pub fn increment(&self) -> u64 {
        self.value.fetch_add(1, Ordering::Relaxed)
    }
    
    pub fn compare_and_swap(&self, current: u64, new: u64) -> Result<u64, u64> {
        self.value.compare_exchange(current, new, Ordering::SeqCst, Ordering::SeqCst)
    }
}
```

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

## LAYER 5: ASYNC AND I/O PATTERNS

### Stream Processing
```rust
// ✅ Async stream transformation
use tokio_stream::{Stream, StreamExt};

pub fn message_stream(
    room_id: RoomId,
) -> impl Stream<Item = Result<Message, StreamError>> {
    async_stream::stream! {
        let mut receiver = subscribe_to_room(room_id).await?;
        
        while let Some(event) = receiver.recv().await {
            match event {
                RoomEvent::NewMessage(msg) => yield Ok(msg),
                RoomEvent::Error(e) => yield Err(StreamError::from(e)),
            }
        }
    }
}
```

### Backpressure Handling
```rust
// ✅ Bounded channels prevent memory exhaustion
use tokio::sync::mpsc;

pub async fn rate_limited_processor() {
    let (tx, mut rx) = mpsc::channel::<Message>(100); // Bounded to 100
    
    // Producer will block when channel is full
    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            // Process with natural backpressure
            process_message(msg).await;
        }
    });
}
```

### Zero-Copy I/O Operations
```rust
// ✅ Avoid unnecessary allocations in I/O
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn efficient_copy(
    mut reader: impl AsyncReadExt + Unpin,
    mut writer: impl AsyncWriteExt + Unpin,
) -> io::Result<u64> {
    let mut buffer = [0u8; 8192];
    let mut total = 0;
    
    loop {
        let n = reader.read(&mut buffer).await?;
        if n == 0 { break; }
        
        writer.write_all(&buffer[..n]).await?;
        total += n as u64;
    }
    
    Ok(total)
}
```

### Database Connection Pooling
```rust
// ✅ Efficient database resource management
#[derive(Clone)]
pub struct Database {
    pool: sqlx::PgPool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = sqlx::PgPoolOptions::new()
            .max_connections(20)
            .min_connections(5)
            .acquire_timeout(Duration::from_secs(30))
            .connect(database_url)
            .await?;
            
        // Run migrations automatically
        sqlx::migrate!("./migrations").run(&pool).await?;
        
        Ok(Self { pool })
    }
}

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

### Test Organization Patterns
```rust
// ✅ Module-level test organization
#[cfg(test)]
mod tests {
    use super::*;
    
    mod unit_tests {
        use super::*;
        
        #[test]
        fn test_message_validation() {
            // Unit tests for individual functions
        }
    }
    
    mod integration_tests {
        use super::*;
        
        #[tokio::test]
        async fn test_message_flow() {
            // Integration tests for component interaction
        }
    }
    
    mod property_tests {
        use super::*;
        use proptest::prelude::*;
        
        proptest! {
            #[test]
            fn message_roundtrip(content in ".*") {
                // Property-based tests for invariants
            }
        }
    }
}
```

### Mock and Test Doubles
```rust
// ✅ Trait-based mocking for testability
#[async_trait]
pub trait MessageRepository {
    async fn save(&self, message: &Message) -> Result<(), RepoError>;
    async fn find_by_id(&self, id: MessageId) -> Result<Option<Message>, RepoError>;
}

// Production implementation
pub struct SqlMessageRepository {
    pool: sqlx::PgPool,
}

// Test implementation
pub struct MockMessageRepository {
    messages: Arc<Mutex<HashMap<MessageId, Message>>>,
}

#[async_trait]
impl MessageRepository for MockMessageRepository {
    async fn save(&self, message: &Message) -> Result<(), RepoError> {
        let mut messages = self.messages.lock().unwrap();
        messages.insert(message.id, message.clone());
        Ok(())
    }
    
    async fn find_by_id(&self, id: MessageId) -> Result<Option<Message>, RepoError> {
        let messages = self.messages.lock().unwrap();
        Ok(messages.get(&id).cloned())
    }
}
```

### Async Testing Patterns
```rust
// ✅ Time manipulation in tests
#[tokio::test]
async fn test_timeout_behavior() {
    tokio::time::pause(); // Pause time for deterministic testing
    
    let start = tokio::time::Instant::now();
    
    let result = tokio::time::timeout(
        Duration::from_secs(5),
        slow_operation()
    ).await;
    
    tokio::time::advance(Duration::from_secs(6)).await;
    
    assert!(result.is_err()); // Should timeout
    assert!(start.elapsed() >= Duration::from_secs(6));
}

// ✅ Concurrent testing
#[tokio::test]
async fn test_concurrent_access() {
    let counter = Arc::new(AtomicU64::new(0));
    let mut handles = vec![];
    
    for _ in 0..100 {
        let counter = Arc::clone(&counter);
        handles.push(tokio::spawn(async move {
            for _ in 0..1000 {
                counter.fetch_add(1, Ordering::Relaxed);
            }
        }));
    }
    
    for handle in handles {
        handle.await.unwrap();
    }
    
    assert_eq!(counter.load(Ordering::Relaxed), 100_000);
}
```

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

## LAYER 7: ADVANCED TYPE SYSTEM PATTERNS

### Type-State Programming
```rust
// ✅ Encode state transitions in the type system
pub struct Connection<State> {
    socket: TcpStream,
    _state: PhantomData<State>,
}

pub struct Disconnected;
pub struct Connected;
pub struct Authenticated;

impl Connection<Disconnected> {
    pub async fn connect(addr: SocketAddr) -> io::Result<Connection<Connected>> {
        let socket = TcpStream::connect(addr).await?;
        Ok(Connection {
            socket,
            _state: PhantomData,
        })
    }
}

impl Connection<Connected> {
    pub async fn authenticate(self, credentials: &Credentials) -> Result<Connection<Authenticated>, AuthError> {
        // Perform authentication
        // Only return Authenticated state on success
        Ok(Connection {
            socket: self.socket,
            _state: PhantomData,
        })
    }
}

impl Connection<Authenticated> {
    pub async fn send_message(&mut self, msg: &Message) -> io::Result<()> {
        // Can only send messages when authenticated
        self.socket.write_all(&msg.serialize()).await
    }
}
```

### Const Generics for Compile-Time Validation
```rust
// ✅ Array bounds checked at compile time
pub struct FixedRingBuffer<T, const N: usize> {
    data: [Option<T>; N],
    head: usize,
    tail: usize,
}

impl<T, const N: usize> FixedRingBuffer<T, N> {
    pub fn new() -> Self {
        const { assert!(N > 0, "Buffer size must be positive") };
        const { assert!(N.is_power_of_two(), "Buffer size must be power of 2") };
        
        Self {
            data: [const { None }; N],
            head: 0,
            tail: 0,
        }
    }
    
    pub fn push(&mut self, item: T) -> Result<(), T> {
        if self.is_full() {
            Err(item)
        } else {
            self.data[self.tail] = Some(item);
            self.tail = (self.tail + 1) % N;
            Ok(())
        }
    }
}
```

### Associated Types and GATs
```rust
// ✅ Generic Associated Types for flexible APIs
pub trait AsyncIterator {
    type Item;
    type Future<'a>: Future<Output = Option<Self::Item>> + 'a
    where
        Self: 'a;
    
    fn next(&mut self) -> Self::Future<'_>;
}

// ✅ Associated types for cleaner trait bounds
pub trait Repository {
    type Entity;
    type Error;
    type Query;
    
    async fn find(&self, query: Self::Query) -> Result<Vec<Self::Entity>, Self::Error>;
    async fn save(&self, entity: &Self::Entity) -> Result<(), Self::Error>;
}
```

### Phantom Types for Zero-Cost State
```rust
// ✅ Compile-time state tracking with zero runtime cost
use std::marker::PhantomData;

pub struct Validated;
pub struct Unvalidated;

pub struct UserInput<State = Unvalidated> {
    data: String,
    _state: PhantomData<State>,
}

impl UserInput<Unvalidated> {
    pub fn new(data: String) -> Self {
        Self {
            data,
            _state: PhantomData,
        }
    }
    
    pub fn validate(self) -> Result<UserInput<Validated>, ValidationError> {
        if self.data.len() > 0 && self.data.len() <= 1000 {
            Ok(UserInput {
                data: self.data,
                _state: PhantomData,
            })
        } else {
            Err(ValidationError::InvalidLength)
        }
    }
}

impl UserInput<Validated> {
    pub fn process(&self) -> ProcessedData {
        // Can only process validated input
        ProcessedData::from(&self.data)
    }
}
```

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

## LAYER 8: SERIALIZATION AND API PATTERNS

### Serde Best Practices
```rust
// ✅ Versioned serialization for API evolution
#[derive(Serialize, Deserialize)]
#[serde(tag = "version")]
pub enum MessageV1 {
    #[serde(rename = "1")]
    V1 {
        id: String,
        content: String,
        timestamp: u64,
    },
    #[serde(rename = "2")]
    V2 {
        id: Uuid,
        content: String,
        timestamp: DateTime<Utc>,
        metadata: HashMap<String, String>,
    },
}

// ✅ Custom serialization for performance
#[derive(Serialize, Deserialize)]
pub struct OptimizedMessage {
    #[serde(with = "uuid_as_string")]
    id: Uuid,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<HashMap<String, String>>,
    
    #[serde(default)]
    flags: MessageFlags,
}

mod uuid_as_string {
    use serde::{Deserialize, Deserializer, Serializer};
    use uuid::Uuid;
    
    pub fn serialize<S>(uuid: &Uuid, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&uuid.to_string())
    }
    
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Uuid, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Uuid::parse_str(&s).map_err(serde::de::Error::custom)
    }
}
```

### Into/From Conversion Patterns
```rust
// ✅ Ergonomic conversions between types
#[derive(Debug)]
pub struct UserId(Uuid);

impl From<Uuid> for UserId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<UserId> for Uuid {
    fn from(user_id: UserId) -> Self {
        user_id.0
    }
}

// ✅ Fallible conversions
impl TryFrom<String> for UserId {
    type Error = uuid::Error;
    
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Ok(Self(Uuid::parse_str(&s)?))
    }
}

// ✅ Generic Into parameters for flexibility
pub fn create_user(id: impl Into<UserId>, name: impl Into<String>) -> User {
    User {
        id: id.into(),
        name: name.into(),
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

---

## LAYER 9: WORKSPACE AND PROJECT ORGANIZATION

### Module Organization
```rust
// ✅ Clear module hierarchy
// src/lib.rs
pub mod models;
pub mod services;
pub mod repositories;
pub mod errors;

// Re-export commonly used types
pub use models::{User, Message, Room};
pub use errors::{AppError, Result};

// ✅ Prelude module for common imports
// src/prelude.rs
pub use crate::{
    models::{User, Message, Room},
    services::{UserService, MessageService},
    errors::{AppError, Result},
};

// ✅ Feature flags for optional functionality
// Cargo.toml
[features]
default = ["sqlite"]
sqlite = ["sqlx/sqlite"]
postgres = ["sqlx/postgres"]
redis-cache = ["redis"]
metrics = ["prometheus"]
```

### Conditional Compilation
```rust
// ✅ Platform-specific code
#[cfg(target_os = "linux")]
fn get_system_info() -> SystemInfo {
    // Linux-specific implementation
}

#[cfg(target_os = "windows")]
fn get_system_info() -> SystemInfo {
    // Windows-specific implementation
}

// ✅ Feature-gated functionality
#[cfg(feature = "metrics")]
pub mod metrics {
    pub fn record_request_duration(duration: Duration) {
        // Metrics implementation
    }
}

#[cfg(not(feature = "metrics"))]
pub mod metrics {
    pub fn record_request_duration(_duration: Duration) {
        // No-op implementation
    }
}
```

---

## DECISION FRAMEWORK

When writing Rust code, ask these questions in order:

### 1. **SAFETY FIRST**
- Can the type system prevent this bug? → Use newtypes, enums, const generics
- Can this fail? → Return `Result<T, E>`, use `?` operator
- Is this thread-safe? → Check Send/Sync bounds, use Arc/Mutex appropriately

### 2. **OWNERSHIP CLARITY**
- Who owns this data? → Accept `&T`, store owned types, return owned types
- Do I need shared ownership? → Use `Rc<T>` (single-thread) or `Arc<T>` (multi-thread)
- Do I need interior mutability? → Use `RefCell<T>` (single-thread) or `Mutex<T>` (multi-thread)

### 3. **PERFORMANCE CONSIDERATIONS**
- Is this zero-cost? → Prefer iterators, avoid unnecessary allocations
- Can this be computed at compile-time? → Use const generics, const functions
- Is memory layout optimal? → Consider `#[repr(packed)]` or `#[repr(align)]`

### 4. **API DESIGN**
- Is this ergonomic? → Use builder patterns, Into/From conversions
- Is this extensible? → Use traits, extension traits, sealed traits
- Is this testable? → Use dependency injection, trait abstractions

### 5. **MAINTAINABILITY**
- Are invariants encoded in types? → Use phantom types, type-state programming
- Is error handling comprehensive? → Use thiserror for libraries, anyhow for applications
- Are tests comprehensive? → Unit tests, integration tests, property-based tests

**Remember**: If it compiles and follows these patterns, it's likely correct, performant, and maintainable.