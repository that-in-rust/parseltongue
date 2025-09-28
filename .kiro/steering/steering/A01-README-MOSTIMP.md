# Codebase Wisdom 101
Constantly do cargo clean etc so that unnecessary files do not messs up your context or space

# Technical Design101: TDD-First Architecture Principles

Test-First Development: I should be writing tests FIRST, following the STUB → RED → GREEN → REFACTOR cycle


# Product thinking for us
Think like Shreyas Doshi  - the famous product leader - his minimalism - user journeys mindset

## The Essence: Executable Specifications Drive Everything

Exectuable Specifications is the concept  - stick to it 


**Core Truth**: Traditional user stories fail LLMs because they're designed for human conversation. LLMs need executable blueprints, not ambiguous narratives.

**The Solution**: Transform all specifications into formal, testable contracts with preconditions, postconditions, and error conditions. Every claim must be validated by automated tests.

**Why This Matters**: Eliminates the #1 cause of LLM hallucination - ambiguous requirements that lead to incorrect implementations.

## The Non-Negotiables: 8 Architectural Principles

These principles are derived from the Parseltongue AIM Daemon design process and prevent the most common architectural failures in Rust systems:

### 1. Executable Specifications Over Narratives
**Contract-driven development with measurable outcomes**

### 2. Layered Rust Architecture (L1→L2→L3)
**Clear separation: Core → Std → External dependencies**

### 3. Dependency Injection for Testability
**Every component depends on traits, not concrete types**

### 4. RAII Resource Management
**All resources automatically managed with Drop implementations**

### 5. Performance Claims Must Be Test-Validated
**Every performance assertion backed by automated tests**

### 6. Structured Error Handling
**thiserror for libraries, anyhow for applications**

### 7. Complex Domain Model Support
**Handle real-world complexity, not simplified examples**

### 8. Concurrency Model Validation
**Thread safety validated with stress tests**

### 9. MVP-First Rigor (New Pattern)
**Proven architectures over theoretical abstractions**

## IMPORTANT FOR VISUALS AND DIAGRAMS

ALL DIAGRAMS WILL BE IN MERMAID ONLY TO ENSURE EASE WITH GITHUB - DO NOT SKIP THAT - use MermaidSteering.md file for that - it must be somewhere - use it
