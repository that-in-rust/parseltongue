#!/bin/bash

# === Agenda Items ===
# 🎯 Project: Parseltongue v0.1.3
# 
# 📦 Installation Tasks:
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

# Initialize global arrays and constants
declare -r REQUIRED_UBUNTU_VERSION="22.04"
declare -r MIN_DISK_SPACE=5120
declare -r TIMEOUT_SECONDS=30
declare -r MAX_RETRIES=3
declare -g TEMP_DIR=""
declare -r NODE_VERSION="20"
declare -r NEXT_VERSION="14"
declare -r PNPM_VERSION="8.15.1"
declare -r SPRING_BOOT_VERSION="3.2.0"
declare -r JAVA_VERSION="21"
declare -r RUST_VERSION="1.70"

# Robust shell options
set -Eeuo pipefail
IFS=$'\n\t'

# Enhanced debug mode with line numbers
if [[ "${DEBUG:-}" == "true" ]]; then
    export PS4='+(${BASH_SOURCE}:${LINENO}): ${FUNCNAME[0]:+${FUNCNAME[0]}(): }'
    set -x
fi

# Function to handle errors with line numbers
handle_error() {
    local line_no=$1
    local exit_code=$2
    log_error "Error occurred in script $0 at line $line_no with exit code $exit_code"
}

# Function declarations
cleanup() {
    trap - SIGINT SIGTERM ERR EXIT
    if [[ -n "${TEMP_DIR:-}" ]] && [[ -d "$TEMP_DIR" ]]; then 
        rm -rf "$TEMP_DIR"
    fi
    jobs -p | xargs -r kill
    if [[ -f "/tmp/mongod.conf.backup" ]]; then
        sudo mv /tmp/mongod.conf.backup /etc/mongod/mongod.conf
    fi
}

# Error handling
trap cleanup SIGINT SIGTERM ERR EXIT
trap 'handle_error ${LINENO} $?' ERR

# Get script location
script_dir=$(cd "$(dirname "${BASH_SOURCE[0]}")" &>/dev/null && pwd -P)
export script_dir

# Enhanced logging functions with timestamps
log_info() {
    echo -e "\e[36m[$(date +'%Y-%m-%d %H:%M:%S')] INFO: $1\e[0m"
}

log_success() {
    echo -e "\e[32m[$(date +'%Y-%m-%d %H:%M:%S')] SUCCESS: $1\e[0m"
}

log_error() {
    echo -e "\e[31m[$(date +'%Y-%m-%d %H:%M:%S')] ERROR: $1\e[0m" >&2
}

log_warn() {
    echo -e "\e[33m[$(date +'%Y-%m-%d %H:%M:%S')] WARNING: $1\e[0m"
}

log_debug() {
    if [[ "${DEBUG:-}" == "true" ]]; then
        echo -e "\e[35m[$(date +'%Y-%m-%d %H:%M:%S')] DEBUG: $1\e[0m"
    fi
}

# Function to check Ubuntu version
check_ubuntu_version() {
    log_info "Checking Ubuntu version..."
    if ! grep -q "Ubuntu ${REQUIRED_UBUNTU_VERSION}" /etc/os-release; then
        log_error "This script requires Ubuntu ${REQUIRED_UBUNTU_VERSION}"
        return 1
    fi
    log_success "Ubuntu version check passed"
    return 0
}

# Function to check sudo with timeout
check_sudo() {
    log_info "Checking sudo privileges..."
    if ! timeout 10 sudo -v &>/dev/null; then
        log_error "This script requires sudo privileges. Please run as a user with sudo access."
        return 1
    fi
    log_success "Sudo access verified"
    return 0
}

# Enhanced permissions check
check_permissions() {
    log_info "Checking permissions..."
    local required_dirs=("$script_dir" "/etc" "/usr/local/bin")
    
    for dir in "${required_dirs[@]}"; do
        if [[ ! -w "$dir" ]] && ! sudo -n test -w "$dir"; then
            log_error "Insufficient permissions to write in $dir"
            return 1
        fi
    done
    log_success "Permissions check passed"
    return 0
}

# Enhanced disk space check
check_disk_space() {
    log_info "Checking disk space..."
    local available_space
    available_space=$(df -m . | awk 'NR==2 {print $4}')
    
    log_debug "Available space: ${available_space}MB, Required: ${MIN_DISK_SPACE}MB"
    
    if [[ "$available_space" -lt "$MIN_DISK_SPACE" ]]; then
        log_error "Insufficient disk space. Required: ${MIN_DISK_SPACE}MB, Available: ${available_space}MB"
        return 1
    fi
    log_success "Sufficient disk space available"
    return 0
}

# Enhanced network check with curl fallback to wget
check_network() {
    log_info "Checking network connectivity..."
    local test_urls=(
        "https://www.google.com"
        "https://registry.npmjs.org"
        "https://repo1.maven.org/maven2"
        "https://repo.mongodb.org"
        "https://sh.rustup.rs"
        "https://deb.nodesource.com"
    )
    
    local failed_urls=()
    for url in "${test_urls[@]}"; do
        log_debug "Testing connection to $url"
        if ! timeout "$TIMEOUT_SECONDS" curl --silent --head --fail "$url" &>/dev/null; then
            if ! timeout "$TIMEOUT_SECONDS" wget --spider --quiet "$url" &>/dev/null; then
                failed_urls+=("$url")
            fi
        fi
    done
    
    if [[ ${#failed_urls[@]} -ne 0 ]]; then
        log_error "Cannot reach the following URLs:"
        printf '%s\n' "${failed_urls[@]}"
        return 1
    fi
    
    log_success "Network connectivity verified"
    return 0
}

# Enhanced safe download with checksum verification
safe_download() {
    local url="$1"
    local output="$2"
    local checksum_url="${3:-}"
    local max_retries="${4:-$MAX_RETRIES}"
    local retry_count=0
    
    log_info "Downloading $url to $output"
    
    while [[ "$retry_count" -lt "$max_retries" ]]; do
        if wget -q "$url" -O "$output"; then
            if [[ -n "$checksum_url" ]]; then
                if ! verify_checksum "$output" "$checksum_url"; then
                    log_error "Checksum verification failed"
                    return 1
                fi
            fi
            log_success "Download completed successfully"
            return 0
        fi
        retry_count=$((retry_count + 1))
        log_warn "Download failed, retrying... ($retry_count/$max_retries)"
        sleep 2
    done
    
    log_error "Download failed after $max_retries attempts"
    return 1
}

install_node() {
    log_info "Installing Node.js ${NODE_VERSION}..."
    
    if command -v node &>/dev/null; then
        local current_version
        current_version=$(node -v | cut -d 'v' -f 2)
        if [[ "${current_version}" == "${NODE_VERSION}"* ]]; then
            log_success "Node.js ${NODE_VERSION} is already installed"
            return 0
        fi
    fi

    curl -fsSL https://deb.nodesource.com/setup_${NODE_VERSION}.x | sudo -E bash -
    sudo apt-get install -y nodejs
    
    # Install pnpm and ensure it's available
    log_info "Installing pnpm..."
    curl -fsSL https://get.pnpm.io/install.sh | sh -
    export PNPM_HOME="/root/.local/share/pnpm"
    export PATH="$PNPM_HOME:$PATH"
    source ~/.bashrc
    
    # Verify installation
    if ! command -v pnpm &>/dev/null; then
        log_error "PNPM installation failed"
        return 1
    fi
    
    log_success "Node.js and pnpm installed successfully"
}

setup_frontend() {
    log_info "Setting up Next.js frontend..."
    
    # Create frontend directory
    local frontend_dir="${script_dir}/../../frontend"
    mkdir -p "$frontend_dir"
    cd "$frontend_dir" || exit 1
    
    # Initialize Next.js project with TypeScript and Tailwind
    pnpm create next-app@latest . \
        --typescript \
        --tailwind \
        --eslint \
        --app \
        --src-dir \
        --import-alias "@/*" \
        --no-git

    # Install additional dependencies
    pnpm add @heroicons/react @headlessui/react \
            @tailwindcss/forms @tailwindcss/typography \
            @tailwindcss/aspect-ratio clsx

    # Update tailwind.config.js
    cat > tailwind.config.js << 'EOL'
/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    './src/pages/**/*.{js,ts,jsx,tsx,mdx}',
    './src/components/**/*.{js,ts,jsx,tsx,mdx}',
    './src/app/**/*.{js,ts,jsx,tsx,mdx}',
  ],
  theme: {
    extend: {
      fontFamily: {
        sans: ['var(--font-inter)'],
      },
    },
  },
  plugins: [
    require('@tailwindcss/forms'),
    require('@tailwindcss/typography'),
    require('@tailwindcss/aspect-ratio'),
  ],
}
EOL

    # Create basic project structure
    mkdir -p src/{components,lib,hooks,types,utils}
    
    # Add .env files
    cat > .env.local << 'EOL'
NEXT_PUBLIC_API_URL=http://localhost:8080
EOL

    cat > .env.development << 'EOL'
NEXT_PUBLIC_API_URL=http://localhost:8080
EOL

    cat > .env.production << 'EOL'
NEXT_PUBLIC_API_URL=https://api.yourdomain.com
EOL

    log_success "Frontend setup completed"
}

setup_java_backend() {
    log_info "Setting up Spring WebFlux backend..."
    
    local backend_dir="${script_dir}/../../backend-java"
    mkdir -p "$backend_dir"
    cd "$backend_dir" || exit 1
    
    # Initialize Spring Boot project with WebFlux
    curl https://start.spring.io/starter.zip \
        -d type=gradle-project \
        -d language=java \
        -d bootVersion="${SPRING_BOOT_VERSION}" \
        -d baseDir=. \
        -d groupId=com.parseltongue \
        -d artifactId=backend \
        -d name=backend \
        -d description="Parseltongue Backend" \
        -d packageName=com.parseltongue \
        -d packaging=jar \
        -d javaVersion="${JAVA_VERSION}" \
        -d dependencies=webflux,data-mongodb-reactive,websocket,lombok \
        -o backend.zip
    
    unzip -q backend.zip
    rm backend.zip
    
    # Create required directories
    mkdir -p src/main/java/com/parseltongue/{config,controller,model,repository,service}
    
    log_success "Java backend setup completed"
}

setup_rust_backend() {
    log_info "Setting up Async Rust backend..."
    
    local backend_dir="${script_dir}/../../backend-rust"
    mkdir -p "$backend_dir"
    cd "$backend_dir" || exit 1
    
    # Initialize Rust project
    cargo init --bin .
    
    # Add dependencies to Cargo.toml
    cat > Cargo.toml << 'EOL'
[package]
name = "parseltongue-backend"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1.0", features = ["full"] }
actix-web = "4"
actix-rt = "2.8"
mongodb = { version = "2.6", features = ["tokio-runtime"] }
futures = "0.3"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio-stream = "0.1"
EOL

    # Create required directories
    mkdir -p src/{config,handlers,models,services}
    
    log_success "Rust backend setup completed"
}

setup_shared_types() {
    log_info "Setting up shared types..."
    
    local shared_dir="${script_dir}/../../shared"
    mkdir -p "$shared_dir/types"
    
    # Create async types file
    cat > "$shared_dir/types/async.ts" << 'EOL'
export interface AnalysisProgress {
    jobId: string;
    progress: number;
    status: 'PENDING' | 'PROCESSING' | 'COMPLETED' | 'FAILED';
}

export interface ProgressEvent {
    type: 'PROGRESS' | 'COMPLETION' | 'ERROR';
    payload: any;
}

export interface FileChunk {
    id: string;
    data: Uint8Array;
    offset: number;
}
EOL
    
    log_success "Shared types setup completed"
}

# Main function
main() {
    log_info "Starting installation process..."
    
    # Initial checks
    check_ubuntu_version || exit 1
    check_sudo || exit 1
    check_permissions || exit 1
    check_network || exit 1
    check_disk_space || exit 1
    
    log_success "All preliminary checks passed"
    
    # Install dependencies
    install_node || exit 1
    
    # Setup all components
    setup_frontend || exit 1
    setup_java_backend || exit 1
    setup_rust_backend || exit 1
    setup_shared_types || exit 1
    
    log_success "Installation completed successfully"
}

# Run the script
main "$@"
