# 🚀 Parseltongue Distribution

## Grab and Go - No Installation Required

### **What You Get**
- `binaries/parseltongue` - 4.3MB standalone binary
- `copy-paste-ready/` - Scripts, prompts, and Kiro templates
- `steering-integration/` - Integration guides

### **30-Second Setup**
```bash
# 1. Make executable
chmod +x binaries/parseltongue

# 2. Test it works  
./binaries/parseltongue --version

# 3. Analyze your project
./binaries/parseltongue onboard .
```

### **Copy-Paste Integration**

#### **For Any Project**
```bash
# Copy binary to project root
cp binaries/parseltongue /your/project/
cd /your/project
./parseltongue onboard .
```

#### **For Kiro Projects**
```bash
# Copy complete steering template
cp copy-paste-ready/kiro-steering-complete.md /your/project/.kiro/steering/parseltongue.md
```

#### **For LLM Workflows**
```bash
# Use the ready-made prompts
cat copy-paste-ready/llm-prompts.md
# Copy prompts for Claude, ChatGPT, Cursor, etc.
```

### **What's Inside**

#### **`binaries/`**
- `parseltongue` - The main binary (works on macOS, Linux, Windows)

#### **`copy-paste-ready/`**
- `pt-wrapper.sh` - Convenient wrapper script
- `llm-prompts.md` - Ready-made prompts for AI tools
- `kiro-steering-complete.md` - Complete Kiro integration

#### **`steering-integration/`**
- `README.md` - Integration guide
- `kiro-steering-template.md` - Basic template

### **Zero Dependencies**
- ✅ No Rust installation needed
- ✅ No package managers
- ✅ No configuration files
- ✅ Just download and run

Perfect for teams who want architectural intelligence without setup complexity.