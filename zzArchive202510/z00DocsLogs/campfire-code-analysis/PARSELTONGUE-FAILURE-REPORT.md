# Parseltongue Ruby Indexing Failure Report

**Date:** 2025-11-03
**Reporter:** @amuldotexe
**Issue:** Parseltongue v0.8.8 creates 0 entities when indexing Ruby/Rails codebase despite claiming Ruby support

---

## Executive Summary

Parseltongue v0.8.8 successfully processes 311 Ruby files but extracts **0 entities** from a production Ruby on Rails application (Basecamp's Campfire). The README claims "Multi-language query-based extraction for 12 languages including Ruby", but entity extraction from Ruby code appears non-functional.

---

## Environment Details

### System Information
```
OS: macOS (Darwin Kernel Version 24.3.0)
Architecture: arm64 (Apple Silicon)
Machine: Mac mini
```

### Parseltongue Version
```bash
$ ./parseltongue --version
parseltongue 0.8.8
```

### Target Codebase
- **Project:** Basecamp Campfire (original Ruby/Rails implementation)
- **Source:** https://github.com/basecamp/campfire (once-campfire fork)
- **Ruby Version:** 3.4.5
- **Rails Version:** main branch (Rails 8+)
- **Total Ruby Files:** 117 files in `app/` directory
- **Codebase Size:** 667 total files

---

## Reproduction Steps

### 1. Setup
```bash
cd /path/to/campfire-codebase
# Ensure parseltongue v0.8.8 binary is in current directory
./parseltongue --version  # Should show: parseltongue 0.8.8
```

### 2. Index Command
```bash
./parseltongue pt01-folder-to-cozodb-streamer . \
  --db "rocksdb:campfire-analysis.db" \
  --verbose
```

### 3. Observe Output
```
Running Tool 1: folder-to-cozodb-streamer
Starting directory streaming...

Streaming Summary:
Total files found: 667
Files processed: 311
Entities created: 0          ← ❌ PROBLEM: Should be > 0
Errors encountered: 356
Duration: 224.268916ms
✓ Indexing completed
  Files processed: 311
  Entities created: 0
  Duration: 224.268916ms
```

---

## Expected vs Actual Results

### Expected Behavior
Based on README claims:
- **v0.8.8 Feature**: "Query-based entity extraction for Rust, Python, C, C++, **Ruby**, JavaScript, TypeScript, Go, Java, PHP, C#, Swift"
- Should extract classes, methods, modules from Ruby files
- Should create entities for ActiveRecord models, controllers, helpers
- Entities created count should be > 0 (expected ~100-500 entities for this codebase)

### Actual Behavior
- ✅ Files are discovered (667 found)
- ✅ Files are processed (311 processed)
- ❌ **0 entities extracted** (critical failure)
- ⚠️ 356 errors encountered (likely parsing failures)

---

## Codebase Characteristics

### Directory Structure
```
app/
├── assets/
├── channels/     (9 Ruby files - Action Cable channels)
├── controllers/  (21 Ruby files - Rails controllers)
├── helpers/      (24 Ruby files - View helpers)
├── javascript/   (Non-Ruby, Stimulus controllers)
├── jobs/         (8 Ruby files - Background jobs)
├── models/       (27 Ruby files - ActiveRecord models)
└── views/        (ERB templates, not .rb files)
```

### Sample Ruby Files

#### Example 1: Rails Model (`app/models/room.rb`)
```ruby
class Room < ApplicationRecord
  has_many :memberships, dependent: :delete_all do
    def grant_to(users)
      room = proxy_association.owner
      Membership.insert_all(Array(users).collect { |user| { room_id: room.id, user_id: user.id, involvement: room.default_involvement } })
    end

    def revoke_from(users)
      destroy_by user: users
    end

    def revise(granted: [], revoked: [])
      transaction do
        grant_to(granted) if granted.present?
        revoke_from(revoked) if revoked.present?
      end
    end
  end

  has_many :users, through: :memberships
  has_many :messages, dependent: :destroy

  belongs_to :creator, class_name: "User", default: -> { Current.user }

  scope :opens,           -> { where(type: "Rooms::Open") }
  scope :closeds,         -> { where(type: "Rooms::Closed") }
  scope :directs,         -> { where(type: "Rooms::Direct") }
  scope :without_directs, -> { where.not(type: "Rooms::Direct") }

  scope :ordered, -> { order("LOWER(name)") }
  # ... more code
end
```

**Expected Entities:**
- Class: `Room`
- Methods: `grant_to`, `revoke_from`, `revise`
- Scopes: `opens`, `closeds`, `directs`, `without_directs`, `ordered`
- Associations: `has_many`, `belongs_to`

**Actual Entities:** 0 (none extracted)

#### Example 2: Rails Controller (`app/controllers/rooms_controller.rb`)
```ruby
class RoomsController < ApplicationController
  before_action :set_room, only: %i[ show destroy ]
  before_action :ensure_can_administer, only: %i[ destroy ]
  before_action :remember_last_room_visited, only: :show

  def index
    redirect_to room_url(Current.user.rooms.last)
  end

  def show
    @messages = find_messages
  end

  def destroy
    @room.destroy

    broadcast_remove_room
    redirect_to root_url
  end

  private
    def set_room
      if room = Current.user.rooms.find_by(id: params[:room_id] || params[:id])
        @room = room
      else
        redirect_to root_url, alert: "Room not found or inaccessible"
      end
    end

    def ensure_can_administer
      # ... implementation
    end
end
```

**Expected Entities:**
- Class: `RoomsController`
- Public methods: `index`, `show`, `destroy`
- Private methods: `set_room`, `ensure_can_administer`
- Callbacks: `before_action` declarations

**Actual Entities:** 0 (none extracted)

---

## Ruby Patterns in Codebase

The codebase uses standard Rails/Ruby patterns that should be parseable:

1. **Class Definitions**
   ```ruby
   class Room < ApplicationRecord
   class RoomsController < ApplicationController
   ```

2. **Method Definitions**
   ```ruby
   def index
   def show
   def private_method
   ```

3. **Blocks and Lambdas**
   ```ruby
   has_many :memberships do
     def grant_to(users)
   end

   scope :opens, -> { where(type: "Rooms::Open") }
   ```

4. **ActiveRecord DSL**
   ```ruby
   has_many :users, through: :memberships
   belongs_to :creator, class_name: "User"
   scope :ordered, -> { order("LOWER(name)") }
   ```

5. **Rails Callbacks**
   ```ruby
   before_action :set_room, only: %i[ show destroy ]
   ```

---

## Diagnostic Questions

To help debug this issue, please clarify:

1. **Ruby Support Status**: Is Ruby entity extraction fully implemented in v0.8.8, or is it partial/experimental?

2. **Tree-sitter Grammar**: Which tree-sitter Ruby grammar version is parseltongue using? Is it compatible with Ruby 3.4.5 syntax?

3. **Entity Types**: What Ruby entities should be extracted?
   - Classes?
   - Methods (public/private)?
   - Modules?
   - DSL methods (has_many, scope, etc.)?
   - Blocks and lambdas?

4. **Error Details**: The "356 errors encountered" suggests parsing failures. Can these be:
   - Logged to stderr for debugging?
   - Exported with `--verbose` flag?
   - Captured in a separate error log?

5. **Test Coverage**: Are there integration tests for Ruby entity extraction? If so, what syntax patterns are tested?

6. **Known Limitations**: Are there known Ruby patterns that fail to parse? Examples:
   - Metaprogramming (define_method, class_eval)?
   - Rails DSL magic (has_many with blocks)?
   - Refinements or prepend/include?

---

## Impact

This issue blocks the intended use case:
- **Goal:** Analyze legacy Ruby/Rails codebase architecture using ISG (Interface Signature Graph)
- **Blocked:** Cannot generate dependency graph or entity map
- **Workaround:** Must use traditional code exploration (grep, file reading) instead of ISG analysis

---

## Suggested Investigations

1. **Enable Debug Logging**: Add flag to show which files fail parsing and why
2. **Minimal Reproduction**: Test with simple Ruby file:
   ```ruby
   # test.rb
   class SimpleClass
     def simple_method
       puts "hello"
     end
   end
   ```
3. **Compare with Other Languages**: Test same command structure on Python/JavaScript to verify working behavior
4. **Tree-sitter Validation**: Test tree-sitter Ruby grammar independently to isolate parser vs extraction issue

---

## Additional Context

### Gem Dependencies (Gemfile)
```ruby
gem "rails", github: "rails/rails", branch: "main"  # Rails 8+
gem "sqlite3", "~> 2.7"
gem "redis", "~> 5.4"
gem "puma", "~> 6.6"
gem "resque", "~> 2.7.0"
gem "turbo-rails"
gem "stimulus-rails"
```

### File Count by Type
```
Ruby files (.rb): 117
ERB templates (.erb): ~50+
JavaScript (.js): ~30+
CSS/SCSS: ~20+
Total files: 667
```

### Successfully Processed
311 files were processed (likely all .rb files + some config files), but zero entities extracted from any of them.

---

## Request for Assistance

Please investigate why Ruby entity extraction produces 0 entities despite:
- ✅ Correct parseltongue version (v0.8.8)
- ✅ Valid Ruby syntax (Ruby 3.4.5 standard)
- ✅ Standard Rails patterns
- ✅ Files successfully processed (311 files)

Any guidance on:
1. Debug flags to see parsing errors
2. Ruby syntax limitations
3. Expected entity types
4. Timeline for Ruby support improvements

Would be greatly appreciated!

---

## Contact

- **GitHub:** @amuldotexe
- **Repository:** https://github.com/that-in-rust/campfire-on-rust
- **Original Codebase:** https://github.com/basecamp/campfire

Happy to provide additional information, test patches, or run diagnostic commands as needed.
