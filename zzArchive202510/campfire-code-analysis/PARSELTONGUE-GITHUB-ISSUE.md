# Bug Report: Ruby Entity Extraction Produces 0 Entities in v0.8.8

## Description

Parseltongue v0.8.8 processes Ruby files successfully but extracts **0 entities** from a production Ruby on Rails codebase, despite README claiming "Multi-language query-based extraction for 12 languages including Ruby".

## Environment

- **Parseltongue Version:** 0.8.8
- **OS:** macOS (Darwin 24.3.0, ARM64)
- **Target Codebase:** Ruby 3.4.5 + Rails 8 (Basecamp Campfire)
- **Codebase Stats:** 117 Ruby files, 667 total files

## Reproduction

```bash
# Index Ruby/Rails codebase
./parseltongue pt01-folder-to-cozodb-streamer . \
  --db "rocksdb:campfire.db" \
  --verbose
```

## Output

```
Streaming Summary:
Total files found: 667
Files processed: 311
Entities created: 0         ← ISSUE: Expected > 0
Errors encountered: 356
Duration: 224ms
```

## Expected Behavior

From a Rails model like:

```ruby
class Room < ApplicationRecord
  has_many :memberships

  def grant_to(users)
    # ...
  end

  scope :ordered, -> { order("LOWER(name)") }
end
```

**Expected entities:**
- Class: `Room`
- Method: `grant_to`
- Associations: `has_many :memberships`
- Scope: `ordered`

**Actual entities:** 0

## Questions

1. Is Ruby entity extraction fully implemented in v0.8.8?
2. Which tree-sitter Ruby grammar version is used?
3. Can the 356 errors be logged for debugging?
4. What Ruby entities are supported (classes, methods, DSL calls)?
5. Are there known limitations with Rails DSL or metaprogramming?

## Impact

Blocks ISG-based codebase analysis for Ruby/Rails projects. Users must fall back to traditional code exploration.

## Additional Info

- Standard Rails patterns (models, controllers, ActiveRecord DSL)
- Valid Ruby 3.4.5 syntax
- All 311 .rb files processed, 0 entities extracted
- Full detailed report available with code samples

## Test Case

Minimal reproduction test:

```ruby
# test.rb
class SimpleClass
  def simple_method
    puts "hello"
  end
end
```

Expected: 2 entities (1 class, 1 method)
Actual with v0.8.8: ?

---

Happy to provide more details, test patches, or diagnostic output as needed!
