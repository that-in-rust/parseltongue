# TDD-Driven Development Patterns for Campfire

## Core Philosophy: Interface Contracts Before Implementation

**Fundamental Principle**: Define complete function signatures, type contracts, and property tests before writing any implementation code. This ensures one-shot correctness and prevents coordination complexity from emerging during development.

### TDD Development Methodology

```
TYPE CONTRACTS → PROPERTY TESTS → INTEGRATION CONTRACTS → IMPLEMENTATION → VALIDATION
       ↓               ↓                    ↓                  ↓             ↓
   Complete        Behavior           Service            Type-Guided    Comprehensive
   Interface       Properties         Boundaries         Implementation    Testing
   Design          Specification      Definition         Following         Validation
                                                        Contracts
```

## Phase 1: Type Contract Definition

### Complete Function Signature Specification

Every function must be fully specified before implementation:

```rust
/// Creates a message with automatic deduplication based on client_message_id
/// 
/// # Type Contract
/// - Input: CreateMessageData with validated fields
/// - Output: Result<DeduplicatedMessage<Verified>, MessageError>
/// - Side Effects: Database write, FTS5 index update, room timestamp update
/// 
/// # Properties
/// - Same client_message_id always returns the same Message
/// - Message is atomically created and indexed for search
/// - Broadcast occurs after successful database commit
/// - Room last_message_at is updated atomically
/// 
/// # Error Cases
/// - ValidationError: Invalid message content or parameters
/// - DatabaseError: SQLite operation failure
/// - AuthorizationError: User lacks room access
/// - DuplicationError: Handled by returning existing message
pub async fn create_message_with_deduplication(
    &self,
    data: CreateMessageData,
) -> Result<DeduplicatedMessage<Verified>, MessageError>;
```

### Type Safety Patterns

#### Phantom Types for State Safety
```rust
// Prevent invalid state transitions at compile time
#[derive(Debug)]
pub struct Message<State> {
    inner: MessageData,
    _state: PhantomData<State>,
}

pub struct Draft;
pub struct Validated;
pub struct Persisted;

// Only validated messages can be persisted
impl Message<Draft> {
    pub fn validate(self) -> Result<Message<Validated>, ValidationError> {
        if self.inner.body.trim().is_empty() {
            return Err(ValidationError::EmptyBody);
        }
        Ok(Message {
            inner: self.inner,
            _state: PhantomData,
        })
    }
}

impl Message<Validated> {
    pub async fn persist(self, db: &DatabaseWriter) -> Result<Message<Persisted>, DatabaseError> {
        let persisted_data = db.create_message(self.inner).await?;
        Ok(Message {
            inner: persisted_data,
            _state: PhantomData,
        })
    }
}

// Only persisted messages can be broadcast
impl WebSocketBroadcaster {
    pub async fn broadcast_message(&self, message: Message<Persisted>) {
        // Type system ensures only persisted messages are broadcast
    }
}
```

#### Session Types for Protocol Safety
```rust
// WebSocket connection state machine in types
pub struct WebSocketConnection<State> {
    id: ConnectionId,
    sender: mpsc::UnboundedSender<WebSocketMessage>,
    _state: PhantomData<State>,
}

pub struct Connected;
pub struct Authenticated { user_id: UserId }
pub struct Subscribed { room_id: RoomId }

// State transitions enforced by type system
impl WebSocketConnection<Connected> {
    pub fn authenticate(self, user_id: UserId) -> WebSocketConnection<Authenticated> {
        WebSocketConnection {
            id: self.id,
            sender: self.sender,
            _state: PhantomData,
        }
    }
}

impl WebSocketConnection<Authenticated> {
    pub fn subscribe_to_room(self, room_id: RoomId) -> WebSocketConnection<Subscribed> {
        WebSocketConnection {
            id: self.id,
            sender: self.sender,
            _state: PhantomData,
        }
    }
}

// Only subscribed connections can receive room messages
impl WebSocketConnection<Subscribed> {
    pub async fn send_message(&self, message: &WebSocketMessage) -> Result<(), SendError> {
        self.sender.send(message.clone())
            .map_err(|_| SendError::ConnectionClosed)
    }
}
```

#### Newtype Pattern for Domain Safety
```rust
// Prevent ID confusion at compile time
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct UserId(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct RoomId(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct MessageId(pub i64);

// Compile-time prevention of ID confusion
fn get_user_messages(user_id: UserId, room_id: RoomId) -> Vec<MessageId> {
    // Cannot accidentally pass RoomId where UserId expected
    todo!()
}
```

## Phase 2: Property-Based Test Specification

### Property Test Patterns

```rust
#[cfg(test)]
mod message_service_properties {
    use proptest::prelude::*;
    
    proptest! {
        #[test]
        fn duplicate_client_message_ids_return_same_message(
            room_id in any::<RoomId>(),
            client_id in any::<Uuid>(),
            content1 in ".*",
            content2 in ".*",
            user_id in any::<UserId>()
        ) {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let service = create_test_message_service().await;
                
                let data1 = CreateMessageData {
                    room_id,
                    creator_id: user_id,
                    body: content1,
                    client_message_id: client_id,
                };
                
                let data2 = CreateMessageData {
                    room_id,
                    creator_id: user_id,
                    body: content2, // Different content
                    client_message_id: client_id, // Same client ID
                };
                
                let msg1 = service.create_message_with_deduplication(data1).await.unwrap();
                let msg2 = service.create_message_with_deduplication(data2).await.unwrap();
                
                // Property: Same client_message_id always returns same Message
                prop_assert_eq!(msg1.inner.id, msg2.inner.id);
                prop_assert_eq!(msg1.inner.body, msg2.inner.body); // Original content preserved
            });
        }
        
        #[test]
        fn messages_since_returns_chronological_order(
            room_id in any::<RoomId>(),
            message_count in 1..20usize
        ) {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let service = create_test_message_service().await;
                
                // Create messages in sequence
                let mut message_ids = Vec::new();
                for i in 0..message_count {
                    let data = CreateMessageData {
                        room_id,
                        creator_id: UserId(1),
                        body: format!("Message {}", i),
                        client_message_id: Uuid::new_v4(),
                    };
                    let msg = service.create_message_with_deduplication(data).await.unwrap();
                    message_ids.push(msg.inner.id);
                }
                
                // Get messages since first message
                let since_id = message_ids[0];
                let messages = service.get_messages_since(room_id, since_id).await.unwrap();
                
                // Property: Messages returned in chronological order
                for window in messages.windows(2) {
                    prop_assert!(window[0].created_at <= window[1].created_at);
                }
            });
        }
        
        #[test]
        fn websocket_reconnection_delivers_missed_messages(
            room_id in any::<RoomId>(),
            user_id in any::<UserId>(),
            message_count in 1..10usize,
            disconnect_after in 0..5usize
        ) {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let service = create_test_websocket_service().await;
                
                // Create initial messages
                let mut all_messages = Vec::new();
                for i in 0..message_count {
                    let msg = create_test_message(room_id, format!("Message {}", i)).await;
                    all_messages.push(msg);
                }
                
                // Simulate disconnect after some messages
                let last_seen = if disconnect_after < all_messages.len() {
                    Some(all_messages[disconnect_after].id)
                } else {
                    None
                };
                
                // Reconnect and get missed messages
                let missed_messages = service.handle_reconnection(
                    user_id, 
                    room_id, 
                    last_seen
                ).await.unwrap();
                
                // Property: All messages after last_seen are delivered
                if let Some(last_seen_id) = last_seen {
                    let expected_missed: Vec<_> = all_messages
                        .into_iter()
                        .skip_while(|m| m.id <= last_seen_id)
                        .collect();
                    
                    prop_assert_eq!(missed_messages.len(), expected_missed.len());
                    for (received, expected) in missed_messages.iter().zip(expected_missed.iter()) {
                        prop_assert_eq!(received.id, expected.id);
                    }
                }
            });
        }
    }
}
```

### Invariant Testing Patterns

```rust
// Test invariants that must hold across all operations
proptest! {
    #[test]
    fn room_membership_invariants(
        room_type in any::<RoomType>(),
        user_ids in prop::collection::vec(any::<UserId>(), 1..10),
        operations in prop::collection::vec(membership_operation(), 1..20)
    ) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let service = create_test_room_service().await;
            
            // Create room with initial members
            let room = service.create_room(room_type, user_ids.clone()).await.unwrap();
            
            // Apply sequence of membership operations
            for op in operations {
                let _ = service.apply_membership_operation(room.id, op).await;
            }
            
            // Verify invariants
            let memberships = service.get_room_memberships(room.id).await.unwrap();
            
            // Invariant 1: Room creator always has membership (unless deactivated)
            let creator_membership = memberships.iter()
                .find(|m| m.user_id == room.creator_id);
            prop_assert!(creator_membership.is_some());
            
            // Invariant 2: Direct rooms have exactly 2 members
            if matches!(room.room_type, RoomType::Direct) {
                prop_assert_eq!(memberships.len(), 2);
            }
            
            // Invariant 3: No duplicate memberships
            let mut seen_users = HashSet::new();
            for membership in &memberships {
                prop_assert!(seen_users.insert(membership.user_id));
            }
        });
    }
}
```

## Phase 3: Integration Contract Definition

### Service Boundary Contracts

```rust
// Define complete service interaction contracts
pub struct ServiceContracts {
    pub message_service: Arc<dyn MessageService<Error = MessageError>>,
    pub room_service: Arc<dyn RoomService<Error = RoomError>>,
    pub broadcaster: Arc<dyn WebSocketBroadcaster<Error = BroadcastError>>,
    pub membership_service: Arc<dyn MembershipService<Error = MembershipError>>,
}

// Integration test contracts
#[tokio::test]
async fn message_creation_integration_contract() {
    let contracts = create_test_service_contracts().await;
    
    // Given: A room with connected users
    let room_id = RoomId(1);
    let user_id = UserId(1);
    let connection_id = ConnectionId::new();
    
    // Setup room membership
    contracts.membership_service
        .create_membership(user_id, room_id, Involvement::Everything)
        .await
        .unwrap();
    
    // Setup WebSocket connection
    let mut receiver = contracts.broadcaster
        .subscribe_to_room(connection_id, room_id)
        .await
        .unwrap();
    
    // When: A message is created
    let message_data = CreateMessageData {
        room_id,
        creator_id: user_id,
        body: "Test message".to_string(),
        client_message_id: Uuid::new_v4(),
    };
    
    let created_message = contracts.message_service
        .create_message_with_deduplication(message_data)
        .await
        .unwrap();
    
    // Then: Message is broadcast to room subscribers
    let broadcast_message = tokio::time::timeout(
        Duration::from_millis(100),
        receiver.recv()
    ).await.unwrap().unwrap();
    
    match broadcast_message {
        WebSocketMessage::MessageCreated { message } => {
            assert_eq!(message.id, created_message.inner.id);
            assert_eq!(message.body, created_message.inner.body);
        }
        _ => panic!("Expected MessageCreated broadcast"),
    }
}
```

### Cross-Service Integration Tests

```rust
#[tokio::test]
async fn end_to_end_message_flow_contract() {
    let app = create_test_app().await;
    
    // Create user and room
    let user = create_test_user(&app, "test@example.com").await;
    let room = create_test_room(&app, user.id, RoomType::Open).await;
    
    // Establish WebSocket connection
    let ws_client = connect_websocket(&app, user.id, room.id).await;
    
    // Send message via HTTP API
    let message_response = app
        .post(&format!("/api/rooms/{}/messages", room.id))
        .json(&json!({
            "body": "Hello, world!",
            "client_message_id": Uuid::new_v4()
        }))
        .send()
        .await;
    
    assert_eq!(message_response.status(), 201);
    
    // Verify WebSocket broadcast received
    let ws_message = ws_client.receive_message().await;
    match ws_message {
        WebSocketMessage::MessageCreated { message } => {
            assert_eq!(message.body, "Hello, world!");
            assert_eq!(message.creator_id, user.id);
            assert_eq!(message.room_id, room.id);
        }
        _ => panic!("Expected MessageCreated WebSocket message"),
    }
    
    // Verify message persisted in database
    let stored_message = app.database
        .get_message(message.id)
        .await
        .unwrap()
        .unwrap();
    
    assert_eq!(stored_message.body, "Hello, world!");
    
    // Verify FTS5 search index updated
    let search_results = app.database
        .search_messages("Hello", user.id, 10)
        .await
        .unwrap();
    
    assert_eq!(search_results.len(), 1);
    assert_eq!(search_results[0].id, message.id);
}
```

## Phase 4: Type-Guided Implementation

### Actor Pattern Implementation

```rust
// Implementation follows from type contracts
pub struct DatabaseWriter {
    sender: mpsc::UnboundedSender<WriteCommand>,
}

impl DatabaseWriter {
    pub fn new(pool: SqlitePool) -> Self {
        let (sender, mut receiver) = mpsc::unbounded_channel();
        
        // Single writer task - no coordination needed
        tokio::spawn(async move {
            while let Some(command) = receiver.recv().await {
                match command {
                    WriteCommand::CreateMessage { data, reply } => {
                        let result = Self::execute_create_message(&pool, data).await;
                        let _ = reply.send(result);
                    }
                    WriteCommand::UpdateMessage { id, data, reply } => {
                        let result = Self::execute_update_message(&pool, id, data).await;
                        let _ = reply.send(result);
                    }
                }
            }
        });
        
        Self { sender }
    }
    
    pub async fn create_message(&self, data: CreateMessageData) -> Result<Message, MessageError> {
        let (reply_tx, reply_rx) = oneshot::channel();
        
        self.sender.send(WriteCommand::CreateMessage { data, reply: reply_tx })
            .map_err(|_| MessageError::WriterUnavailable)?;
            
        reply_rx.await
            .map_err(|_| MessageError::WriterUnavailable)?
    }
}
```

### RAII Resource Management

```rust
// Automatic resource cleanup with RAII
pub struct PresenceGuard {
    user_id: UserId,
    room_id: RoomId,
    tracker: Arc<PresenceTracker>,
}

impl PresenceGuard {
    pub fn new(user_id: UserId, room_id: RoomId, tracker: Arc<PresenceTracker>) -> Self {
        tracker.increment_presence(user_id, room_id);
        Self { user_id, room_id, tracker }
    }
}

impl Drop for PresenceGuard {
    fn drop(&mut self) {
        // Automatic cleanup - no coordination needed
        self.tracker.decrement_presence(self.user_id, self.room_id);
    }
}

// Usage ensures presence is always cleaned up
pub async fn handle_websocket_connection(
    user_id: UserId,
    room_id: RoomId,
    presence_tracker: Arc<PresenceTracker>,
) {
    let _presence = PresenceGuard::new(user_id, room_id, presence_tracker);
    // Connection handling logic
    // Presence automatically decremented when _presence is dropped
}
```

## Phase 5: Comprehensive Validation

### Test Coverage Requirements

1. **Unit Tests**: Every function has corresponding unit tests
2. **Property Tests**: All invariants verified with property-based testing
3. **Integration Tests**: All service boundaries tested with real dependencies
4. **End-to-End Tests**: Complete user workflows tested through HTTP API
5. **Performance Tests**: Critical paths benchmarked for regression detection

### Continuous Validation

```rust
// Benchmark critical paths
#[bench]
fn bench_message_creation(b: &mut Bencher) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let service = rt.block_on(create_test_message_service());
    
    b.iter(|| {
        rt.block_on(async {
            let data = CreateMessageData {
                room_id: RoomId(1),
                creator_id: UserId(1),
                body: "Benchmark message".to_string(),
                client_message_id: Uuid::new_v4(),
            };
            
            service.create_message_with_deduplication(data).await.unwrap()
        })
    });
}
```

## Key Benefits of TDD-First Approach

1. **One-Shot Correctness**: Complete interface design prevents most bugs before implementation
2. **Coordination Prevention**: Type system enforces simple patterns, prevents complex coordination
3. **Comprehensive Testing**: Property tests catch edge cases that unit tests miss
4. **Documentation**: Function signatures serve as executable documentation
5. **Refactoring Safety**: Type contracts ensure refactoring doesn't break interfaces
6. **Performance Predictability**: Benchmarks catch performance regressions early

## Anti-Patterns to Avoid

1. **Implementation Before Contracts**: Never write implementation without complete type contracts
2. **Weak Error Types**: All error cases must be enumerated in Result types
3. **Untested Properties**: All invariants must have corresponding property tests
4. **Coordination Complexity**: Type system should prevent coordination patterns
5. **Incomplete Integration Tests**: All service boundaries must be integration tested

This TDD-first approach ensures that Campfire achieves Rails-equivalent reliability with Rust performance benefits while preventing coordination complexity through type-driven design.