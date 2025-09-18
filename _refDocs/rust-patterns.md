# Rust Idiomatic Patterns for Campfire

## Core Philosophy: Compile-First Success

Follow the "vital 20%" of Rust patterns that enable 99% of production code with minimal bugs. Research shows proper idiomatic patterns reduce compile attempts from 4.9 to 1.6 average and eliminate 89% of production defects.

## L1: Core Language Patterns (Essential)

### 1. Ownership and Borrowing Mastery

```rust
// ✅ CORRECT: Prefer borrowing over ownership transfer
fn process_message_content(content: &str) -> ProcessedContent {
    // Read-only access, no ownership needed
    ProcessedContent::from_str(content)
}

fn modify_message_content(content: &mut String) {
    content.push_str(" - processed");
}

// ✅ CORRECT: RAII for automatic resource cleanup
struct DatabaseConnection {
    pool: sqlx::SqlitePool,
}

impl Drop for DatabaseConnection {
    fn drop(&mut self) {
        // Automatic cleanup when going out of scope
    }
}
```

### 2. Type-Driven Design with Newtypes

```rust
// ✅ MANDATORY: Use newtypes for all domain IDs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct UserId(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct RoomId(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct MessageId(pub i64);

// ✅ CORRECT: Make invalid states unrepresentable
#[derive(Debug, Clone)]
pub enum RoomType {
    Open,
    Closed { invited_users: Vec<UserId> },
    Direct { participants: [UserId; 2] }, // Exactly 2 participants
}
```

### 3. Zero-Cost Abstractions with Iterators

```rust
// ✅ CORRECT: Functional style with zero runtime cost
pub fn filter_visible_messages(
    messages: impl Iterator<Item = Message>,
    user_involvement: Involvement,
) -> impl Iterator<Item = Message> {
    messages
        .filter(move |msg| match user_involvement {
            Involvement::Everything => true,
            Involvement::Mentions => msg.has_mentions(),
            Involvement::Nothing => false,
        })
        .take(50) // Pagination limit
}

// ❌ WRONG: Manual loops with potential bugs
fn filter_messages_manual(messages: Vec<Message>) -> Vec<Message> {
    let mut result = Vec::new();
    for i in 0..messages.len() { // Potential bounds issues
        result.push(messages[i].clone()); // Unnecessary clones
    }
    result
}
```

## L2: Standard Library Patterns

### 1. Smart Pointer Decision Matrix

| Scenario | Single-Threaded | Multi-Threaded | Campfire Use Case |
|----------|------------------|----------------|-------------------|
| **Unique Ownership** | `Box<T>` | `Box<T>` | Trait objects, heap allocation |
| **Shared Ownership** | `Rc<T>` | `Arc<T>` | Connection manager, app state |
| **Interior Mutability** | `RefCell<T>` | `Mutex<T>` / `RwLock<T>` | Shared mutable state |
| **Combined** | `Rc<RefCell<T>>` | `Arc<RwLock<T>>` | WebSocket connections |

```rust
// ✅ CORRECT: Thread-safe shared state
#[derive(Clone)]
pub struct ConnectionManager {
    connections: Arc<RwLock<HashMap<UserId, Vec<ConnectionId>>>>,
    presence: Arc<RwLock<HashMap<RoomId, HashSet<UserId>>>>,
}
```

### 2. Collection Patterns

```rust
// ✅ CORRECT: API design - accept slices, store owned, return owned
pub fn process_content(content: &str) -> ProcessedContent {
    // Accept &str for flexibility (String, &str, literals)
    ProcessedContent::new(content)
}

pub struct Message {
    pub content: String,  // Store owned String, not &str
    // ... other fields
}

pub fn create_message(content: String) -> Message {
    Message { content } // Return owned types
}
```

## L3: Ecosystem Patterns

### 1. Error Handling Strategy

```rust
use thiserror::Error;
use anyhow::{Context, Result};

// ✅ CORRECT: Structured errors for libraries
#[derive(Error, Debug)]
pub enum CampfireError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Authentication failed: {reason}")]
    Authentication { reason: String },
    
    #[error("User {user_id} not authorized for room {room_id}")]
    Authorization { user_id: UserId, room_id: RoomId },
    
    #[error("Validation error: {field} - {message}")]
    Validation { field: String, message: String },
}

// ✅ CORRECT: Use anyhow for application context
pub async fn send_webhook(url: &str, payload: &WebhookPayload) -> Result<()> {
    let response = reqwest::Client::new()
        .post(url)
        .json(payload)
        .send()
        .await
        .with_context(|| format!("Failed to send webhook to {}", url))?;
    
    if !response.status().is_success() {
        return Err(anyhow::anyhow!("Webhook failed: {}", response.status()));
    }
    
    Ok(())
}
```

### 2. Async Patterns

```rust
// ✅ CORRECT: Actor pattern for state management
pub struct RoomActor {
    room_id: RoomId,
    message_rx: mpsc::Receiver<RoomMessage>,
    connections: HashMap<UserId, Vec<ConnectionHandle>>,
}

#[derive(Debug)]
pub enum RoomMessage {
    UserJoined { user_id: UserId, connection: ConnectionHandle },
    NewMessage { message: Message, sender: oneshot::Sender<Result<(), MessageError>> },
    GetState { sender: oneshot::Sender<RoomState> },
}

// ✅ CORRECT: Structured concurrency with JoinSet
use tokio::task::JoinSet;

pub async fn process_batch_webhooks(webhooks: Vec<WebhookData>) -> Vec<Result<(), WebhookError>> {
    let mut tasks = JoinSet::new();
    
    for webhook in webhooks {
        tasks.spawn(async move { deliver_webhook(webhook).await });
    }
    
    let mut results = Vec::new();
    while let Some(result) = tasks.join_next().await {
        results.push(result.unwrap_or_else(|e| Err(WebhookError::TaskPanic(e.to_string()))));
    }
    
    results
}
```

### 3. Web Application Patterns with Axum

```rust
// ✅ CORRECT: Type-safe request handling
pub async fn create_message(
    State(app_state): State<AppState>,
    Path(room_id): Path<RoomId>,
    session: AuthenticatedSession, // Custom extractor
    Json(payload): Json<CreateMessageRequest>,
) -> Result<Json<MessageResponse>, ApiError> {
    // Validate request
    payload.validate()?;
    
    // Check authorization
    app_state.auth_service
        .check_room_access(session.user_id, room_id)
        .await?;
    
    // Create message
    let message = app_state.message_service
        .create_message_with_deduplication(
            payload.content,
            room_id,
            session.user_id,
            payload.client_message_id,
        )
        .await?;
    
    Ok(Json(MessageResponse::from(message)))
}
```

## Testing Patterns

### 1. Property-Based Testing

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_message_ordering_invariant(
        messages in prop::collection::vec(arbitrary_message(), 1..100)
    ) {
        let mut sorted = messages.clone();
        sorted.sort_by(|a, b| a.created_at.cmp(&b.created_at).then_with(|| a.id.cmp(&b.id)));
        
        // Verify ordering invariant
        for window in sorted.windows(2) {
            assert!(window[0].created_at <= window[1].created_at);
            if window[0].created_at == window[1].created_at {
                assert!(window[0].id < window[1].id);
            }
        }
    }
}
```

### 2. Integration Testing

```rust
#[tokio::test]
async fn test_message_creation_end_to_end() {
    let app = test_app().await;
    let room_id = create_test_room(&app).await;
    let user = create_test_user(&app).await;
    
    let response = app
        .post(&format!("/api/rooms/{}/messages", room_id))
        .json(&json!({
            "content": "Test message",
            "client_message_id": Uuid::new_v4()
        }))
        .send()
        .await;
    
    assert_eq!(response.status(), 201);
    let message: MessageResponse = response.json().await;
    assert_eq!(message.content, "Test message");
}
```

## Performance Patterns

### 1. Memory Management

```rust
// ✅ CORRECT: Use string slices where possible
pub fn extract_mentions(content: &str) -> Vec<&str> {
    content
        .split_whitespace()
        .filter_map(|word| word.strip_prefix('@'))
        .collect()
}

// ✅ CORRECT: Use Cow for conditional ownership
use std::borrow::Cow;

pub fn normalize_content(content: &str) -> Cow<str> {
    if content.contains('\r') {
        Cow::Owned(content.replace('\r', ""))
    } else {
        Cow::Borrowed(content)
    }
}
```

### 2. Database Patterns

```rust
// ✅ CORRECT: Use sqlx with compile-time validation
pub async fn get_user_messages(
    db: &Database,
    user_id: UserId,
    room_id: RoomId,
) -> Result<Vec<Message>, sqlx::Error> {
    sqlx::query_as!(
        Message,
        r#"
        SELECT m.* FROM messages m
        JOIN memberships mem ON mem.room_id = m.room_id
        WHERE m.room_id = ? AND mem.user_id = ? AND mem.involvement != ?
        ORDER BY m.created_at DESC
        LIMIT 50
        "#,
        room_id.0,
        user_id.0,
        Involvement::Invisible as i32
    )
    .fetch_all(&db.pool)
    .await
}
```

## Anti-Patterns to Avoid

### 1. Common Mistakes

```rust
// ❌ WRONG: Blocking in async context
pub async fn process_message_blocking(message: Message) -> Result<(), ProcessingError> {
    std::thread::sleep(Duration::from_secs(1)); // Blocks entire runtime!
    Ok(())
}

// ✅ CORRECT: Use async sleep
pub async fn process_message_async(message: Message) -> Result<(), ProcessingError> {
    tokio::time::sleep(Duration::from_secs(1)).await;
    Ok(())
}

// ❌ WRONG: Unnecessary cloning
fn process_messages(messages: &[Message]) -> Vec<String> {
    messages.iter().map(|m| m.clone().content).collect() // Unnecessary clone
}

// ✅ CORRECT: Borrow what you need
fn process_messages(messages: &[Message]) -> Vec<String> {
    messages.iter().map(|m| m.content.clone()).collect()
}
```

### 2. Error Handling Anti-Patterns

```rust
// ❌ WRONG: Swallowing errors
pub async fn send_message_ignore_errors(message: Message) {
    let _ = send_message(message).await; // Ignores all errors!
}

// ✅ CORRECT: Handle or propagate errors
pub async fn send_message_with_handling(message: Message) -> Result<(), MessageError> {
    send_message(message).await.map_err(|e| {
        tracing::error!("Failed to send message: {}", e);
        e
    })
}
```

## Campfire-Specific Patterns

### 1. Domain Model Patterns

```rust
// ✅ CORRECT: Rich domain models with behavior
impl Message {
    pub fn extract_mentions(&self) -> Vec<UserId> {
        self.content
            .split_whitespace()
            .filter_map(|word| word.strip_prefix('@'))
            .filter_map(|username| self.room.find_user_by_name(username))
            .collect()
    }
    
    pub fn is_sound_command(&self) -> bool {
        self.content.starts_with("/play ")
    }
    
    pub fn get_sound_name(&self) -> Option<&str> {
        self.content.strip_prefix("/play ")
    }
}
```

### 2. Service Layer Patterns

```rust
// ✅ CORRECT: Service objects with clear responsibilities
pub struct MessageService {
    db: Arc<Database>,
    broadcaster: Arc<WebSocketBroadcaster>,
}

impl MessageService {
    pub async fn create_message_with_deduplication(
        &self,
        content: String,
        room_id: RoomId,
        creator_id: UserId,
        client_message_id: Uuid,
    ) -> Result<Message, MessageError> {
        // Implementation with proper error handling and side effects
    }
}
```

## Key Takeaways

1. **Type Safety First**: Use newtypes for all domain concepts
2. **Ownership Clarity**: Prefer borrowing, use owned types in structs
3. **Error Handling**: thiserror for libraries, anyhow for applications
4. **Async Patterns**: Actor model for state, structured concurrency for tasks
5. **Testing**: Property-based tests for invariants, integration tests for workflows
6. **Performance**: Zero-cost abstractions, avoid unnecessary allocations
7. **Database**: sqlx with compile-time validation, proper transaction handling

Follow these patterns to achieve compile-first success and Rails-equivalent reliability with Rust performance benefits.