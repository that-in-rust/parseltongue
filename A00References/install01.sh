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

# Robust shell options
set -Eeuo pipefail
IFS=$'\n\t'

# Enable debug mode if requested
[[ "${DEBUG:-}" == "true" ]] && set -x

# Function declarations (move all functions before main)
cleanup() {
    trap - SIGINT SIGTERM ERR EXIT
    [ -n "${TEMP_DIR:-}" ] && [ -d "$TEMP_DIR" ] && rm -rf "$TEMP_DIR"
    jobs -p | xargs -r kill
    [ -f "/tmp/mongod.conf.backup" ] && sudo mv /tmp/mongod.conf.backup /etc/mongod/mongod.conf
}

# Error handling
trap cleanup SIGINT SIGTERM ERR EXIT
trap 'handle_error ${LINENO} $?' ERR  # Ensure error handling captures line number and exit status

# Get script location
script_dir=$(cd "$(dirname "${BASH_SOURCE[0]}")" &>/dev/null && pwd -P)
export script_dir

# Function to check Ubuntu version using REQUIRED_UBUNTU_VERSION constant
check_ubuntu_version() {
    if ! grep -q "Ubuntu ${REQUIRED_UBUNTU_VERSION}" /etc/os-release; then
        log_error "This script requires Ubuntu ${REQUIRED_UBUNTU_VERSION}"
        return 1
    fi
    return 0
}

# Update disk space check to use MIN_DISK_SPACE constant
check_disk_space() {
    log_info "Checking disk space..."
    local available_space
    available_space=$(df -m . | awk 'NR==2 {print $4}')
    
    if [ "$available_space" -lt "$MIN_DISK_SPACE" ]; then
        log_error "Insufficient disk space. Required: ${MIN_DISK_SPACE}MB, Available: ${available_space}MB"
        return 1
    fi
    log_success "Sufficient disk space available"
    return 0
}

# Update network check to use TIMEOUT_SECONDS constant
check_network() {
    log_info "Checking network connectivity..."
    local test_urls=(
        "https://www.google.com"
        "https://registry.npmjs.org"
        "https://repo.spring.io"
        "https://repo.mongodb.org"
        "https://sh.rustup.rs"
        "https://deb.nodesource.com"
    )
    
    local failed_urls=()
    for url in "${test_urls[@]}"; do
        if ! timeout "$TIMEOUT_SECONDS" curl --silent --head --fail "$url" >/dev/null; then
            failed_urls+=("$url")
        fi
    done
    
    if [ ${#failed_urls[@]} -ne 0 ]; then
        log_error "Cannot reach the following URLs:"
        printf '%s\n' "${failed_urls[@]}"
        return 1
    fi
    
    log_success "Network connectivity verified"
    return 0
}

# Update safe_download to properly quote variables
safe_download() {
    local url="$1"
    local output="$2"
    local max_retries="${3:-$MAX_RETRIES}"
    local retry_count=0
    
    while [ "$retry_count" -lt "$max_retries" ]; do
        if wget -q "$url" -O "$output"; then
            return 0
        fi
        retry_count=$((retry_count + 1))
        log_warn "Download failed, retrying... ($retry_count/$max_retries)"
        sleep 2
    done
    
    return 1
}

# Function to check if the user has sudo privileges
check_sudo() {
    if ! sudo -v &>/dev/null; then
        log_error "This script requires sudo privileges. Please run as a user with sudo access."
        return 1
    fi
    log_success "Sudo access verified"
    return 0
}

# Main function updates
main() {
    # Initial checks using constants
    check_ubuntu_version || exit 1
    check_sudo || exit 1  # Ensure this function is defined
    check_permissions || exit 1
    check_network || exit 1
    check_disk_space || exit 1
    
    # Rest of main function remains the same...
}

# Run the script
main "$@"
