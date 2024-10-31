#!/bin/bash

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
    sudo apt-get install -y unzip
    wget -q https://repo.spring.io/release/org/springframework/boot/spring-boot-cli/3.2.0/spring-boot-cli-3.2.0-bin.zip
    sudo unzip spring-boot-cli-3.2.0-bin.zip -d /opt/
    sudo ln -s /opt/spring-3.2.0/bin/spring /usr/local/bin/spring
    rm spring-boot-cli-3.2.0-bin.zip
    
    # Verify installation
    if ! command_exists spring; then
        echo "❌ Spring Boot CLI installation failed"
        exit 1
    fi
}

# Function to install Node.js and npm
install_node() {
    echo "Installing Node.js and npm..."
    
    # Remove old Node.js if exists
    sudo apt-get remove -y nodejs npm
    sudo apt-get autoremove -y
    
    # Clean up any old Node.js related files
    sudo rm -rf /usr/local/bin/npm /usr/local/share/man/man1/node* /usr/local/lib/dtrace/node.d ~/.npm ~/.node-gyp /opt/local/bin/node /opt/local/include/node /opt/local/lib/node_modules
    
    # Add NodeSource repository
    curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
    
    # Install Node.js (which includes npm)
    sudo apt-get install -y nodejs
    
    # Verify Node.js installation
    if ! command_exists node; then
        echo "❌ Node.js installation failed"
        exit 1
    fi
    
    # Verify npm installation specifically
    if ! command_exists npm; then
        echo "npm not found after Node.js installation. Attempting direct npm installation..."
        sudo apt-get install -y npm
        
        if ! command_exists npm; then
            echo "❌ npm installation failed"
            exit 1
        fi
    fi
    
    # Install global packages
    echo "Installing global npm packages..."
    sudo npm install -g typescript @types/node
    
    # Display versions
    echo "Node.js version: $(node -v)"
    echo "npm version: $(npm -v)"
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

# Main installation process
main() {
    # Check Ubuntu version first
    if ! grep -q "Ubuntu 22.04" /etc/os-release; then
        echo "❌ This script is optimized for Ubuntu 22.04"
        exit 1
    fi

    # Update package list
    sudo apt-get update

    # Initial prerequisites check
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

    # Re-enable strict error checking for the rest of the script
    set -e

    # Verify prerequisites
    echo "🔍 Checking prerequisites..."
    MISSING_DEPS=()

    # Check required tools
    for cmd in node npm java gradle cargo mongodb; do
        if ! command_exists "$cmd"; then
            MISSING_DEPS+=("$cmd")
        fi
    done

    if [ "${#MISSING_DEPS[@]}" -ne 0 ]; then
        echo "❌ Missing required tools: ${MISSING_DEPS[*]}"
        echo "Please install them before continuing."
        exit 1
    fi

    # Version checks with proper quoting
    NODE_VERSION=$(node -v | cut -d'v' -f2)
    JAVA_VERSION=$(java -version 2>&1 | head -n 1 | cut -d'"' -f2 | cut -d'.' -f1)
    RUST_VERSION=$(rustc --version | cut -d' ' -f2)

    # Verify minimum versions with proper comparison
    if [ "$(echo "$NODE_VERSION 20.0.0" | awk '{if ($1 < $2) print 1}')" ]; then
        echo "❌ Node.js 20+ required, found $NODE_VERSION"
        exit 1
    fi

    if [ "$JAVA_VERSION" -lt 21 ]; then
        echo "❌ Java 21+ required, found $JAVA_VERSION"
        exit 1
    fi

    if [ "$(echo "$RUST_VERSION 1.70.0" | awk '{if ($1 < $2) print 1}')" ]; then
        echo "❌ Rust 1.70+ required, found $RUST_VERSION"
        exit 1
    fi

    echo "✅ All prerequisites met"

    # Create project structure with proper quoting
    echo "📁 Creating project structure..."
    mkdir -p frontend/{src/{app,components,services,types},config} \
           backend-java/src/main/java/com/parseltongue/{config,controller,model,repository,service} \
           backend-rust/src/{config,handlers,models,services} \
           shared/{types,config}

    # Frontend setup
    echo "⚛️ Setting up Next.js frontend..."
    (
        cd frontend || exit 1
        npm init -y
        npm install next@latest react@latest react-dom@latest typescript@latest @types/react@latest \
            @types/node@latest @types/react-dom@latest tailwindcss@latest postcss@latest \
            autoprefixer@latest @tanstack/react-query@latest
        npx create-next-app@latest . --typescript --tailwind --eslint --app --src-dir --import-alias "@/*" --no-git --use-npm

        # Create frontend component structure
        for comp in AnalysisDashboard FileUpload BackendSelector ProgressBar ResultsView; do
            mkdir -p "src/components/$comp"
            touch "src/components/$comp/index.tsx"
        done

        # Create frontend service files
        for service in analysisApi progressApi resultsApi; do
            touch "src/services/$service.ts"
        done
    )

    # Java backend setup
    echo "☕ Setting up Java backend..."
    (
        cd backend-java || exit 1
        gradle init --type java-application --test-framework junit-jupiter --dsl groovy \
            --project-name parseltongue-java --package com.parseltongue

        # Create Spring Boot configuration
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

    # Rust backend setup
    echo "🦀 Setting up Rust backend..."
    (
        cd backend-rust || exit 1
        cargo init

        # Configure Rust dependencies
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

        # Create Rust module files
        for module in config handlers models services; do
            mkdir -p "src/$module"
        done

        touch src/config/{app,db}.rs
        touch src/handlers/{analysis,health}.rs
        touch src/models/{job,result}.rs
        touch src/services/{analyzer,file}.rs
    )

    # Create shared resources
    echo "🤝 Setting up shared resources..."
    (
        cd shared || exit 1
        touch types/index.ts
        touch config/default.json
    )

    # Create development scripts
    echo "📜 Creating development scripts..."
    mkdir -p scripts
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

    # Create validation script
    cat > scripts/validate-structure.sh << 'EOL'
#!/bin/bash
echo "Validating project structure..."
# Add validation logic here
EOL
    chmod +x scripts/validate-structure.sh

    # MongoDB configuration
    echo "🍃 Configuring MongoDB..."
    cat > shared/config/mongodb.json << 'EOL'
{
    "uri": "mongodb://localhost:27017",
    "database": "parseltongue",
    "collections": {
        "jobs": "analysis_jobs",
        "results": "analysis_results",
        "files": "file_metadata"
    }
}
EOL

    # Create MongoDB indexes
    echo "Creating MongoDB indexes..."
    mongosh --eval '
        use parseltongue;
        db.analysis_jobs.createIndex({ "createdAt": 1 });
        db.analysis_jobs.createIndex({ "status": 1 });
        db.file_metadata.createIndex({ "jobId": 1 });
        db.analysis_results.createIndex({ "jobId": 1 }, { unique: true });
    '

    echo "✅ Installation complete!"
    echo "📝 Next steps:"
    echo "1. Configure MongoDB connection"
    echo "2. Run './scripts/dev-commands.sh start' to start all services"
    echo "3. Access the application at http://localhost:3000"
}

# Run the script
main "$@"
