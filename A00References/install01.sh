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

set -e

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
        "mongodb:6.0"
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
