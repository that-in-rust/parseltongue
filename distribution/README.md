# Parseltongue v0.1.0 Distribution

## 🚀 **Ready-to-Use Distribution**

### **Quick Start**
```bash
# Make executable (if needed)
chmod +x binaries/parseltongue

# Test the binary
./binaries/parseltongue --version
./binaries/parseltongue --help

# Start using immediately
./binaries/parseltongue onboard .
```

### **📦 Contents**

#### **binaries/**
- `parseltongue` - Optimized release binary (4.3MB)
- No dependencies required
- Works on macOS, Linux, Windows

#### **steering-integration/**
- `README.md` - Complete integration guide
- `kiro-steering-template.md` - Ready-to-use Kiro steering template
- Examples and best practices

### **🎯 Integration Approaches**

#### **1. Standalone Usage**
```bash
# Copy to your PATH
cp binaries/parseltongue /usr/local/bin/

# Use anywhere
parseltongue onboard /path/to/project
```

#### **2. Project-Specific**
```bash
# Keep in project root
./distribution/binaries/parseltongue onboard .
```

#### **3. Kiro Steering Integration**
Copy `steering-integration/kiro-steering-template.md` to your `.kiro/steering/` folder.

### **🔧 No Setup Required**
- ✅ Single binary, no installation
- ✅ No runtime dependencies  
- ✅ No configuration files needed
- ✅ Works immediately after download

### **📋 Capabilities**
- **Discovery**: List entities, find definitions, browse codebase
- **Analysis**: Blast radius, dependencies, caller traces
- **Workflows**: Onboarding, feature planning, debug assistance
- **Integration**: JSON output, markdown generation, LLM context

### **⚡ Performance**
- Entity listing: <100ms
- Blast radius queries: <500μs
- File analysis: <2s for large files
- Complete onboarding: <15 minutes

Perfect for steering documentation integration - lightweight, fast, and focused.