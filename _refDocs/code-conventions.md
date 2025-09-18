---
inclusion: always
---

# Campfire-on-Rust: Code Conventions

## FILE ORGANIZATION

- **Maximum 500 lines per file** - Split larger files into smaller modules
- **Rails-style modules**: models/, handlers/, services/, middleware/
- **Clear module boundaries** - No circular dependencies
- **Single responsibility** - Each file has one clear purpose

## ERROR HANDLING

- **Result<T, E> only** - No custom error types unless absolutely necessary
- **anyhow** allowed in tests and main.rs only
- **Flat error handling** - Avoid nested Result chains
- **User-friendly messages** - Convert technical errors to user messages
- **No complex error recovery** - Log and return simple errors

## ASYNC PATTERNS

- **tokio::spawn** for simple background tasks only
- **No complex async coordination** - Keep async operations simple
- **Direct function calls** preferred over async when possible
- **Channel communication** only for Dedicated Writer Task pattern

## TYPE SAFETY

- **Newtypes for IDs**: UserId(i64), RoomId(i64), MessageId(i64)
- **Strong typing** for domain concepts
- **Avoid stringly-typed** APIs
- **Use enums** for finite state (UserRole, RoomType, etc.)

## DATABASE PATTERNS

- **sqlx::query!** macros for compile-time SQL validation
- **Direct SQL** - No query builders beyond sqlx
- **Prepared statements** for performance
- **Transactions** for consistency, but keep them simple

## TESTING PATTERNS

- **Unit tests** for individual functions
- **Integration tests** for API endpoints
- **Property tests** for invariants (message ordering, etc.)
- **No mocking** - Use real SQLite databases in tests

## PERFORMANCE GUIDELINES

- **Profile before optimizing** - Don't guess at bottlenecks
- **Rust's natural performance** is usually sufficient
- **Avoid premature optimization** - Simple code first
- **Memory efficiency** through ownership, not complex caching

When complex patterns are suggested, respond with: "This violates our simplicity constraints. Here's the straightforward approach that stays within our conventions..."