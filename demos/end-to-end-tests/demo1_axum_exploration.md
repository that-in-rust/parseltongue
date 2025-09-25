# Demo 1: Axum Codebase Exploration Journey

## 🎯 Scenario
A developer wants to understand the Axum web framework codebase to:
1. Understand the architecture
2. Find key components for building web apps
3. Trace how requests flow through the system
4. Identify extension points for middleware

## 🚀 Step-by-Step Exploration

### Step 1: Initial Codebase Ingestion
```bash
# Ingest the Axum codebase dump
./parseltongue ingest axum_codebase.dump
```

### Step 2: Get Overview with pt wrapper
```bash
# Map the codebase structure
./pt map
```

### Step 3: Find Key Components
```bash
# Find the main Router - core of any web framework
./pt find Router

# Find Handler trait - how requests are processed
./pt find Handler

# Find middleware components
./pt find Middleware
```

### Step 4: Understand Request Flow
```bash
# Trace how requests are handled
./pt trace handle_request

# Understand routing mechanism
./pt trace route

# See how extractors work
./pt trace FromRequest
```

### Step 5: Impact Analysis for Extensions
```bash
# If I want to add custom middleware
./pt impact Middleware

# If I want to extend routing
./pt impact Router

# If I want to add custom extractors
./pt impact FromRequest
```

### Step 6: Safety Checks for Modifications
```bash
# Check safety of modifying core routing
./pt safe Router

# Check safety of handler modifications
./pt safe Handler
```

### Step 7: Generate LLM Context for Deep Understanding
```bash
# Get comprehensive context for Router
./pt context Router

# Get context for understanding middleware patterns
./pt context Middleware

# Get context for request/response cycle
./pt context FromRequest
```

## 📊 Expected Insights

### Architecture Understanding
- **Router**: Central routing mechanism
- **Handler**: Request processing abstraction
- **Extractor**: Data extraction from requests
- **Middleware**: Request/response transformation
- **Service**: Tower service integration

### Request Flow
1. Request → Router → Handler
2. Extractors pull data from request
3. Handler processes and returns response
4. Middleware can wrap any stage

### Extension Points
- Custom extractors via `FromRequest`
- Custom middleware via `tower::Layer`
- Custom handlers via `Handler` trait
- Custom routing via `Router` methods

## 🎯 Developer Value
After this exploration, a developer can:
- ✅ Understand Axum's architecture in 15 minutes
- ✅ Know where to add custom functionality
- ✅ Understand request flow and data extraction
- ✅ Make informed decisions about framework usage