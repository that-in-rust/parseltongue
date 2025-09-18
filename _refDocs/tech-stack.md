---
inclusion: always
---

# Campfire-on-Rust: TDD-Driven Technology Stack

## TDD-FIRST DEVELOPMENT PHILOSOPHY

**Core Principle**: Define complete function signatures, type contracts, and property tests before writing any implementation code. This ensures one-shot correctness and prevents coordination complexity.

### TDD Development Cycle
1. **Type Contracts**: Define complete function signatures with all error cases
2. **Property Tests**: Specify behavior through property-based testing
3. **Integration Contracts**: Define service boundaries and interaction patterns
4. **Implementation**: Type-guided implementation following contracts
5. **Validation**: Comprehensive testing validates contract compliance

## REQUIRED STACK WITH TDD PATTERNS

### Backend: Rust with Compile-First Success
- **Web Framework**: Axum with type-safe extractors and comprehensive error handling
- **Database**: SQLite with sqlx compile-time query validation and Dedicated Writer Task pattern
- **Real-time**: ActionCable-inspired WebSocket with Actor pattern for connection management
- **Concurrency**: Structured concurrency with tokio JoinSet and message-passing actors
- **Error Handling**: thiserror for library errors, anyhow for application context
- **Type Safety**: Newtype pattern for all IDs, making invalid states unrepresentable
- **Authentication**: JWT with secure session management and rate limiting
- **Testing**: Property-based testing with proptest, integration tests with real SQLite

### Frontend: React with Modern Patterns (2025)
- **Component Architecture**: Functional components with custom hooks for logic separation
- **State Management**: TanStack Query for server state, Zustand for client state
- **Error Handling**: Error boundaries with graceful fallback components
- **Real-time**: WebSocket integration with automatic reconnection and optimistic updates
- **Testing**: React Testing Library with comprehensive component and hook testing
- **Performance**: Strategic memoization and virtual scrolling for large lists
- **Type Safety**: TypeScript with strict mode and comprehensive type definitions

## ADVANCED RUST PATTERNS (L1-L3)

### L1: Core Language Patterns
```rust
// Ownership and borrowing mastery
fn process_message_content(content: &str) -> ProcessedContent {
    // Read-only access, no ownership needed
    ProcessedContent::from_str(content)
}

// Type-driven design with newtypes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct UserId(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct RoomId(pub i64);

// Making invalid states unrepresentable
#[derive(Debug, Clone)]
pub enum RoomType {
    Open,
    Closed { invited_users: Vec<UserId> },
    Direct { participants: [UserId; 2] }, // Exactly 2 participants
}

// Zero-cost abstractions with iterators
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
```

### L2: Standard Library Patterns
```rust
// Smart pointer decision matrix for thread safety
#[derive(Clone)]
pub struct ConnectionManager {
    connections: Arc<RwLock<HashMap<UserId, Vec<ConnectionId>>>>,
    presence: Arc<RwLock<HashMap<RoomId, HashSet<UserId>>>>,
}

// Collection patterns: accept slices, store owned, return owned
pub fn process_content(content: &str) -> ProcessedContent {
    ProcessedContent::new(content)
}

pub struct Message {
    pub content: String,  // Store owned String, not &str
    // ... other fields
}
```

### L3: Ecosystem Patterns
```rust
// Comprehensive error handling with thiserror
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

// Actor pattern for state management
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

// Structured concurrency with JoinSet
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

## ADVANCED REACT PATTERNS (2025 EDITION)

### Type-Safe Component Architecture
```typescript
// Branded types for domain safety
type UserId = string & { readonly brand: unique symbol };
type RoomId = string & { readonly brand: unique symbol };
type MessageId = string & { readonly brand: unique symbol };

// Component contracts defined first
interface MessageComponentProps {
  message: Message;
  currentUser: User;
  onEdit: (messageId: MessageId, content: string) => Promise<void>;
  onBoost: (messageId: MessageId, emoji: string) => Promise<void>;
}

// Compound component pattern for complex UI
const MessageComponent = {
  Root: MessageRoot,
  Header: MessageHeader,
  Body: MessageBody,
  Actions: MessageActions,
  Boosts: MessageBoosts,
} as const;
```

### State Management with Zustand + Immer
```typescript
// Type-safe store with immer integration
interface ChatStore {
  // State
  rooms: Map<RoomId, Room>;
  messages: Map<RoomId, Message[]>;
  currentUser: User | null;
  connectionState: ConnectionState;
  
  // Actions (no reducers, direct mutations with immer)
  actions: {
    addMessage: (roomId: RoomId, message: Message) => void;
    updateMessage: (messageId: MessageId, updates: Partial<Message>) => void;
    setConnectionState: (state: ConnectionState) => void;
    optimisticAddMessage: (roomId: RoomId, tempMessage: OptimisticMessage) => void;
    confirmOptimisticMessage: (tempId: string, confirmedMessage: Message) => void;
  };
}

// Store implementation with immer
const useChatStore = create<ChatStore>()(immer((set, get) => ({
  rooms: new Map(),
  messages: new Map(),
  currentUser: null,
  connectionState: 'disconnected',
  
  actions: {
    addMessage: (roomId, message) => set((state) => {
      const roomMessages = state.messages.get(roomId) || [];
      roomMessages.push(message);
      state.messages.set(roomId, roomMessages);
    }),
    
    updateMessage: (messageId, updates) => set((state) => {
      for (const [roomId, messages] of state.messages) {
        const messageIndex = messages.findIndex(m => m.id === messageId);
        if (messageIndex !== -1) {
          Object.assign(messages[messageIndex], updates);
          break;
        }
      }
    }),
    
    // Optimistic updates with rollback capability
    optimisticAddMessage: (roomId, tempMessage) => set((state) => {
      const roomMessages = state.messages.get(roomId) || [];
      roomMessages.push({ ...tempMessage, isOptimistic: true });
      state.messages.set(roomId, roomMessages);
    }),
    
    confirmOptimisticMessage: (tempId, confirmedMessage) => set((state) => {
      for (const [roomId, messages] of state.messages) {
        const tempIndex = messages.findIndex(m => m.tempId === tempId);
        if (tempIndex !== -1) {
          messages[tempIndex] = confirmedMessage;
          break;
        }
      }
    }),
  },
})));
```

### WebSocket Integration with React Query
```typescript
// WebSocket hook with automatic reconnection
function useWebSocket(roomId: RoomId) {
  const queryClient = useQueryClient();
  const { actions } = useChatStore();
  
  const { data: connection, error } = useQuery({
    queryKey: ['websocket', roomId],
    queryFn: () => createWebSocketConnection(roomId),
    staleTime: Infinity,
    retry: (failureCount, error) => {
      // Exponential backoff with jitter
      const delay = Math.min(1000 * Math.pow(2, failureCount), 30000);
      const jitter = Math.random() * 1000;
      setTimeout(() => queryClient.invalidateQueries(['websocket', roomId]), delay + jitter);
      return failureCount < 5;
    },
  });
  
  useEffect(() => {
    if (!connection) return;
    
    const handleMessage = (event: MessageEvent) => {
      const wsMessage = JSON.parse(event.data) as WebSocketMessage;
      
      switch (wsMessage.type) {
        case 'MessageCreated':
          actions.addMessage(roomId, wsMessage.data.message);
          // Invalidate queries to refetch if needed
          queryClient.invalidateQueries(['messages', roomId]);
          break;
          
        case 'MessageUpdated':
          actions.updateMessage(wsMessage.data.message.id, wsMessage.data.message);
          break;
          
        case 'PresenceUpdate':
          queryClient.setQueryData(['presence', roomId], wsMessage.data.onlineUsers);
          break;
      }
    };
    
    connection.addEventListener('message', handleMessage);
    return () => connection.removeEventListener('message', handleMessage);
  }, [connection, roomId, actions, queryClient]);
  
  return { connection, error };
}
```

### Optimistic Updates Pattern
```typescript
// Optimistic message sending with rollback
function useOptimisticMessageSend(roomId: RoomId) {
  const { actions } = useChatStore();
  const queryClient = useQueryClient();
  
  return useMutation({
    mutationFn: async (content: string) => {
      const tempId = crypto.randomUUID();
      const clientMessageId = crypto.randomUUID();
      
      // Optimistic update
      const tempMessage: OptimisticMessage = {
        tempId,
        clientMessageId,
        content,
        roomId,
        createdAt: new Date().toISOString(),
        isOptimistic: true,
      };
      
      actions.optimisticAddMessage(roomId, tempMessage);
      
      try {
        // Send to server
        const confirmedMessage = await api.createMessage(roomId, {
          content,
          clientMessageId,
        });
        
        // Replace optimistic message with confirmed
        actions.confirmOptimisticMessage(tempId, confirmedMessage);
        
        return confirmedMessage;
      } catch (error) {
        // Rollback optimistic update
        actions.rollbackOptimisticMessage(tempId);
        throw error;
      }
    },
    
    onError: (error, variables, context) => {
      // Show error toast
      toast.error('Failed to send message. Please try again.');
    },
  });
}
```

## DATABASE RULES WITH TDD PATTERNS

- **SQLite only** with compile-time query validation using sqlx macros
- **Direct SQL with sqlx** - No ORMs, property-based testing for query correctness
- **WAL mode** for basic concurrency with Dedicated Writer Task pattern
- **FTS5** for search with comprehensive test coverage
- **Connection pooling** with sqlx Pool and proper error handling
- **Dedicated Writer Task** pattern for write serialization (Critical Gap #3)

## FORBIDDEN DEPENDENCIES (Anti-Coordination)

- **NO Redis** - Use SQLite for everything with proper testing
- **NO message queues** (RabbitMQ, Kafka, etc.) - Use tokio channels with actor pattern
- **NO event stores** or event sourcing libraries - Direct database operations
- **NO coordination frameworks** (Akka, Orleans, etc.) - Simple message passing
- **NO microservice frameworks** - Single binary with embedded assets
- **NO complex async runtimes** beyond tokio - Structured concurrency patterns

## DEPLOYMENT CONSTRAINTS

- **Single binary** with embedded assets using rust-embed
- **No Docker orchestration** - simple container deployment with health checks
- **Environment variables** for configuration with validation
- **No service discovery** or load balancing complexity
- **Database in mounted volume** - NEVER in container image
- **Comprehensive monitoring** with Prometheus metrics and structured logging

## TDD TESTING STRATEGY

### Property-Based Testing
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

### Integration Testing
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

When complex dependencies are suggested, respond with: "This violates our TDD-first simplicity constraints. Here's the type-safe, property-tested approach that stays within our Rails-equivalent patterns..."