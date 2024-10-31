#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m'

echo "🚀 Setting up Code Analyzer development environment..."

# Check for required package managers
check_requirements() {
    command -v npm >/dev/null 2>&1 || { echo -e "${RED}npm is required but not installed.${NC}" >&2; exit 1; }
    command -v java >/dev/null 2>&1 || { echo -e "${RED}Java 21 JDK is required but not installed.${NC}" >&2; exit 1; }
    command -v cargo >/dev/null 2>&1 || { echo -e "${RED}Rust is required but not installed.${NC}" >&2; exit 1; }
}

# Frontend setup
setup_frontend() {
    echo "📦 Setting up Next.js frontend..."
    cd frontend
    # Create Next.js app with TypeScript and Tailwind
    npx create-next-app@latest . --typescript --tailwind --eslint --app --src-dir --use-npm
    # Install additional dependencies
    npm install @tanstack/react-query axios react-hook-form zod @headlessui/react
    cd ..
}

# Java backend setup
setup_java_backend() {
    echo "☕ Setting up Spring Boot backend..."
    cd backend-java
    # Use Spring Initializr to create project
    curl https://start.spring.io/starter.tgz \
        -d type=gradle-project \
        -d language=java \
        -d bootVersion=3.2.0 \
        -d baseDir=. \
        -d groupId=com.parseltongue \
        -d artifactId=analyzer \
        -d name=analyzer \
        -d description="Code Analyzer" \
        -d packageName=com.parseltongue.analyzer \
        -d packaging=jar \
        -d javaVersion=21 \
        -d dependencies=web,data-mongodb,lombok,actuator \
        | tar -xzvf -
    cd ..
}

# Rust backend setup
setup_rust_backend() {
    echo "🦀 Setting up Rust backend..."
    cd backend-rust
    # Initialize new Rust project
    cargo init --bin
    cd ..
}

# MongoDB setup
setup_mongodb() {
    echo "🍃 Setting up MongoDB..."
    # Create data directory
    mkdir -p data/db
    # Start MongoDB (assumes MongoDB is installed)
    mongod --dbpath ./data/db --fork --logpath ./data/mongod.log
}

# Create project structure
create_structure() {
    echo "📁 Creating project structure..."
    mkdir -p {frontend,backend-java,backend-rust,shared}/{src,config}
    mkdir -p frontend/src/{components,services,types}
    mkdir -p backend-java/src/main/java/com/parseltongue/{config,controller,model,repository,service}
    mkdir -p backend-rust/src/{config,handlers,models,services}
    mkdir -p shared/{types,config}
}

# Main setup
main() {
    check_requirements
    create_structure
    setup_frontend
    setup_java_backend
    setup_rust_backend
    setup_mongodb
    
    echo -e "${GREEN}✅ Setup complete! Run './scripts/dev-commands.sh start' to start development servers${NC}"
}

# Run setup
main 