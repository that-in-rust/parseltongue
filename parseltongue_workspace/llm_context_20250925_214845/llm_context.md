# LLM Context: Codebase Analysis
Generated: Thu Sep 25 21:48:45 IST 2025
Codebase: .

## Executive Summary
- Total Entities:      212
- Functions:        0
- Structs:        0
- Traits:        0

## Key Functions (Top 20)

## Key Structs (Top 15)

## Key Traits (Top 10)

## Architecture Patterns
Based on entity analysis, this codebase appears to use:
- Service pattern (multiple *Service entities found)
- Repository pattern (data access abstraction)
- Manager pattern (resource management)

## LLM Instructions
When working with this codebase:
1. Focus on the top 20 functions for understanding core functionality
2. Key structs represent the main data models
3. Traits define the main abstractions and interfaces
5. Use entity names exactly as listed (case-sensitive)
6. Consider impact analysis before suggesting changes

## Quick Reference Commands
```bash
# Find entity definition
./target/release/parseltongue_20250924231324 where-defined EntityName

# Analyze impact of changes
./target/release/parseltongue_20250924231324 blast-radius EntityName

# List entities by type
./target/release/parseltongue_20250924231324 list-entities --type functions
./target/release/parseltongue_20250924231324 list-entities --type structs
./target/release/parseltongue_20250924231324 list-entities --type traits
```
