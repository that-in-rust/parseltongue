#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}🚀 Starting Parseltongue Installation...${NC}"

# Error handling
set -e
trap 'echo -e "${RED}Error: Command failed at line $LINENO${NC}" >&2' ERR

# Function to check command existence and version
check_command() {
    local cmd=$1
    local version_arg=$2
    if ! command -v $cmd &> /dev/null; then
        echo -e "${RED}$cmd is not installed${NC}"
        return 1
    fi
    echo "$($cmd $version_arg)"
    return 0
}

# 1. Infrastructure Requirements
echo -e "${BLUE}📦 Installing Infrastructure Requirements...${NC}"

# MongoDB 6.0+
if ! check_command mongod --version | grep -q "v6.0"; then
    echo "Installing MongoDB 6.0..."
    # Import MongoDB public key
    curl -fsSL https://pgp.mongodb.com/server-6.0.asc | \
        sudo gpg -o /usr/share/keyrings/mongodb-server-6.0.gpg \
        --dearmor
    
    # Create list file for MongoDB
    echo "deb [ arch=amd64,arm64 signed-by=/usr/share/keyrings/mongodb-server-6.0.gpg ] https://repo.mongodb.org/apt/ubuntu jammy/mongodb-org/6.0 multiverse" | \
        sudo tee /etc/apt/sources.list.d/mongodb-org-6.0.list
    
    sudo apt-get update
    sudo apt-get install -y mongodb-org=6.0.12 mongodb-org-database=6.0.12 mongodb-org-server=6.0.12 mongodb-org-mongos=6.0.12 mongodb-org-tools=6.0.12
    
    # Pin package versions
    echo "mongodb-org hold" | sudo dpkg --set-selections
    echo "mongodb-org-database hold" | sudo dpkg --set-selections
    echo "mongodb-org-server hold" | sudo dpkg --set-selections
    echo "mongodb-org-mongos hold" | sudo dpkg --set-selections
    echo "mongodb-org-tools hold" | sudo dpkg --set-selections
    
    sudo systemctl daemon-reload
    sudo systemctl start mongod
    sudo systemctl enable mongod
fi

# Java 21 JDK
if ! check_command java --version | grep -q "21"; then
    echo "Installing Java 21..."
    curl -s "https://get.sdkman.io" | bash
    source "$HOME/.sdkman/bin/sdkman-init.sh"
    sdk install java 21-tem
fi

# Node.js 20+
if ! check_command node --version | grep -q "v20"; then
    echo "Installing Node.js 20..."
    curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
    sudo apt-get install -y nodejs
fi

# Rust 1.70+
if ! check_command rustc --version | grep -q "1.70"; then
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

# Create project structure as per step02_module.txt
echo -e "${BLUE}📁 Creating Project Structure...${NC}"

# Create base directories
mkdir -p frontend/src/{app,components,services,types}
mkdir -p backend-java/src/main/java/com/parseltongue/{config,controller,model,repository,service}
mkdir -p backend-rust/src/{config,handlers,models,services}
mkdir -p shared/{types,config}

# Initialize Next.js frontend
cd frontend
echo "Setting up Next.js 14+ frontend..."
npx create-next-app@latest . \
    --typescript \
    --tailwind \
    --eslint \
    --app \
    --src-dir \
    --use-npm \
    --import-alias "@/*"

# Add required dependencies
npm install @tanstack/react-query axios react-hook-form zod @headlessui/react
cd ..

# Initialize Java backend
cd backend-java
echo "Setting up Spring Boot backend..."
curl https://start.spring.io/starter.tgz \
    -d type=gradle-project \
    -d language=java \
    -d bootVersion=3.2.0 \
    -d baseDir=. \
    -d groupId=com.parseltongue \
    -d artifactId=analyzer \
    -d name=analyzer \
    -d packageName=com.parseltongue.analyzer \
    -d packaging=jar \
    -d javaVersion=21 \
    -d dependencies=web,data-mongodb,lombok,actuator,validation \
    | tar -xzvf -
cd ..

# Initialize Rust backend
cd backend-rust
echo "Setting up Rust backend..."
cargo init --bin
# Add dependencies to Cargo.toml
cat > Cargo.toml << EOL
[package]
name = "parseltongue-analyzer"
version = "0.1.0"
edition = "2021"

[dependencies]
actix-web = "4.4"
tokio = { version = "1.35", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
mongodb = "2.8"
futures = "0.3"
tracing = "0.1"
tracing-subscriber = "0.3"
EOL
cd ..

# Create scripts directory
mkdir -p scripts
cat > scripts/dev-commands.sh << 'EOL'
#!/bin/bash
# Start all services
start_all() {
    mongod --dbpath ./data/db &
    cd frontend && npm run dev &
    cd ../backend-java && ./gradlew bootRun &
    cd ../backend-rust && cargo run &
}
# Usage
case "$1" in
    "start") start_all ;;
    *) echo "Usage: $0 start" ;;
esac
EOL
chmod +x scripts/dev-commands.sh

echo -e "${GREEN}✅ Installation Complete!${NC}"
echo -e "\nNext steps:"
echo "1. Run health checks:     ./scripts/dev-commands.sh start"
echo "2. Access services:"
echo "   - Frontend:    http://localhost:3000"
echo "   - Java API:    http://localhost:8080/health"
echo "   - Rust API:    http://localhost:8081/health"
echo "3. Start development by implementing components marked as ❌ in step02_module.txt"