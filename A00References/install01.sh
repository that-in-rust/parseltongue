#!/bin/bash

# === Agenda Items ===
# 🎯 Project: Parseltongue v0.1.3
# 
# 📋 Installation Tasks:
# ✨ Frontend: Next.js 14+ with TypeScript and Tailwind
# 🔧 Backend (Java): Spring Boot 3.x with MongoDB
# 🦀 Backend (Rust): Actix-web with MongoDB
# 🗄️ Database: MongoDB 6.0+
# 
# 🔍 Prerequisites:
# - Ubuntu 22.04
# - Node.js 20+
# - Java 21+
# - Rust 1.70+
# - MongoDB 6.0+
# 
# 📦 Components to Install:
# - Next.js and dependencies
# - Spring Boot CLI
# - Gradle
# - Cargo/Rust tools
# - MongoDB tools
#
# 🚀 Post-Install:
# - Project structure setup
# - Development scripts
# - Database initialization
# ===========================

# Exit on error, but allow installations to continue even with unrelated errors
set +e

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to install Java 21
install_java() {
    echo "Installing Java 21 JDK..."
    sudo apt-get install -y software-properties-common
    sudo add-apt-repository -y ppa:linuxuprising/java
    sudo apt-get update
    sudo apt-get install -y oracle-java21-installer
    
    # Verify installation
    if ! command_exists java; then
        echo "❌ Java installation failed"
        exit 1
    fi
}

# Function to install Gradle
install_gradle() {
    echo "Installing Gradle..."
    sudo apt-get install -y unzip
    wget -q https://services.gradle.org/distributions/gradle-8.5-bin.zip
    sudo unzip -d /opt/gradle gradle-8.5-bin.zip
    sudo ln -s /opt/gradle/gradle-8.5/bin/gradle /usr/bin/gradle
    rm gradle-8.5-bin.zip
    
    # Verify installation
    if ! command_exists gradle; then
        echo "❌ Gradle installation failed"
        exit 1
    fi
}

# Function to install Spring Boot CLI
install_spring_boot_cli() {
    echo "Installing Spring Boot CLI..."
    
    # Install required packages
    sudo apt-get install -y unzip wget
    
    # Create temporary directory
    TEMP_DIR=$(mktemp -d)
    cd "$TEMP_DIR" || exit 1
    
    # Download with retry and verification
    SPRING_VERSION="3.2.0"
    SPRING_URL="https://repo.spring.io/release/org/springframework/boot/spring-boot-cli/${SPRING_VERSION}/spring-boot-cli-${SPRING_VERSION}-bin.zip"
    
    echo "Downloading Spring Boot CLI..."
    for i in {1..3}; do
        if wget -q "$SPRING_URL" && [ -f "spring-boot-cli-${SPRING_VERSION}-bin.zip" ]; then
            break
        fi
        echo "Retrying download..."
        sleep 2
        if [ $i -eq 3 ]; then
            echo "❌ Failed to download Spring Boot CLI"
            exit 1
        fi
    done
    
    # Verify download
    if [ ! -f "spring-boot-cli-${SPRING_VERSION}-bin.zip" ]; then
        echo "❌ Spring Boot CLI download failed"
        exit 1
    fi
    
    # Clean up old installation
    sudo rm -rf /opt/spring* /usr/local/bin/spring
    
    # Install
    sudo unzip -q "spring-boot-cli-${SPRING_VERSION}-bin.zip" -d /opt/
    sudo ln -sf "/opt/spring-${SPRING_VERSION}/bin/spring" /usr/local/bin/spring
    
    # Clean up
    cd - || exit 1
    rm -rf "$TEMP_DIR"
    
    # Verify installation
    if ! command_exists spring; then
        echo "❌ Spring Boot CLI installation failed"
        exit 1
    fi
    
    # Verify version
    INSTALLED_VERSION=$(spring --version | grep -oP '\d+\.\d+\.\d+' || echo "0.0.0")
    if [ "$(echo "$INSTALLED_VERSION $SPRING_VERSION" | awk '{if ($1 < $2) print 1}')" ]; then
        echo "❌ Spring Boot CLI version mismatch. Expected $SPRING_VERSION, got $INSTALLED_VERSION"
        exit 1
    fi
    
    echo "✅ Spring Boot CLI $SPRING_VERSION installed successfully"
}

# Function to install Node.js and npm
install_node() {
    echo "Installing Node.js and npm..."
    
    # Remove old Node.js
    sudo apt-get remove -y nodejs npm
    sudo apt-get autoremove -y
    
    # Clean up old files
    sudo rm -rf /usr/local/bin/npm /usr/local/share/man/man1/node* ~/.npm ~/.node-gyp
    
    # Add NodeSource repository (Node.js 20.x for Next.js 14+)
    curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
    sudo apt-get install -y nodejs
    
    # Verify and upgrade npm
    sudo npm install -g npm@latest
    
    # Install required global packages
    sudo npm install -g typescript @types/node @types/react @types/react-dom
}

# Function to install Rust and Cargo
install_rust() {
    echo "Installing Rust and Cargo..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
    
    # Verify installation
    if ! command_exists cargo; then
        echo "❌ Rust/Cargo installation failed"
        exit 1
    fi
}

# Function to install MongoDB for Ubuntu 22.04
install_mongodb() {
    # Check Ubuntu version
    if ! grep -q "Ubuntu 22.04" /etc/os-release; then
        echo "❌ This script is optimized for Ubuntu 22.04"
        exit 1
    fi

    echo "Installing MongoDB on Ubuntu 22.04..."
    
    # Install required packages
    sudo apt-get install -y gnupg curl

    # Remove existing MongoDB keys and sources
    sudo rm -f /etc/apt/sources.list.d/mongodb*.list
    sudo rm -f /usr/share/keyrings/mongodb*.gpg
    
    # Import MongoDB public GPG key
    curl -fsSL https://www.mongodb.org/static/pgp/server-6.0.asc | \
        sudo gpg --dearmor -o /usr/share/keyrings/mongodb-server-6.0.gpg
    
    # Create list file for MongoDB
    echo "deb [ arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/mongodb-server-6.0.gpg ] https://repo.mongodb.org/apt/ubuntu jammy/mongodb-org/6.0 multiverse" | \
        sudo tee /etc/apt/sources.list.d/mongodb-org-6.0.list

    sudo apt-get update -y
    sudo apt-get install -y mongodb-org
    
    # Start MongoDB
    sudo systemctl daemon-reload
    sudo systemctl start mongod
    sudo systemctl enable mongod
    
    # Wait for MongoDB to start
    echo "Waiting for MongoDB to start..."
    for i in {1..30}; do
        if mongosh --eval "db.version()" >/dev/null 2>&1; then
            break
        fi
        sleep 1
        if [ $i -eq 30 ]; then
            echo "❌ MongoDB failed to start"
            exit 1
        fi
    done

    # Setup MongoDB for project
    echo "Setting up MongoDB for Parseltongue..."
    mongosh --eval '
        use parseltongue;
        db.createCollection("analysis_jobs");
        db.createCollection("analysis_results");
        db.createCollection("file_metadata");
        db.analysis_jobs.createIndex({ "createdAt": 1 });
        db.analysis_jobs.createIndex({ "status": 1 });
        db.file_metadata.createIndex({ "jobId": 1 });
        db.analysis_results.createIndex({ "jobId": 1 }, { unique: true });
    '
}

# Function to install all missing dependencies
install_missing_deps() {
    local missing=("$@")
    
    for dep in "${missing[@]}"; do
        echo "Installing $dep..."
        case $dep in
            "java")
                install_java
                ;;
            "gradle")
                install_gradle
                ;;
            "spring")
                install_spring_boot_cli
                ;;
            "node"|"npm")
                install_node
                ;;
            "cargo")
                install_rust
                ;;
            "mongodb"|"mongosh")
                install_mongodb
                ;;
        esac
    done
}

# Function to setup Next.js frontend
setup_frontend() {
    echo "Setting up Next.js frontend..."
    (
        cd frontend || exit 1
        npx create-next-app@latest . --typescript --tailwind --eslint --app --src-dir --import-alias "@/*"
        
        # Create component directories
        for comp in AnalysisDashboard FileUpload BackendSelector ProgressBar ResultsView; do
            mkdir -p "src/components/$comp"
            touch "src/components/$comp/index.tsx"
        done
        
        # Create service files
        for service in analysisApi progressApi resultsApi; do
            touch "src/services/$service.ts"
        done
        
        # Install additional dependencies
        npm install @tanstack/react-query@latest
    )
}

# Function to setup Java backend
setup_java_backend() {
    echo "Setting up Java backend..."
    (
        cd backend-java || exit 1
        
        # Create Spring Boot project structure
        mkdir -p src/main/java/com/parseltongue/{config,controller,model,repository,service}
        
        # Create build.gradle with Spring Boot 3.x
        cat > build.gradle << 'EOL'
plugins {
    id 'java'
    id 'org.springframework.boot' version '3.2.0'
    id 'io.spring.dependency-management' version '1.1.4'
}

group = 'com.parseltongue'
version = '0.1.0'
sourceCompatibility = '21'

repositories {
    mavenCentral()
}

dependencies {
    implementation 'org.springframework.boot:spring-boot-starter-web'
    implementation 'org.springframework.boot:spring-boot-starter-data-mongodb'
    implementation 'org.springframework.boot:spring-boot-starter-actuator'
    compileOnly 'org.projectlombok:lombok'
    annotationProcessor 'org.projectlombok:lombok'
    testImplementation 'org.springframework.boot:spring-boot-starter-test'
}
EOL
    )
}

# Function to setup Rust backend
setup_rust_backend() {
    echo "Setting up Rust backend..."
    (
        cd backend-rust || exit 1
        cargo init
        
        # Configure Cargo.toml with required dependencies
        cat > Cargo.toml << 'EOL'
[package]
name = "parseltongue-rust"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1.35", features = ["full"] }
actix-web = "4.4"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
mongodb = "2.8"
futures = "0.3"
env_logger = "0.10"
log = "0.4"
EOL

        # Create Rust module structure
        mkdir -p src/{config,handlers,models,services}
        touch src/config/{app,db}.rs
        touch src/handlers/{analysis,health}.rs
        touch src/models/{job,result}.rs
        touch src/services/{analyzer,file}.rs
    )
}

# Function to setup MongoDB
setup_mongodb() {
    echo "Setting up MongoDB..."
    
    # Create MongoDB configuration
    sudo mkdir -p /etc/mongod/
    cat > /tmp/mongod.conf << 'EOL'
storage:
  dbPath: /var/lib/mongodb
  journal:
    enabled: true

systemLog:
  destination: file
  logAppend: true
  path: /var/log/mongodb/mongod.log

net:
  port: 27017
  bindIp: 127.0.0.1

security:
  authorization: disabled
EOL
    sudo mv /tmp/mongod.conf /etc/mongod/mongod.conf

    # Create required collections and indexes
    mongosh --eval '
        use parseltongue;
        db.createCollection("analysis_jobs");
        db.createCollection("analysis_results");
        db.createCollection("file_metadata");
        db.analysis_jobs.createIndex({ "createdAt": 1 });
        db.analysis_jobs.createIndex({ "status": 1 });
        db.file_metadata.createIndex({ "jobId": 1 });
        db.analysis_results.createIndex({ "jobId": 1 }, { unique: true });
    '
}

# Function to setup development scripts
setup_dev_scripts() {
    mkdir -p scripts
    
    # Create dev-commands script
    cat > scripts/dev-commands.sh << 'EOL'
#!/bin/bash
case "$1" in
    start)
        echo "Starting services..."
        (cd frontend && npm run dev) &
        (cd backend-java && ./gradlew bootRun) &
        (cd backend-rust && cargo run) &
        ;;
    clean)
        echo "Cleaning builds..."
        (cd frontend && rm -rf .next node_modules)
        (cd backend-java && ./gradlew clean)
        (cd backend-rust && cargo clean)
        ;;
    *)
        echo "Usage: $0 {start|clean}"
        exit 1
esac
EOL
    chmod +x scripts/dev-commands.sh
}

# Function to verify system requirements
verify_system_requirements() {
    echo "🔍 Verifying system requirements..."
    
    # Check Ubuntu version
    if ! grep -q "Ubuntu 22.04" /etc/os-release; then
        echo "❌ This script is optimized for Ubuntu 22.04"
        exit 1
    fi

    # Check minimum required versions
    local required_versions=(
        "node:20.0.0"
        "java:21"
        "rust:1.70.0"
    )

    for req in "${required_versions[@]}"; do
        local tool="${req%%:*}"
        local min_version="${req#*:}"
        
        if ! verify_tool_version "$tool" "$min_version"; then
            echo "❌ $tool version check failed. Minimum required: $min_version"
            exit 1
        fi
    done
}

# Function to verify tool versions
verify_tool_version() {
    local tool=$1
    local min_version=$2
    
    case $tool in
        "node")
            if command_exists node; then
                local version=$(node -v | cut -d'v' -f2)
                if [ "$(echo "$version $min_version" | awk '{if ($1 < $2) print 1}')" ]; then
                    return 1
                fi
            else
                return 1
            fi
            ;;
        "java")
            if command_exists java; then
                local version=$(java -version 2>&1 | head -n 1 | cut -d'"' -f2 | cut -d'.' -f1)
                if [ "$version" -lt "$min_version" ]; then
                    return 1
                fi
            else
                return 1
            fi
            ;;
        "rust")
            if command_exists rustc; then
                local version=$(rustc --version | cut -d' ' -f2)
                if [ "$(echo "$version $min_version" | awk '{if ($1 < $2) print 1}')" ]; then
                    return 1
                fi
            else
                return 1
            fi
            ;;
    esac
    return 0
}

# Main installation process
main() {
    # Verify system requirements first
    verify_system_requirements
    
    # Check for missing dependencies
    echo "🔍 Checking prerequisites..."
    MISSING_DEPS=()
    
    # Check required tools
    for cmd in node npm java gradle spring cargo mongodb mongosh; do
        if ! command_exists "$cmd"; then
            MISSING_DEPS+=("$cmd")
        fi
    done

    # Install missing dependencies
    if [ "${#MISSING_DEPS[@]}" -ne 0 ]; then
        echo "Missing tools: ${MISSING_DEPS[*]}"
        install_missing_deps "${MISSING_DEPS[@]}"
    fi

    # Create project structure
    mkdir -p frontend/{src/{app,components,services,types},config} \
           backend-java/src/main/java/com/parseltongue/{config,controller,model,repository,service} \
           backend-rust/src/{config,handlers,models,services} \
           shared/{types,config}

    # Setup each component
    setup_frontend
    setup_java_backend
    setup_rust_backend
    setup_mongodb
    setup_dev_scripts

    echo "✅ Installation complete!"
    echo "Next steps:"
    echo "1. Start services: ./scripts/dev-commands.sh start"
    echo "2. Access frontend: http://localhost:3000"
    echo "3. Java backend: http://localhost:8080"
    echo "4. Rust backend: http://localhost:8081"

    # === Final Summary ===
    echo "
=== Installation Summary ===
🔍 System Requirements:
- Ubuntu 22.04: ${grep -q "Ubuntu 22.04" /etc/os-release && echo "✅" || echo "❌"}
- Node.js 20+: $(node -v 2>/dev/null || echo "❌")
- Java 21+: $(java -version 2>&1 | head -n 1 | cut -d'"' -f2 || echo "❌")
- Rust 1.70+: $(rustc --version 2>/dev/null || echo "❌")

🏗️ Project Structure:
- Frontend (Next.js): $(test -d frontend/src && echo "✅" || echo "❌")
- Backend (Java): $(test -d backend-java/src && echo "✅" || echo "❌")
- Backend (Rust): $(test -d backend-rust/src && echo "✅" || echo "❌")
- Shared Resources: $(test -d shared && echo "✅" || echo "❌")

🔧 Development Tools:
- Spring Boot CLI: $(command -v spring >/dev/null 2>&1 && echo "✅" || echo "❌")
- Gradle: $(command -v gradle >/dev/null 2>&1 && echo "✅" || echo "❌")
- MongoDB: $(mongosh --version >/dev/null 2>&1 && echo "✅" || echo "❌")

📦 Dependencies Status:
Frontend:
- Next.js: $(test -f frontend/package.json && grep -q "next" frontend/package.json && echo "✅" || echo "❌")
- React Query: $(test -f frontend/package.json && grep -q "@tanstack/react-query" frontend/package.json && echo "✅" || echo "❌")
- Tailwind: $(test -f frontend/tailwind.config.js && echo "✅" || echo "❌")

Backend (Java):
- Spring Boot: $(test -f backend-java/build.gradle && grep -q "org.springframework.boot" backend-java/build.gradle && echo "✅" || echo "❌")
- MongoDB Driver: $(test -f backend-java/build.gradle && grep -q "spring-boot-starter-data-mongodb" backend-java/build.gradle && echo "✅" || echo "❌")

Backend (Rust):
- Actix-web: $(test -f backend-rust/Cargo.toml && grep -q "actix-web" backend-rust/Cargo.toml && echo "✅" || echo "❌")
- MongoDB Driver: $(test -f backend-rust/Cargo.toml && grep -q "mongodb" backend-rust/Cargo.toml && echo "✅" || echo "❌")

🗄️ Database Setup:
- MongoDB Service: $(systemctl is-active mongod >/dev/null 2>&1 && echo "✅" || echo "❌")
- Collections Created: $(mongosh --eval "use parseltongue; db.getCollectionNames().length" --quiet >/dev/null 2>&1 && echo "✅" || echo "❌")

📝 Next Steps:
1. Start services: ./scripts/dev-commands.sh start
2. Access frontend: http://localhost:3000
3. Java backend: http://localhost:8080
4. Rust backend: http://localhost:8081

⚠️ Note: Any ❌ marks indicate components that may need manual attention.
==========================="
}

# Run the script
main "$@"
