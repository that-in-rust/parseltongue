### 9.3 Context Injection Methods

#### **Direct File Injection Techniques**

**File Reading Patterns:**
```bash
# Chunked reading for large files
Read /path/to/file offset=0 limit=1000      # First chunk
Read /path/to/file offset=1000 limit=1000   # Second chunk
Read /path/to/file offset=2000 limit=1000   # Third chunk
```

**Optimization Strategies:**
- **Survey with Glob/Grep first**: Find relevant patterns before reading
- **Targeted reading**: Focus on specific sections rather than entire files
- **Format-specific handling**: Different approaches for txt, json, md files
- **Token budget management**: Track token usage during injection

#### **Large File Handling (150k Token Context)**

**Character-to-Token Ratio:**
- **Approximate ratio**: 4 characters = 1 token
- **150k tokens ≈ 600k characters** of text
- **Line overhead**: Line numbers and formatting add to token count
- **Margin planning**: Leave buffer for system prompts and interactions

**File Type Considerations:**
- **Text files**: Direct character-to-token conversion
- **JSON files**: Structure adds parsing overhead
- **Markdown files**: Formatting characters add to token count
- **Code files**: Syntax highlighting and structure affect tokenization

#### **Data Structure Integration**

**Structured Data Injection:**
```python
# Example: Large JSON data structure
large_data = {
    "interfaces": [...],  # ISG interface data
    "relationships": [...],  # Dependency relationships
    "metadata": {...}  # Enrichment metadata
}

# Strategy: Serialize and inject in chunks
for chunk in chunk_data(large_data, chunk_size=1000):
    inject_into_context(chunk)
```

**Batch Processing Patterns:**
- **Chunked serialization**: Break large data into manageable pieces
- **Progressive loading**: Load data as needed during conversation
- **Context preservation**: Maintain context across multiple injections
- **Reference management**: Keep track of injected content for later reference
