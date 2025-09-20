# Advanced React Patterns for Campfire Frontend (2025 Edition)

## Core Philosophy: Type-Safe Component Architecture

**Fundamental Principle**: Build React components with complete TypeScript type safety, modern patterns, and comprehensive error handling. Every component should be predictable, testable, and maintainable.

### React Development Methodology

```
TYPE DEFINITIONS → COMPONENT CONTRACTS → HOOK LOGIC → IMPLEMENTATION → TESTING
       ↓                    ↓               ↓            ↓             ↓
   Complete           Component         Custom        Type-Safe     Comprehensive
   TypeScript         Interface         Hooks         Component     Testing
   Types              Contracts         Logic         Implementation Coverage
```

## Type-Safe Component Architecture

### Branded Types for Domain Safety

```typescript
// Prevent ID confusion at compile time
type UserId = string & { readonly brand: unique symbol };
type RoomId = string & { readonly brand: unique symbol };
type MessageId = string & { readonly brand: unique symbol };

// Type-safe component props
interface MessageComponentProps {
  message: Message;
  currentUser: User;
  onEdit: (messageId: MessageId, content: string) => Promise<void>;
  onBoost: (messageId: MessageId, emoji: string) => Promise<void>;
  onReply: (messageId: MessageId) => void;
}

// Compound component pattern for complex UI
const MessageComponent = {
  Root: MessageRoot,
  Header: MessageHeader,
  Body: MessageBody,
  Actions: MessageActions,
  Boosts: MessageBoosts,
  Replies: MessageReplies,
} as const;

// Usage with full type safety
<MessageComponent.Root message={message} currentUser={currentUser}>
  <MessageComponent.Header />
  <MessageComponent.Body />
  <MessageComponent.Actions onEdit={handleEdit} onBoost={handleBoost} />
  <MessageComponent.Boosts />
</MessageComponent.Root>
```

### Component Composition Patterns

#### Render Props Pattern for Complex State Sharing
```typescript
interface MessageListRenderProps {
  messages: Message[];
  isLoading: boolean;
  error: Error | null;
  hasMore: boolean;
  loadMore: () => void;
}

function MessageList({ 
  roomId, 
  children 
}: { 
  roomId: RoomId;
  children: (props: MessageListRenderProps) => React.ReactNode;
}) {
  const {
    data: messages = [],
    isLoading,
    error,
    hasNextPage,
    fetchNextPage,
  } = useInfiniteQuery({
    queryKey: ['messages', roomId],
    queryFn: ({ pageParam = null }) => api.getMessages(roomId, { before: pageParam }),
    getNextPageParam: (lastPage) => lastPage.hasMore ? lastPage.messages[0]?.id : undefined,
  });
  
  return (
    <>
      {children({
        messages: messages.pages.flatMap(page => page.messages),
        isLoading,
        error,
        hasMore: hasNextPage,
        loadMore: fetchNextPage,
      })}
    </>
  );
}

// Usage with render props
<MessageList roomId={roomId}>
  {({ messages, isLoading, hasMore, loadMore }) => (
    <VirtualizedList
      items={messages}
      renderItem={({ item }) => <MessageComponent message={item} />}
      onEndReached={loadMore}
      hasMore={hasMore}
      isLoading={isLoading}
    />
  )}
</MessageList>
```

#### Provider Pattern for Global State
```typescript
// Theme provider for Campfire
function ThemeProvider({ children }: { children: React.ReactNode }) {
  const [theme, setTheme] = useState<'light' | 'dark'>('light');
  
  const toggleTheme = useCallback(() => {
    setTheme(prev => prev === 'light' ? 'dark' : 'light');
  }, []);
  
  return (
    <ThemeContext.Provider value={{ theme, toggleTheme }}>
      <div className={`app theme-${theme}`}>
        {children}
      </div>
    </ThemeContext.Provider>
  );
}

// Usage in any component
function Header() {
  const { theme, toggleTheme } = useContext(ThemeContext);
  
  return (
    <header>
      <h1>Campfire</h1>
      <button onClick={toggleTheme}>
        Switch to {theme === 'light' ? 'dark' : 'light'} mode
      </button>
    </header>
  );
}
```

## State Management with Zustand + Immer

### Type-Safe Store with Immer Integration

```typescript
// Type-safe store with immer for immutable updates
interface ChatStore {
  // State
  rooms: Map<RoomId, Room>;
  messages: Map<RoomId, Message[]>;
  optimisticMessages: Map<string, OptimisticMessage>;
  currentUser: User | null;
  connectionState: ConnectionState;
  typingUsers: Map<RoomId, Set<UserId>>;
  
  // Actions (no reducers - direct mutations with immer)
  actions: {
    // Message actions
    addMessage: (roomId: RoomId, message: Message) => void;
    updateMessage: (messageId: MessageId, updates: Partial<Message>) => void;
    removeMessage: (messageId: MessageId) => void;
    
    // Optimistic updates
    addOptimisticMessage: (roomId: RoomId, tempMessage: OptimisticMessage) => void;
    confirmOptimisticMessage: (tempId: string, confirmedMessage: Message) => void;
    rollbackOptimisticMessage: (tempId: string) => void;
    
    // Connection state
    setConnectionState: (state: ConnectionState) => void;
    
    // Typing indicators
    setUserTyping: (roomId: RoomId, userId: UserId, isTyping: boolean) => void;
    clearTypingUsers: (roomId: RoomId) => void;
  };
}

// Store implementation with immer
const useChatStore = create<ChatStore>()(immer((set, get) => ({
  rooms: new Map(),
  messages: new Map(),
  optimisticMessages: new Map(),
  currentUser: null,
  connectionState: 'disconnected',
  typingUsers: new Map(),
  
  actions: {
    addMessage: (roomId, message) => set((state) => {
      const roomMessages = state.messages.get(roomId) || [];
      // Insert in chronological order
      const insertIndex = roomMessages.findIndex(m => 
        new Date(m.createdAt) > new Date(message.createdAt)
      );
      if (insertIndex === -1) {
        roomMessages.push(message);
      } else {
        roomMessages.splice(insertIndex, 0, message);
      }
      state.messages.set(roomId, roomMessages);
    }),
    
    addOptimisticMessage: (roomId, tempMessage) => set((state) => {
      state.optimisticMessages.set(tempMessage.tempId, tempMessage);
      
      // Also add to room messages for immediate UI update
      const roomMessages = state.messages.get(roomId) || [];
      roomMessages.push({
        ...tempMessage,
        id: tempMessage.tempId as MessageId,
        isOptimistic: true,
      });
      state.messages.set(roomId, roomMessages);
    }),
    
    confirmOptimisticMessage: (tempId, confirmedMessage) => set((state) => {
      // Remove from optimistic messages
      state.optimisticMessages.delete(tempId);
      
      // Replace in room messages
      for (const [roomId, messages] of state.messages) {
        const tempIndex = messages.findIndex(m => 
          m.isOptimistic && m.id === tempId
        );
        if (tempIndex !== -1) {
          messages[tempIndex] = confirmedMessage;
          break;
        }
      }
    }),
    
    setUserTyping: (roomId, userId, isTyping) => set((state) => {
      const roomTypers = state.typingUsers.get(roomId) || new Set();
      if (isTyping) {
        roomTypers.add(userId);
      } else {
        roomTypers.delete(userId);
      }
      state.typingUsers.set(roomId, roomTypers);
    }),
  },
})));
```

### Store Selectors for Performance

```typescript
// Efficient selectors to prevent unnecessary re-renders
const useRoomMessages = (roomId: RoomId) => {
  return useChatStore(
    useCallback(
      (state) => state.messages.get(roomId) || [],
      [roomId]
    )
  );
};

const useTypingUsers = (roomId: RoomId) => {
  return useChatStore(
    useCallback(
      (state) => Array.from(state.typingUsers.get(roomId) || []),
      [roomId]
    )
  );
};

const useConnectionState = () => {
  return useChatStore((state) => state.connectionState);
};
```

## WebSocket Integration with React Query

### WebSocket Hook with Automatic Reconnection

```typescript
// WebSocket hook with automatic reconnection and state sync
function useWebSocket(roomId: RoomId) {
  const queryClient = useQueryClient();
  const { actions } = useChatStore();
  const [lastSeenMessageId, setLastSeenMessageId] = useState<MessageId | null>(null);
  
  const { data: connection, error, refetch } = useQuery({
    queryKey: ['websocket', roomId],
    queryFn: () => createWebSocketConnection(roomId, lastSeenMessageId),
    staleTime: Infinity,
    retry: (failureCount, error) => {
      // Exponential backoff with jitter (Rails ActionCable pattern)
      const baseDelay = Math.min(1000 * Math.pow(2, failureCount), 30000);
      const jitter = Math.random() * 1000;
      const delay = baseDelay + jitter;
      
      setTimeout(() => {
        queryClient.invalidateQueries(['websocket', roomId]);
      }, delay);
      
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
          setLastSeenMessageId(wsMessage.data.message.id);
          // Optimistically update React Query cache
          queryClient.setQueryData(
            ['messages', roomId],
            (old: InfiniteData<MessagePage> | undefined) => {
              if (!old) return old;
              const newPages = [...old.pages];
              newPages[0] = {
                ...newPages[0],
                messages: [wsMessage.data.message, ...newPages[0].messages],
              };
              return { ...old, pages: newPages };
            }
          );
          break;
          
        case 'MessageUpdated':
          actions.updateMessage(wsMessage.data.message.id, wsMessage.data.message);
          queryClient.invalidateQueries(['messages', roomId]);
          break;
          
        case 'TypingNotification':
          actions.setUserTyping(
            roomId,
            wsMessage.data.user.id,
            wsMessage.data.isTyping
          );
          break;
          
        case 'PresenceUpdate':
          queryClient.setQueryData(['presence', roomId], wsMessage.data.onlineUsers);
          break;
          
        case 'Connected':
          actions.setConnectionState('connected');
          break;
          
        case 'Disconnected':
          actions.setConnectionState('disconnected');
          // Trigger reconnection
          setTimeout(() => refetch(), 1000);
          break;
      }
    };
    
    const handleError = (event: Event) => {
      actions.setConnectionState('error');
      // Trigger reconnection with backoff
      setTimeout(() => refetch(), 2000);
    };
    
    connection.addEventListener('message', handleMessage);
    connection.addEventListener('error', handleError);
    connection.addEventListener('close', handleError);
    
    return () => {
      connection.removeEventListener('message', handleMessage);
      connection.removeEventListener('error', handleError);
      connection.removeEventListener('close', handleError);
    };
  }, [connection, roomId, actions, queryClient, refetch]);
  
  return { connection, error, isConnected: !!connection };
}
```

## Optimistic Updates with Rollback

### Optimistic Message Sending

```typescript
// Optimistic message sending with comprehensive error handling
function useOptimisticMessageSend(roomId: RoomId) {
  const { actions } = useChatStore();
  const queryClient = useQueryClient();
  
  return useMutation({
    mutationFn: async (content: string) => {
      const tempId = crypto.randomUUID();
      const clientMessageId = crypto.randomUUID();
      
      // Create optimistic message
      const tempMessage: OptimisticMessage = {
        tempId,
        clientMessageId,
        content,
        roomId,
        createdAt: new Date().toISOString(),
        isOptimistic: true,
      };
      
      // Optimistic update
      actions.addOptimisticMessage(roomId, tempMessage);
      
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
      // Show user-friendly error message
      toast.error('Failed to send message. Please try again.');
    },
    
    onSuccess: (data, variables, context) => {
      // Optional success feedback
      // toast.success('Message sent successfully');
    },
  });
}
```

### Optimistic Room Updates

```typescript
function useOptimisticRoomUpdate() {
  const { actions } = useChatStore();
  const queryClient = useQueryClient();
  
  return useMutation({
    mutationFn: async ({ roomId, updates }: { roomId: RoomId; updates: Partial<Room> }) => {
      // Optimistic update
      queryClient.setQueryData(['room', roomId], (old: Room | undefined) => {
        if (!old) return old;
        return { ...old, ...updates };
      });
      
      try {
        const updatedRoom = await api.updateRoom(roomId, updates);
        return updatedRoom;
      } catch (error) {
        // Rollback on error
        queryClient.invalidateQueries(['room', roomId]);
        throw error;
      }
    },
    
    onError: (error) => {
      toast.error('Failed to update room. Please try again.');
    },
  });
}
```

## Error Handling and Resilience

### Error Boundary with Retry Logic

```typescript
// Comprehensive error boundary
class ChatErrorBoundary extends React.Component<
  { children: React.ReactNode; fallback?: React.ComponentType<{ error: Error; retry: () => void }> },
  { hasError: boolean; error: Error | null }
> {
  constructor(props: any) {
    super(props);
    this.state = { hasError: false, error: null };
  }
  
  static getDerivedStateFromError(error: Error) {
    return { hasError: true, error };
  }
  
  componentDidCatch(error: Error, errorInfo: React.ErrorInfo) {
    // Log to monitoring service
    console.error('Chat error:', error, errorInfo);
    
    // Send to error reporting service
    if (window.Sentry) {
      window.Sentry.captureException(error, { extra: errorInfo });
    }
  }
  
  retry = () => {
    this.setState({ hasError: false, error: null });
  };
  
  render() {
    if (this.state.hasError) {
      const Fallback = this.props.fallback || DefaultErrorFallback;
      return <Fallback error={this.state.error!} retry={this.retry} />;
    }
    
    return this.props.children;
  }
}

// Default error fallback component
function DefaultErrorFallback({ error, retry }: { error: Error; retry: () => void }) {
  return (
    <div className="error-boundary">
      <h2>Something went wrong</h2>
      <details>
        <summary>Error details</summary>
        <pre>{error.message}</pre>
      </details>
      <button onClick={retry}>Try again</button>
    </div>
  );
}
```

### Suspense with Error Boundaries

```typescript
// Suspense wrapper with error handling
function SuspenseWithErrorBoundary({ 
  children, 
  fallback = <LoadingSpinner />,
  errorFallback 
}: {
  children: React.ReactNode;
  fallback?: React.ReactNode;
  errorFallback?: React.ComponentType<{ error: Error; retry: () => void }>;
}) {
  return (
    <ChatErrorBoundary fallback={errorFallback}>
      <Suspense fallback={fallback}>
        {children}
      </Suspense>
    </ChatErrorBoundary>
  );
}

// Usage
<SuspenseWithErrorBoundary>
  <MessageList roomId={roomId} />
</SuspenseWithErrorBoundary>
```

## Performance Optimization Patterns

### Strategic Memoization

```typescript
// Memoize expensive calculations
function MessageList({ messages, searchTerm }: { messages: Message[]; searchTerm: string }) {
  // Only memoize expensive filtering operation
  const filteredMessages = useMemo(() => {
    if (!searchTerm) return messages;
    
    return messages.filter(message => 
      message.content.toLowerCase().includes(searchTerm.toLowerCase()) ||
      message.author.name.toLowerCase().includes(searchTerm.toLowerCase())
    );
  }, [messages, searchTerm]);
  
  return (
    <div>
      {filteredMessages.map(message => (
        <MessageItem key={message.id} message={message} />
      ))}
    </div>
  );
}

// Stable callback references for memoized children
const MemoizedMessageItem = React.memo(MessageItem);

function MessageList({ messages }: { messages: Message[] }) {
  // Stable callback reference for memoized child
  const handleEdit = useCallback((messageId: MessageId, newContent: string) => {
    updateMessage(messageId, newContent);
  }, []); // No dependencies if updateMessage is stable
  
  return (
    <div>
      {messages.map(message => (
        <MemoizedMessageItem 
          key={message.id} 
          message={message}
          onEdit={handleEdit} // Stable reference prevents re-renders
        />
      ))}
    </div>
  );
}
```

### Virtual Scrolling for Large Lists

```typescript
import { FixedSizeList as List } from 'react-window';

function VirtualizedMessageList({ messages }: { messages: Message[] }) {
  const Row = ({ index, style }: { index: number; style: React.CSSProperties }) => (
    <div style={style}>
      <MessageItem message={messages[index]} />
    </div>
  );
  
  return (
    <List
      height={600}
      itemCount={messages.length}
      itemSize={80}
      width="100%"
    >
      {Row}
    </List>
  );
}
```

## Testing Patterns

### Component Testing with React Testing Library

```typescript
import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { MessageComposer } from './MessageComposer';

test('sends message when form is submitted', async () => {
  const mockSendMessage = jest.fn().mockResolvedValue({ id: '123' });
  const user = userEvent.setup();
  
  render(<MessageComposer onSend={mockSendMessage} />);
  
  const textarea = screen.getByRole('textbox');
  const submitButton = screen.getByRole('button', { name: /send/i });
  
  await user.type(textarea, 'Hello, world!');
  await user.click(submitButton);
  
  expect(mockSendMessage).toHaveBeenCalledWith('Hello, world!');
  expect(textarea).toHaveValue(''); // Should clear after send
});
```

### Custom Hook Testing

```typescript
import { renderHook, act } from '@testing-library/react';
import { useMessages } from './useMessages';

test('useMessages manages message state correctly', async () => {
  const { result } = renderHook(() => useMessages('room-123'));
  
  expect(result.current.messages).toEqual([]);
  expect(result.current.loading).toBe(true);
  
  // Wait for initial load
  await waitFor(() => {
    expect(result.current.loading).toBe(false);
  });
  
  // Test sending a message
  await act(async () => {
    await result.current.sendMessage('Test message');
  });
  
  expect(result.current.messages).toHaveLength(1);
  expect(result.current.messages[0].content).toBe('Test message');
});
```

## Key Benefits of Advanced React Patterns

1. **Type Safety**: Complete TypeScript coverage prevents runtime errors
2. **Performance**: Strategic memoization and virtual scrolling for large datasets
3. **Error Resilience**: Comprehensive error boundaries with graceful fallbacks
4. **State Management**: Predictable state updates with Zustand and Immer
5. **Real-time Integration**: Seamless WebSocket integration with React Query
6. **Optimistic Updates**: Immediate UI feedback with rollback capabilities
7. **Testing**: Comprehensive testing patterns for components and hooks
8. **Maintainability**: Clear separation of concerns and reusable patterns

These patterns ensure that the Campfire frontend is robust, performant, and maintainable while providing an excellent user experience.