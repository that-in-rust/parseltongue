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

# Robust shell options
set -Eeuo pipefail
IFS=$'\n\t'

# Enable debug mode if requested
if [[ "${DEBUG:-}" == "true" ]]; then
    set -x
fi

# Error handling
trap cleanup SIGINT SIGTERM ERR EXIT

# Get script location
script_dir=$(cd "$(dirname "${BASH_SOURCE[0]}")" &>/dev/null && pwd -P)

# Cleanup function
cleanup() {
    trap - SIGINT SIGTERM ERR EXIT
    # Add cleanup code here
}

# Path safety function
safe_cd() {
    cd "$1" || exit 1
}

# Logging functions
log_info() { echo "ℹ️ $*" >&2; }
log_warn() { echo "⚠️ $*" >&2; }
log_error() { echo "❌ $*" >&2; }
log_success() { echo "✅ $*" >&2; }

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to verify tool versions
verify_tool_version() {
    local tool=$1
    local min_version=$2
    local version_check_failed=false
    
    case $tool in
        "node")
            if command_exists node; then
                local version=$(node -v | cut -d'v' -f2)
                if [ "$(echo "$version $min_version" | awk '{if ($1 < $2) print 1}')" ]; then
                    version_check_failed=true
                fi
            else
                version_check_failed=true
            fi
            ;;
        "java")
            if command_exists java; then
                local version=$(java -version 2>&1 | head -n 1 | cut -d'"' -f2 | cut -d'.' -f1)
                if [ "$version" -lt "$min_version" ]; then
                    version_check_failed=true
                fi
            else
                version_check_failed=true
            fi
            ;;
        "rust")
            if command_exists rustc; then
                local version=$(rustc --version | cut -d' ' -f2)
                if [ "$(echo "$version $min_version" | awk '{if ($1 < $2) print 1}')" ]; then
                    version_check_failed=true
                fi
            else
                version_check_failed=true
            fi
            ;;
        "mongodb")
            if command_exists mongod; then
                local version=$(mongod --version | grep "db version" | cut -d' ' -f3)
                if [ "$(echo "$version $min_version" | awk '{if ($1 < $2) print 1}')" ]; then
                    version_check_failed=true
                fi
            else
                version_check_failed=true
            fi
            ;;
    esac

    [ "$version_check_failed" = false ]
    return $?
}

# Function to verify system requirements
verify_system_requirements() {
    log_info "🔍 Pre-checking required tools..."
    
    # Check Ubuntu version
    if ! grep -q "Ubuntu 22.04" /etc/os-release; then
        log_error "This script is optimized for Ubuntu 22.04"
        exit 1
    fi

    # First check for existence and install if missing
    local required_tools=(node npm java gradle spring cargo mongodb mongosh)
    MISSING_DEPS=()
    
    for tool in "${required_tools[@]}"; do
        if ! command_exists "$tool"; then
            log_warn "$tool is missing, will be installed"
            MISSING_DEPS+=("$tool")
        fi
    done

    # Install missing dependencies first
    if [ ${#MISSING_DEPS[@]} -ne 0 ]; then
        log_info "Installing missing tools: ${MISSING_DEPS[*]}"
        install_missing_deps "${MISSING_DEPS[@]}"
    fi

    # Now check versions after ensuring tools exist
    log_info "Verifying tool versions..."
    local required_versions=(
        "node:20.0.0"
        "java:21"
        "rust:1.70.0"
        "mongodb:6.0"
    )

    for req in "${required_versions[@]}"; do
        local tool="${req%%:*}"
        local min_version="${req#*:}"
        
        if ! verify_tool_version "$tool" "$min_version"; then
            log_error "$tool version check failed. Minimum required: $min_version"
            exit 1
        fi
    done

    log_success "All system requirements verified"
}

# Function to install missing dependencies
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

# Function to verify installation
verify_installation() {
    local component=$1
    local version=$2
    local retries=3
    local verified=false

    echo "🔍 Verifying $component installation..."

    for ((i=1; i<=retries; i++)); do
        if verify_tool_version "$component" "$version"; then
            verified=true
            break
        fi
        echo "⚠️ Verification attempt $i failed, retrying..."
        sleep 2
    done

    if ! $verified; then
        echo "❌ Failed to verify $component installation after $retries attempts"
        return 1
    fi

    echo "✅ $component installation verified"
    return 0
}

# Function to setup frontend
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

# Function to generate summary
generate_summary() {
    echo "=== Summary ==="
    echo "🔍 Available Components:"
    for cmd in node npm java gradle spring cargo mongodb mongosh; do
        if command_exists "$cmd"; then
            echo "✅ $cmd: $(command -v "$cmd")"
            case $cmd in
                "node") echo "   Version: $(node -v)" ;;
                "java") echo "   Version: $(java -version 2>&1 | head -n 1)" ;;
                "mongodb") echo "   Version: $(mongod --version | grep "db version")" ;;
                "cargo") echo "   Version: $(cargo --version)" ;;
            esac
        else
            echo "❌ $cmd: Not available"
        fi
    done

    echo -e "\n📦 Installation Status:"
    echo "Frontend Setup: $(test -d frontend/src && echo "✅" || echo "❌")"
    echo "Java Backend: $(test -d backend-java/src && echo "✅" || echo "❌")"
    echo "Rust Backend: $(test -d backend-rust/src && echo "✅" || echo "❌")"
    echo "MongoDB: $(systemctl is-active mongod >/dev/null 2>&1 && echo "✅" || echo "❌")"

    echo -e "\n🚫 Uninstalled Components:"
    for cmd in "${MISSING_DEPS[@]}"; do
        echo "- $cmd"
    done

    echo -e "\n🚧 Blocked On:"
    if [ ${#failed_verifications[@]} -ne 0 ]; then
        printf '%s\n' "${failed_verifications[@]}"
    else
        echo "No blocking issues"
    fi

    echo -e "\n🔧 Needs Fixing:"
    local needs_fixing=()
    # Check frontend setup
    if [ ! -f frontend/package.json ]; then
        needs_fixing+=("Frontend package.json missing")
    fi
    # Check Java backend setup
    if [ ! -f backend-java/build.gradle ]; then
        needs_fixing+=("Java backend build.gradle missing")
    fi
    # Check Rust backend setup
    if [ ! -f backend-rust/Cargo.toml ]; then
        needs_fixing+=("Rust backend Cargo.toml missing")
    fi
    # Check MongoDB
    if ! mongosh --eval "db.version()" --quiet >/dev/null 2>&1; then
        needs_fixing+=("MongoDB connection issues")
    fi

    if [ ${#needs_fixing[@]} -eq 0 ]; then
        echo "No issues found"
    else
        printf '%s\n' "${needs_fixing[@]}"
    fi
}

# Function to install Java
install_java() {
    log_info "Installing Java 21..."
    
    # Check if already installed with correct version
    if command_exists java && [[ $(java -version 2>&1 | head -n 1) == *"21"* ]]; then
        log_success "Java 21 is already installed"
        return 0
    }

    # Try multiple installation methods
    if command_exists apt; then
        sudo apt-get update
        sudo apt-get install -y software-properties-common
        sudo add-apt-repository -y ppa:linuxuprising/java
        sudo apt-get update
        
        # Handle interactive prompt
        echo "oracle-java21-installer shared/accepted-oracle-license-v1-3 select true" | \
            sudo debconf-set-selections
        
        sudo apt-get install -y oracle-java21-installer
    else
        log_error "Package manager not supported"
        exit 1
    fi

    # Verify installation
    if ! command_exists java; then
        log_error "Java installation failed"
        exit 1
    fi
}

# Function to install Gradle
install_gradle() {
    log_info "Installing Gradle..."
    
    # Check if already installed
    if command_exists gradle; then
        log_success "Gradle is already installed"
        return 0
    }

    # Install required packages
    sudo apt-get install -y unzip

    # Download and install Gradle with retry mechanism
    local GRADLE_VERSION="8.5"
    local MAX_RETRIES=3
    local retry_count=0
    
    while [ $retry_count -lt $MAX_RETRIES ]; do
        if wget -q "https://services.gradle.org/distributions/gradle-${GRADLE_VERSION}-bin.zip"; then
            break
        fi
        retry_count=$((retry_count + 1))
        log_warn "Download failed, retrying... ($retry_count/$MAX_RETRIES)"
        sleep 2
    done

    if [ $retry_count -eq $MAX_RETRIES ]; then
        log_error "Failed to download Gradle"
        exit 1
    fi

    sudo unzip -q "gradle-${GRADLE_VERSION}-bin.zip" -d /opt/gradle
    sudo ln -sf "/opt/gradle/gradle-${GRADLE_VERSION}/bin/gradle" /usr/bin/gradle
    rm "gradle-${GRADLE_VERSION}-bin.zip"

    # Verify installation
    if ! command_exists gradle; then
        log_error "Gradle installation failed"
        exit 1
    fi
}

# Function to install Spring Boot CLI
install_spring_boot_cli() {
    log_info "Installing Spring Boot CLI..."
    
    # Check if already installed
    if command_exists spring; then
        log_success "Spring Boot CLI is already installed"
        return 0
    }

    # Install required packages
    sudo apt-get install -y unzip wget

    # Create temporary directory
    local TEMP_DIR=$(mktemp -d)
    safe_cd "$TEMP_DIR"

    # Download Spring Boot CLI with retry mechanism
    local SPRING_VERSION="3.2.0"
    local SPRING_URL="https://repo.spring.io/release/org/springframework/boot/spring-boot-cli/${SPRING_VERSION}/spring-boot-cli-${SPRING_VERSION}-bin.zip"
    local MAX_RETRIES=3
    local retry_count=0

    while [ $retry_count -lt $MAX_RETRIES ]; do
        if wget -q "$SPRING_URL"; then
            break
        fi
        retry_count=$((retry_count + 1))
        log_warn "Download failed, retrying... ($retry_count/$MAX_RETRIES)"
        sleep 2
    done

    if [ $retry_count -eq $MAX_RETRIES ]; then
        log_error "Failed to download Spring Boot CLI"
        safe_cd -
        rm -rf "$TEMP_DIR"
        exit 1
    fi

    # Clean up old installation
    sudo rm -rf /opt/spring* /usr/local/bin/spring

    # Install
    sudo unzip -q "spring-boot-cli-${SPRING_VERSION}-bin.zip" -d /opt/
    sudo ln -sf "/opt/spring-${SPRING_VERSION}/bin/spring" /usr/local/bin/spring

    # Clean up
    safe_cd -
    rm -rf "$TEMP_DIR"

    # Verify installation
    if ! command_exists spring; then
        log_error "Spring Boot CLI installation failed"
        exit 1
    fi
}

# Function to install Node.js
install_node() {
    log_info "Installing Node.js..."
    
    # Check if already installed with correct version
    if command_exists node && [[ $(node -v) == *"20"* ]]; then
        log_success "Node.js 20 is already installed"
        return 0
    }

    # Remove old versions
    sudo apt-get remove -y nodejs npm || true
    sudo apt-get autoremove -y

    # Clean up old files
    sudo rm -rf /usr/local/bin/npm /usr/local/share/man/man1/node* ~/.npm ~/.node-gyp

    # Add NodeSource repository
    curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -

    # Install Node.js
    sudo apt-get install -y nodejs

    # Upgrade npm
    sudo npm install -g npm@latest

    # Install global packages
    sudo npm install -g typescript @types/node @types/react @types/react-dom

    # Verify installation
    if ! command_exists node || ! command_exists npm; then
        log_error "Node.js installation failed"
        exit 1
    fi
}

# Function to install Rust
install_rust() {
    log_info "Installing Rust..."
    
    # Check if already installed
    if command_exists rustc && command_exists cargo; then
        log_success "Rust is already installed"
        return 0
    }

    # Download and install Rust with retry mechanism
    local MAX_RETRIES=3
    local retry_count=0

    while [ $retry_count -lt $MAX_RETRIES ]; do
        if curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y; then
            break
        fi
        retry_count=$((retry_count + 1))
        log_warn "Installation failed, retrying... ($retry_count/$MAX_RETRIES)"
        sleep 2
    done

    if [ $retry_count -eq $MAX_RETRIES ]; then
        log_error "Failed to install Rust"
        exit 1
    fi

    # Source cargo environment
    source "$HOME/.cargo/env"

    # Verify installation
    if ! command_exists rustc || ! command_exists cargo; then
        log_error "Rust installation failed"
        exit 1
    fi
}

# Function to install MongoDB
install_mongodb() {
    log_info "Installing MongoDB..."
    
    # Check if already installed
    if command_exists mongod && systemctl is-active --quiet mongod; then
        log_success "MongoDB is already installed and running"
        return 0
    }

    # Install required packages
    sudo apt-get install -y gnupg curl

    # Remove existing MongoDB keys and sources
    sudo rm -f /etc/apt/sources.list.d/mongodb*.list
    sudo rm -f /usr/share/keyrings/mongodb*.gpg

    # Import MongoDB public GPG key with retry mechanism
    local MAX_RETRIES=3
    local retry_count=0

    while [ $retry_count -lt $MAX_RETRIES ]; do
        if curl -fsSL https://www.mongodb.org/static/pgp/server-6.0.asc | \
            sudo gpg --dearmor -o /usr/share/keyrings/mongodb-server-6.0.gpg; then
            break
        fi
        retry_count=$((retry_count + 1))
        log_warn "Key import failed, retrying... ($retry_count/$MAX_RETRIES)"
        sleep 2
    done

    if [ $retry_count -eq $MAX_RETRIES ]; then
        log_error "Failed to import MongoDB GPG key"
        exit 1
    fi

    # Create list file for MongoDB
    echo "deb [ arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/mongodb-server-6.0.gpg ] https://repo.mongodb.org/apt/ubuntu jammy/mongodb-org/6.0 multiverse" | \
        sudo tee /etc/apt/sources.list.d/mongodb-org-6.0.list

    sudo apt-get update
    sudo apt-get install -y mongodb-org

    # Start MongoDB
    sudo systemctl daemon-reload
    sudo systemctl start mongod
    sudo systemctl enable mongod

    # Wait for MongoDB to start
    log_info "Waiting for MongoDB to start..."
    local start_timeout=30
    local counter=0
    while ! mongosh --eval "db.version()" >/dev/null 2>&1; do
        sleep 1
        counter=$((counter + 1))
        if [ $counter -eq $start_timeout ]; then
            log_error "MongoDB failed to start within $start_timeout seconds"
            exit 1
        fi
    done

    # Verify installation
    if ! command_exists mongod || ! systemctl is-active --quiet mongod; then
        log_error "MongoDB installation failed"
        exit 1
    fi
}

# Main installation process
main() {
    # Verify system requirements first
    verify_system_requirements
    
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

    # Run post-installation verification
    echo "🔍 Running post-installation verifications..."
    local failed_verifications=()
    
    for component in "node:20.0.0" "java:21" "mongodb:6.0"; do
        if ! verify_installation "${component%%:*}" "${component#*:}"; then
            failed_verifications+=("$component")
        fi
    done

    if [ ${#failed_verifications[@]} -ne 0 ]; then
        echo "❌ The following components failed verification:"
        printf '%s\n' "${failed_verifications[@]}"
        exit 1
    fi

    # Generate summary
    generate_summary
}

# Run the script
main "$@"
