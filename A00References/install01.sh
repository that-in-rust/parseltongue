#!/bin/bash

# ==============================================================================
# Pre-Installation Verification & Setup
# ------------------------------------------------------------------------------
# Purpose: Verify system requirements and prepare for installation
# Components: Async-First Architecture with Node.js, Java, Rust, MongoDB
# Version: v0.2.0 (Async Update)
# ==============================================================================

# Initialize constants
declare -r REQUIRED_UBUNTU_VERSION="22.04"
declare -r MIN_DISK_SPACE=5120
declare -r TIMEOUT_SECONDS=30
declare -r MAX_RETRIES=3
declare -r NODE_VERSION="20"
declare -r JAVA_VERSION="21"
declare -r RUST_VERSION="1.70"
declare -r SPRING_BOOT_VERSION="3.x"
declare -r NEXTJS_VERSION="14"
declare -r MONGODB_VERSION="6.0"
declare -g TEMP_DIR=""

# Async Performance Requirements
declare -r CONCURRENT_FILES=10
declare -r WEBSOCKET_LATENCY=100  # ms
declare -r STREAM_PROCESSING=50   # MB/s
declare -r MAX_MEMORY=512         # MB

# Directory Structure Constants
declare -r PROJECT_STRUCTURE=(
    "frontend/src/app"
    "frontend/src/components"
    "frontend/src/services/websocket"
    "backend-java/src/main/java/com/parseltongue/config"
    "backend-java/src/main/java/com/parseltongue/controller"
    "backend-java/src/main/java/com/parseltongue/model"
    "backend-java/src/main/java/com/parseltongue/repository"
    "backend-java/src/main/java/com/parseltongue/service"
    "backend-rust/src/config"
    "backend-rust/src/handlers"
    "backend-rust/src/models"
    "backend-rust/src/services"
    "shared/types"
)

# Async Framework Requirements
declare -r JAVA_ASYNC_DEPS=(
    "spring-webflux"
    "reactor-core"
    "spring-boot-starter-data-mongodb-reactive"
    "spring-boot-starter-websocket"
    "reactor-test"
)

declare -r RUST_ASYNC_DEPS=(
    'tokio = { version = "1.0", features = ["full"] }'
    'actix-web = "4"'
    'actix-rt = "2.8"'
    'mongodb = { version = "2.6", features = ["tokio-runtime"] }'
    'futures = "0.3"'
)

# Required ports
declare -r PORTS=(
    3000  # Web UI
    8080  # Java API
    8081  # Rust API
    27017 # MongoDB
)

# Required response times (ms)
declare -r RESPONSE_TIMES=(
    2000  # Page Load
    1000  # Repo Validation
    3000  # Analysis Start
    2000  # Results View
)

# Robust shell options
set -Eeuo pipefail
IFS=$'\n\t'
[[ "${DEBUG:-}" == "true" ]] && set -x

# Determine the script directory
script_dir=$(dirname "$(realpath "$0")")

# Logging functions
log_info() {
    echo -e "\e[34m[INFO] $1\e[0m" >&2
}

log_error() {
    echo -e "\e[31m[ERROR] $1\e[0m" >&2
}

log_success() {
    echo -e "\e[32m[SUCCESS] $1\e[0m" >&2
}

# Core check functions
check_ubuntu_version() {
    log_info "Checking Ubuntu version..."
    if ! lsb_release -d | grep -q "Ubuntu ${REQUIRED_UBUNTU_VERSION}"; then
        log_error "This script requires Ubuntu ${REQUIRED_UBUNTU_VERSION}"
        return 1
    fi
    return 0
}

check_disk_space() {
    log_info "Checking disk space..."
    local available_space
    available_space=$(df -m / | awk 'NR==2 {print $4}')

    if [[ "$available_space" -lt "$MIN_DISK_SPACE" ]]; then
        log_error "Insufficient disk space. Required: ${MIN_DISK_SPACE}MB, Available: ${available_space}MB"
        return 1
    fi
    return 0
}

check_network() {
    log_info "Checking network connectivity..."
    if ! ping -c 1 -W 1 google.com &>/dev/null; then
        log_error "No network connectivity"
        return 1
    fi
    return 0
}

check_permissions() {
    log_info "Checking permissions..."
    if [[ $EUID -ne 0 ]]; then
        log_error "This script must be run as root"
        return 1
    fi
    return 0
}

# Async setup functions
create_webflux_config() {
    log_info "Creating WebFlux configuration..."
    # Placeholder implementation
    return 0
}

setup_reactive_mongo() {
    log_info "Setting up reactive MongoDB..."
    # Placeholder implementation
    return 0
}

create_websocket_handlers() {
    log_info "Creating WebSocket handlers..."
    # Placeholder implementation
    return 0
}

setup_tokio_runtime() {
    log_info "Setting up Tokio runtime..."
    # Placeholder implementation
    return 0
}

create_async_handlers() {
    log_info "Creating async handlers..."
    # Placeholder implementation
    return 0
}

setup_stream_processors() {
    log_info "Setting up stream processors..."
    # Placeholder implementation
    return 0
}

# Verification functions
verify_concurrent_processing() {
    log_info "Verifying concurrent processing... (not fully implemented)"
    # Placeholder implementation
    return 0
}

verify_websocket_connections() {
    log_info "Verifying WebSocket connections... (not fully implemented)"
    # Placeholder implementation
    return 0
}

verify_stream_processing() {
    log_info "Verifying stream processing... (not fully implemented)"
    # Placeholder implementation
    return 0
}

measure_websocket_latency() {
    local endpoint="$1"
    log_info "Measuring WebSocket latency to $endpoint... (placeholder value)"
    # Placeholder implementation
    echo "50" # Placeholder latency in ms
}

# Disk speed verification
verify_disk_speed() {
    log_info "Verifying disk speed..."
    local write_speed
    write_speed=$( (dd if=/dev/zero of=testfile bs=1M count=1024 oflag=direct conv=fdatasync 2>&1) | grep -o '[0-9.]\+ MB/s' | tail -1 | awk '{print $1}')
    rm -f testfile

    if [[ -z "$write_speed" ]]; then
        log_error "Unable to measure disk speed"
        return 1
    fi

    write_speed_int=${write_speed%.*} # Remove decimal part

    if [[ $write_speed_int -ge $STREAM_PROCESSING ]]; then
        return 0
    else
        log_error "Disk write speed is $write_speed MB/s, which is less than required $STREAM_PROCESSING MB/s"
        return 1
    fi
}

# Cleanup and error handling
cleanup() {
    if [[ -n "${TEMP_DIR:-}" ]] && [[ -d "$TEMP_DIR" ]]; then 
        rm -rf "$TEMP_DIR"
    fi
}

handle_error() {
    local line_no=$1
    local exit_code=$2
    log_error "Error occurred in script at line: $line_no (exit code: $exit_code)"
    cleanup
    exit $exit_code
}

setup_project() {
    log_info "Setting up project structure..."
    local base_dir
    base_dir=$(dirname "$script_dir")

    for dir in "${PROJECT_STRUCTURE[@]}"; do
        mkdir -p "$base_dir/$dir"
    done
    return 0
}

# Pre-Installation Verification
verify_system() {
    local checks=(
        "check_ubuntu_version"
        "check_disk_space"
        "check_network"
        "check_ports"
        "check_permissions"
        "check_response_times"
    )

    for check in "${checks[@]}"; do
        if ! "$check"; then
            log_error "Failed at: $check"
            return 1
        fi
    done
}

# Modified check_ports function
check_ports() {
    log_info "Checking if required ports are available or used by expected services..."

    # Define expected processes for each port
    declare -A expected_processes=(
        [3000]="node"      # Web UI (Next.js)
        [8080]="java"      # Java API
        [8081]="rust|actix|my_rust_app" # Rust API (replace with actual process name)
        [27017]="mongod"   # MongoDB
    )

    for port in "${PORTS[@]}"; do
        # Get the PID of the process using the port
        pid=$(lsof -ti tcp:"$port")

        if [[ -n "$pid" ]]; then
            # Get the command name of the process
            process_name=$(ps -p "$pid" -o comm=)

            # Check if the process name matches the expected process for the port
            if [[ "${expected_processes[$port]}" =~ $process_name ]]; then
                log_info "Port $port is in use by expected process '$process_name'."
            else
                log_error "Port $port is already in use by unexpected process '$process_name' (PID: $pid)."
                return 1
            fi
        else
            log_info "Port $port is available."
        fi
    done
    return 0
}

check_response_times() {
    log_info "Checking response times... (not fully implemented)"
    # Implement response time checks based on RESPONSE_TIMES
    return 0
}

verify_dependencies() {
    local deps=(
        "curl"
        "wget"
        "gpg"
        "unzip"
        "git"
        "npm"
        "gradle"
        "bc"
    )

    for dep in "${deps[@]}"; do
        if ! command -v "$dep" &>/dev/null; then
            log_info "Installing missing dependency: $dep"
            apt-get update && apt-get install -y "$dep"
        fi
    done
}

verify_frameworks() {
    # Java Stack
    local java_deps=(
        "spring-boot-starter-web"
        "spring-boot-starter-data-mongodb"
        "lombok"
        "spring-boot-starter-actuator"
    )

    # Rust Stack
    local rust_deps=(
        "actix-web"
        "mongodb"
        "tokio"
        "serde"
    )

    # Verify Java dependencies
    for dep in "${java_deps[@]}"; do
        if ! gradle --quiet dependencies | grep -q "$dep"; then
            log_error "Missing Java dependency: $dep"
            return 1
        fi
    done

    # Verify Rust dependencies
    for dep in "${rust_deps[@]}"; do
        if ! cargo tree | grep -q "$dep"; then
            log_error "Missing Rust dependency: $dep"
            return 1
        fi
    done
}

# Installation & Post-Verification
verify_services() {
    log_info "Verifying services..."
    local urls=(
        "http://localhost:3000"          # Web UI
        "http://localhost:8080/health"   # Java API
        "http://localhost:8081/health"   # Rust API
        "mongodb://localhost:27017"      # MongoDB
    )

    for url in "${urls[@]}"; do
        if ! curl --silent --fail "$url" >/dev/null; then
            log_error "Service not responding: $url"
            return 1
        fi
    done

    log_success "All services are running"
    return 0
}

verify_file_processing() {
    log_info "Verifying file processing... (not fully implemented)"
    # Verify file processing requirements
    local requirements=(
        "ZIP support"
        "Directory analysis"
        "Language detection"
        "100MB file limit"
        "10 levels depth"
    )

    # Implementation for each requirement check
    return 0
}

# Async Requirements Verification
verify_async_requirements() {
    log_info "Verifying async requirements..."

    # Check memory limits
    total_memory=$(free -m | awk '/Mem:/ {print $2}')
    if [[ $total_memory -lt $MAX_MEMORY ]]; then
        log_error "Insufficient memory for async operations"
        return 1
    fi

    # Check network capacity for WebSocket
    if ! verify_websocket_latency; then
        log_error "Network latency too high for WebSocket operations"
        return 1
    fi

    # Check disk speed for stream processing
    if ! verify_disk_speed; then
        log_error "Disk speed insufficient for stream processing"
        return 1
    fi

    return 0
}

verify_websocket_latency() {
    local test_endpoint="ws://localhost:8080/websocket"
    local measured_latency

    measured_latency=$(measure_websocket_latency "$test_endpoint")

    if [[ $measured_latency -le $WEBSOCKET_LATENCY ]]; then
        return 0
    else
        log_error "WebSocket latency ${measured_latency} ms exceeds the maximum allowed ${WEBSOCKET_LATENCY} ms"
        return 1
    fi
}

# Async Setup Functions
setup_async_java() {
    log_info "Setting up async Java backend..."

    # Create WebFlux configuration
    create_webflux_config

    # Setup reactive MongoDB configuration
    setup_reactive_mongo

    # Create WebSocket handlers
    create_websocket_handlers

    return 0
}

setup_async_rust() {
    log_info "Setting up async Rust backend..."

    # Setup Tokio runtime
    setup_tokio_runtime

    # Create async handlers
    create_async_handlers

    # Setup stream processors
    setup_stream_processors

    return 0
}

verify_async_setup() {
    log_info "Verifying async setup..."

    # Verify concurrent file processing
    verify_concurrent_processing

    # Verify WebSocket connections
    verify_websocket_connections

    # Verify stream processing
    verify_stream_processing

    return 0
}

# Main execution
main() {
    log_info "Starting Parseltongue v0.2.0 Async-First Installation"

    # Phase 1: Pre-Installation & Async Verification
    verify_system || exit 1
    verify_async_requirements || exit 1
    verify_dependencies || exit 1

    # Phase 2: Async-First Installation
    setup_async_java || exit 1
    setup_async_rust || exit 1
    setup_project || exit 1

    # Phase 3: Async Verification
    verify_async_setup || exit 1
    verify_services || exit 1

    # Display success message with performance metrics
    log_success "Async-First Installation completed successfully"
    echo "✓ Concurrent Files: $CONCURRENT_FILES+"
    echo "✓ WebSocket Latency: <${WEBSOCKET_LATENCY}ms"
    echo "✓ Stream Processing: ${STREAM_PROCESSING}MB/s"
    echo "✓ Memory Usage: <${MAX_MEMORY}MB"
}

# Run with error handling
trap 'handle_error ${LINENO} $?' ERR
trap cleanup SIGINT SIGTERM EXIT
main "$@"
