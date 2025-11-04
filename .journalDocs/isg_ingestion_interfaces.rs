// ISG Ingestion Interfaces - Idiomatic Functional Rust Implementation
// Following Test-Driven Development and Executable Specification Principles

use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use thiserror::Error;
use anyhow::Result;

// ================================================================================
// Core Domain Models (L1 - No External Dependencies)
// ================================================================================

/// Core AST trait that all language-specific ASTs must implement
pub trait Ast: std::fmt::Debug + Clone {
    fn node_type(&self) -> NodeType;
    fn children(&self) -> Vec<&dyn Ast>;
    fn source_range(&self) -> SourceRange;
    fn accept<V: AstVisitor>(&self, visitor: &mut V) -> V::Output;
}

/// Visitor pattern for AST traversal (functional approach)
pub trait AstVisitor {
    type Output;
    
    fn visit_node(&mut self, node: &dyn Ast) -> Self::Output;
    fn visit_children(&mut self, node: &dyn Ast) -> Vec<Self::Output> {
        node.children().iter().map(|child| child.accept(self)).collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SourceRange {
    pub start: usize,
    pub end: usize,
    pub file_id: FileId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FileId(pub u32);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeType {
    // C/C++ node types
    TranslationUnit,
    Function,
    Variable,
    Struct,
    Class,
    Template,
    Namespace,
    
    // Ruby/Rails node types
    Module,
    RubyClass,
    Method,
    Block,
    Symbol,
    
    // Common node types
    Expression,
    Statement,
    Declaration,
    Identifier,
    Literal,
}

// ================================================================================
// Parser Traits (Dependency Injection Pattern)
// ================================================================================

/// Main parser trait - all parsers must implement this
pub trait Parser: Send + Sync {
    type Input: AsRef<str>;
    type Output: Ast;
    type Error: std::error::Error + Send + Sync + 'static;
    type Config: Default;
    
    fn parse(&self, input: Self::Input) -> Result<Self::Output, Self::Error>;
    fn parse_with_config(&self, input: Self::Input, config: Self::Config) 
        -> Result<Self::Output, Self::Error>;
}

/// Incremental parsing capability
pub trait IncrementalParser: Parser {
    type Cache: ParseCache;
    type Delta: AstDelta;
    
    fn parse_incremental(
        &self,
        input: Self::Input,
        cache: &mut Self::Cache,
        edits: &[TextEdit],
    ) -> Result<Self::Delta, Self::Error>;
    
    fn apply_delta(&self, ast: &Self::Output, delta: Self::Delta) 
        -> Result<Self::Output, Self::Error>;
}

/// Parse cache for incremental parsing
pub trait ParseCache: Default {
    type Key: Eq + std::hash::Hash;
    type Value: Clone;
    
    fn get(&self, key: &Self::Key) -> Option<&Self::Value>;
    fn insert(&mut self, key: Self::Key, value: Self::Value);
    fn invalidate(&mut self, key: &Self::Key);
    fn clear(&mut self);
}

#[derive(Debug, Clone)]
pub struct TextEdit {
    pub range: SourceRange,
    pub new_text: String,
}

/// Delta representation for incremental updates
pub trait AstDelta {
    type Node: Ast;
    
    fn additions(&self) -> &[Self::Node];
    fn deletions(&self) -> &[NodeId];
    fn modifications(&self) -> &[(NodeId, Self::Node)];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(pub u64);

// ================================================================================
// ISG (Incremental Semantic Graph) Core Types
// ================================================================================

/// Main ISG structure - immutable, persistent data structure
#[derive(Debug, Clone)]
pub struct IncrementalSemanticGraph {
    nodes: Arc<HashMap<NodeId, SemanticNode>>,
    edges: Arc<HashMap<EdgeId, SemanticEdge>>,
    indices: Arc<SemanticIndices>,
    version: Version,
}

#[derive(Debug, Clone)]
pub struct SemanticNode {
    pub id: NodeId,
    pub kind: SemanticKind,
    pub name: String,
    pub source: SourceRange,
    pub attributes: HashMap<String, AttributeValue>,
}

#[derive(Debug, Clone)]
pub enum SemanticKind {
    // Types
    Type { is_primitive: bool },
    Class { is_abstract: bool },
    Interface,
    Trait,
    
    // Functions/Methods
    Function { is_static: bool },
    Method { is_virtual: bool },
    Constructor,
    Destructor,
    
    // Variables
    Variable { is_const: bool },
    Field { is_static: bool },
    Parameter,
    
    // Namespaces/Modules
    Namespace,
    Module,
    Package,
    
    // Rails-specific
    Model,
    Controller,
    Route,
    Migration,
}

#[derive(Debug, Clone)]
pub struct SemanticEdge {
    pub id: EdgeId,
    pub kind: EdgeKind,
    pub from: NodeId,
    pub to: NodeId,
    pub attributes: HashMap<String, AttributeValue>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EdgeId(pub u64);

#[derive(Debug, Clone)]
pub enum EdgeKind {
    // Inheritance
    Extends,
    Implements,
    Mixes,
    
    // Containment
    Contains,
    Declares,
    Defines,
    
    // References
    Calls,
    Uses,
    Imports,
    Depends,
    
    // Data flow
    Reads,
    Writes,
    Assigns,
    
    // Rails associations
    HasMany,
    BelongsTo,
    HasOne,
    HasAndBelongsToMany,
}

#[derive(Debug, Clone)]
pub enum AttributeValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    List(Vec<AttributeValue>),
    Map(HashMap<String, AttributeValue>),
}

/// Semantic indices for fast lookups
#[derive(Debug, Clone)]
pub struct SemanticIndices {
    by_name: HashMap<String, Vec<NodeId>>,
    by_kind: HashMap<SemanticKind, Vec<NodeId>>,
    by_source: HashMap<FileId, Vec<NodeId>>,
    edges_from: HashMap<NodeId, Vec<EdgeId>>,
    edges_to: HashMap<NodeId, Vec<EdgeId>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Version(pub u64);

// ================================================================================
// ISG Builder Trait
// ================================================================================

/// Main trait for building ISG from AST
pub trait IsgBuilder {
    type Ast: Ast;
    type Error: std::error::Error;
    
    fn build(&mut self, ast: &Self::Ast) -> Result<IncrementalSemanticGraph, Self::Error>;
    fn update(&mut self, graph: &IncrementalSemanticGraph, delta: &dyn AstDelta) 
        -> Result<IncrementalSemanticGraph, Self::Error>;
}

// ================================================================================
// Language-Specific Implementations
// ================================================================================

// C Parser Implementation
pub mod c {
    use super::*;
    
    #[derive(Debug, Clone)]
    pub struct CAst {
        pub root: TranslationUnit,
    }
    
    #[derive(Debug, Clone)]
    pub struct TranslationUnit {
        pub declarations: Vec<Declaration>,
        pub source_range: SourceRange,
    }
    
    #[derive(Debug, Clone)]
    pub enum Declaration {
        Function(FunctionDecl),
        Variable(VariableDecl),
        Typedef(TypedefDecl),
        Struct(StructDecl),
    }
    
    #[derive(Debug, Clone)]
    pub struct FunctionDecl {
        pub name: String,
        pub return_type: Type,
        pub parameters: Vec<Parameter>,
        pub body: Option<CompoundStatement>,
        pub source_range: SourceRange,
    }
    
    #[derive(Debug, Clone)]
    pub struct Type {
        pub base: BaseType,
        pub qualifiers: Vec<TypeQualifier>,
        pub pointer_level: usize,
    }
    
    #[derive(Debug, Clone)]
    pub enum BaseType {
        Void,
        Char,
        Short,
        Int,
        Long,
        Float,
        Double,
        Struct(String),
        Typedef(String),
    }
    
    #[derive(Debug, Clone)]
    pub enum TypeQualifier {
        Const,
        Volatile,
        Restrict,
    }
    
    pub struct CParser {
        preprocessor: PreprocessorCache,
        symbol_table: SymbolTable,
    }
    
    impl Parser for CParser {
        type Input = String;
        type Output = CAst;
        type Error = CParseError;
        type Config = CParseConfig;
        
        fn parse(&self, input: Self::Input) -> Result<Self::Output, Self::Error> {
            // Implementation following Option 1, 2, or 3 from design doc
            todo!()
        }
        
        fn parse_with_config(&self, input: Self::Input, config: Self::Config) 
            -> Result<Self::Output, Self::Error> {
            todo!()
        }
    }
    
    #[derive(Debug, Error)]
    pub enum CParseError {
        #[error("Lexical error: {0}")]
        LexicalError(String),
        
        #[error("Syntax error: {0}")]
        SyntaxError(String),
        
        #[error("Semantic error: {0}")]
        SemanticError(String),
        
        #[error("Preprocessor error: {0}")]
        PreprocessorError(String),
    }
    
    #[derive(Debug, Default)]
    pub struct CParseConfig {
        pub standard: CStandard,
        pub gnu_extensions: bool,
        pub include_paths: Vec<String>,
    }
    
    #[derive(Debug, Default)]
    pub enum CStandard {
        C89,
        C99,
        #[default]
        C11,
        C17,
    }
    
    pub struct PreprocessorCache {
        expanded: HashMap<FileId, String>,
        macros: HashMap<String, Macro>,
    }
    
    pub struct SymbolTable {
        scopes: Vec<Scope>,
        current_scope: usize,
    }
    
    #[derive(Debug, Clone)]
    pub struct Scope {
        symbols: HashMap<String, Symbol>,
        parent: Option<usize>,
    }
    
    #[derive(Debug, Clone)]
    pub enum Symbol {
        Type(Type),
        Variable(Type),
        Function(FunctionDecl),
    }
    
    #[derive(Debug, Clone)]
    pub struct Macro {
        pub name: String,
        pub parameters: Option<Vec<String>>,
        pub body: String,
    }
    
    #[derive(Debug, Clone)]
    pub struct Parameter {
        pub name: Option<String>,
        pub ty: Type,
    }
    
    #[derive(Debug, Clone)]
    pub struct CompoundStatement {
        pub statements: Vec<Statement>,
    }
    
    #[derive(Debug, Clone)]
    pub enum Statement {
        Expression(Expression),
        Compound(CompoundStatement),
        If(IfStatement),
        While(WhileStatement),
        Return(Option<Expression>),
    }
    
    #[derive(Debug, Clone)]
    pub struct Expression {
        pub kind: ExpressionKind,
        pub ty: Option<Type>,
    }
    
    #[derive(Debug, Clone)]
    pub enum ExpressionKind {
        Binary { op: BinaryOp, left: Box<Expression>, right: Box<Expression> },
        Unary { op: UnaryOp, operand: Box<Expression> },
        Call { func: Box<Expression>, args: Vec<Expression> },
        Identifier(String),
        Literal(Literal),
    }
    
    #[derive(Debug, Clone)]
    pub enum BinaryOp {
        Add, Sub, Mul, Div, Mod,
        Eq, Ne, Lt, Le, Gt, Ge,
        And, Or,
        Assign,
    }
    
    #[derive(Debug, Clone)]
    pub enum UnaryOp {
        Negate, Not, Address, Deref,
    }
    
    #[derive(Debug, Clone)]
    pub enum Literal {
        Integer(i64),
        Float(f64),
        String(String),
        Char(char),
    }
    
    #[derive(Debug, Clone)]
    pub struct VariableDecl {
        pub name: String,
        pub ty: Type,
        pub initializer: Option<Expression>,
        pub source_range: SourceRange,
    }
    
    #[derive(Debug, Clone)]
    pub struct TypedefDecl {
        pub name: String,
        pub ty: Type,
        pub source_range: SourceRange,
    }
    
    #[derive(Debug, Clone)]
    pub struct StructDecl {
        pub name: Option<String>,
        pub fields: Vec<Field>,
        pub source_range: SourceRange,
    }
    
    #[derive(Debug, Clone)]
    pub struct Field {
        pub name: String,
        pub ty: Type,
    }
    
    #[derive(Debug, Clone)]
    pub struct IfStatement {
        pub condition: Expression,
        pub then_branch: Box<Statement>,
        pub else_branch: Option<Box<Statement>>,
    }
    
    #[derive(Debug, Clone)]
    pub struct WhileStatement {
        pub condition: Expression,
        pub body: Box<Statement>,
    }
}

// C++ Parser Implementation
pub mod cpp {
    use super::*;
    use super::c::*;
    
    #[derive(Debug, Clone)]
    pub struct CppAst {
        pub root: CppTranslationUnit,
    }
    
    #[derive(Debug, Clone)]
    pub struct CppTranslationUnit {
        pub declarations: Vec<CppDeclaration>,
        pub source_range: SourceRange,
    }
    
    #[derive(Debug, Clone)]
    pub enum CppDeclaration {
        C(Declaration),  // Inherit C declarations
        Class(ClassDecl),
        Template(TemplateDecl),
        Namespace(NamespaceDecl),
        Using(UsingDecl),
    }
    
    #[derive(Debug, Clone)]
    pub struct ClassDecl {
        pub name: String,
        pub bases: Vec<BaseSpecifier>,
        pub members: Vec<Member>,
        pub source_range: SourceRange,
    }
    
    #[derive(Debug, Clone)]
    pub struct BaseSpecifier {
        pub access: AccessSpecifier,
        pub is_virtual: bool,
        pub base_type: Type,
    }
    
    #[derive(Debug, Clone)]
    pub enum AccessSpecifier {
        Public,
        Protected,
        Private,
    }
    
    #[derive(Debug, Clone)]
    pub enum Member {
        Field(FieldDecl),
        Method(MethodDecl),
        Constructor(ConstructorDecl),
        Destructor(DestructorDecl),
        TypeAlias(TypeAlias),
    }
    
    #[derive(Debug, Clone)]
    pub struct TemplateDecl {
        pub parameters: Vec<TemplateParameter>,
        pub declaration: Box<CppDeclaration>,
        pub source_range: SourceRange,
    }
    
    #[derive(Debug, Clone)]
    pub enum TemplateParameter {
        Type { name: String, default: Option<Type> },
        NonType { ty: Type, name: String, default: Option<Expression> },
        Template { parameters: Vec<TemplateParameter>, name: String },
    }
    
    pub struct CppParser {
        c_parser: CParser,
        template_cache: TemplateCache,
        overload_resolver: OverloadResolver,
    }
    
    impl Parser for CppParser {
        type Input = String;
        type Output = CppAst;
        type Error = CppParseError;
        type Config = CppParseConfig;
        
        fn parse(&self, input: Self::Input) -> Result<Self::Output, Self::Error> {
            todo!()
        }
        
        fn parse_with_config(&self, input: Self::Input, config: Self::Config) 
            -> Result<Self::Output, Self::Error> {
            todo!()
        }
    }
    
    #[derive(Debug, Error)]
    pub enum CppParseError {
        #[error("C parse error: {0}")]
        CError(#[from] CParseError),
        
        #[error("Template error: {0}")]
        TemplateError(String),
        
        #[error("Overload resolution failed: {0}")]
        OverloadError(String),
    }
    
    #[derive(Debug, Default)]
    pub struct CppParseConfig {
        pub standard: CppStandard,
        pub c_config: CParseConfig,
    }
    
    #[derive(Debug, Default)]
    pub enum CppStandard {
        Cpp98,
        Cpp03,
        Cpp11,
        Cpp14,
        Cpp17,
        #[default]
        Cpp20,
        Cpp23,
    }
    
    pub struct TemplateCache {
        instantiations: HashMap<(String, Vec<Type>), CppAst>,
    }
    
    pub struct OverloadResolver {
        candidates: Vec<FunctionDecl>,
    }
    
    #[derive(Debug, Clone)]
    pub struct FieldDecl {
        pub name: String,
        pub ty: Type,
        pub access: AccessSpecifier,
        pub is_static: bool,
        pub is_mutable: bool,
    }
    
    #[derive(Debug, Clone)]
    pub struct MethodDecl {
        pub name: String,
        pub return_type: Type,
        pub parameters: Vec<Parameter>,
        pub access: AccessSpecifier,
        pub is_static: bool,
        pub is_virtual: bool,
        pub is_const: bool,
        pub body: Option<CompoundStatement>,
    }
    
    #[derive(Debug, Clone)]
    pub struct ConstructorDecl {
        pub parameters: Vec<Parameter>,
        pub initializer_list: Vec<MemberInitializer>,
        pub body: CompoundStatement,
        pub access: AccessSpecifier,
    }
    
    #[derive(Debug, Clone)]
    pub struct MemberInitializer {
        pub member: String,
        pub init: Expression,
    }
    
    #[derive(Debug, Clone)]
    pub struct DestructorDecl {
        pub is_virtual: bool,
        pub body: CompoundStatement,
        pub access: AccessSpecifier,
    }
    
    #[derive(Debug, Clone)]
    pub struct TypeAlias {
        pub name: String,
        pub ty: Type,
    }
    
    #[derive(Debug, Clone)]
    pub struct NamespaceDecl {
        pub name: Option<String>,  // None for anonymous namespace
        pub declarations: Vec<CppDeclaration>,
    }
    
    #[derive(Debug, Clone)]
    pub struct UsingDecl {
        pub name: String,
        pub target: String,
    }
}

// Rails/Ruby Parser Implementation
pub mod rails {
    use super::*;
    
    #[derive(Debug, Clone)]
    pub struct RailsAst {
        pub root: RailsFile,
    }
    
    #[derive(Debug, Clone)]
    pub enum RailsFile {
        Model(ModelFile),
        Controller(ControllerFile),
        View(ViewFile),
        Migration(MigrationFile),
        Routes(RoutesFile),
        Ruby(RubyFile),
    }
    
    #[derive(Debug, Clone)]
    pub struct ModelFile {
        pub class_name: String,
        pub parent: String,
        pub associations: Vec<Association>,
        pub validations: Vec<Validation>,
        pub scopes: Vec<Scope>,
        pub methods: Vec<Method>,
    }
    
    #[derive(Debug, Clone)]
    pub enum Association {
        HasMany { name: String, options: HashMap<String, Value> },
        HasOne { name: String, options: HashMap<String, Value> },
        BelongsTo { name: String, options: HashMap<String, Value> },
        HasAndBelongsToMany { name: String, options: HashMap<String, Value> },
    }
    
    #[derive(Debug, Clone)]
    pub struct Validation {
        pub attribute: String,
        pub kind: ValidationKind,
        pub options: HashMap<String, Value>,
    }
    
    #[derive(Debug, Clone)]
    pub enum ValidationKind {
        Presence,
        Uniqueness,
        Length,
        Format,
        Inclusion,
        Custom(String),
    }
    
    #[derive(Debug, Clone)]
    pub struct Scope {
        pub name: String,
        pub body: Block,
    }
    
    #[derive(Debug, Clone)]
    pub struct Method {
        pub name: String,
        pub parameters: Vec<Parameter>,
        pub body: Block,
        pub visibility: Visibility,
    }
    
    #[derive(Debug, Clone)]
    pub enum Visibility {
        Public,
        Protected,
        Private,
    }
    
    #[derive(Debug, Clone)]
    pub struct Block {
        pub statements: Vec<Statement>,
    }
    
    #[derive(Debug, Clone)]
    pub enum Statement {
        Expression(Expression),
        Assignment(Assignment),
        MethodCall(MethodCall),
        Return(Option<Expression>),
        If(IfStatement),
        Unless(UnlessStatement),
        While(WhileStatement),
        For(ForStatement),
        Block(Block),
    }
    
    #[derive(Debug, Clone)]
    pub struct Expression {
        pub kind: ExpressionKind,
    }
    
    #[derive(Debug, Clone)]
    pub enum ExpressionKind {
        Literal(Value),
        Variable(String),
        MethodCall(MethodCall),
        Binary { op: String, left: Box<Expression>, right: Box<Expression> },
        Unary { op: String, operand: Box<Expression> },
    }
    
    #[derive(Debug, Clone)]
    pub enum Value {
        Nil,
        Boolean(bool),
        Integer(i64),
        Float(f64),
        String(String),
        Symbol(String),
        Array(Vec<Value>),
        Hash(HashMap<Value, Value>),
    }
    
    // Make Value hashable for use in HashMap keys
    impl std::hash::Hash for Value {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            match self {
                Value::Nil => 0.hash(state),
                Value::Boolean(b) => b.hash(state),
                Value::Integer(i) => i.hash(state),
                Value::Float(f) => f.to_bits().hash(state),
                Value::String(s) => s.hash(state),
                Value::Symbol(s) => s.hash(state),
                Value::Array(a) => a.hash(state),
                Value::Hash(_) => 0.hash(state), // Simplified
            }
        }
    }
    
    impl Eq for Value {}
    impl PartialEq for Value {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (Value::Nil, Value::Nil) => true,
                (Value::Boolean(a), Value::Boolean(b)) => a == b,
                (Value::Integer(a), Value::Integer(b)) => a == b,
                (Value::Float(a), Value::Float(b)) => a == b,
                (Value::String(a), Value::String(b)) => a == b,
                (Value::Symbol(a), Value::Symbol(b)) => a == b,
                (Value::Array(a), Value::Array(b)) => a == b,
                (Value::Hash(a), Value::Hash(b)) => a == b,
                _ => false,
            }
        }
    }
    
    pub struct RailsParser {
        ruby_parser: RubyParser,
        pattern_matcher: PatternMatcher,
    }
    
    impl Parser for RailsParser {
        type Input = String;
        type Output = RailsAst;
        type Error = RailsParseError;
        type Config = RailsParseConfig;
        
        fn parse(&self, input: Self::Input) -> Result<Self::Output, Self::Error> {
            todo!()
        }
        
        fn parse_with_config(&self, input: Self::Input, config: Self::Config) 
            -> Result<Self::Output, Self::Error> {
            todo!()
        }
    }
    
    #[derive(Debug, Error)]
    pub enum RailsParseError {
        #[error("Ruby parse error: {0}")]
        RubyError(String),
        
        #[error("Rails DSL error: {0}")]
        DslError(String),
        
        #[error("Pattern match failed: {0}")]
        PatternError(String),
    }
    
    #[derive(Debug, Default)]
    pub struct RailsParseConfig {
        pub rails_version: RailsVersion,
        pub strict_mode: bool,
    }
    
    #[derive(Debug, Default)]
    pub enum RailsVersion {
        Rails5,
        Rails6,
        #[default]
        Rails7,
    }
    
    pub struct RubyParser {
        lexer: RubyLexer,
    }
    
    pub struct RubyLexer {
        keywords: HashMap<String, TokenType>,
    }
    
    pub struct PatternMatcher {
        patterns: Vec<Pattern>,
    }
    
    #[derive(Debug, Clone)]
    pub struct Pattern {
        pub name: String,
        pub regex: String,
        pub captures: Vec<String>,
    }
    
    #[derive(Debug, Clone)]
    pub enum TokenType {
        Keyword(Keyword),
        Identifier,
        Symbol,
        String,
        Integer,
        Float,
        Operator,
    }
    
    #[derive(Debug, Clone)]
    pub enum Keyword {
        Class,
        Module,
        Def,
        End,
        If,
        Unless,
        While,
        For,
        Return,
    }
    
    #[derive(Debug, Clone)]
    pub struct Parameter {
        pub name: String,
        pub default: Option<Value>,
        pub kind: ParameterKind,
    }
    
    #[derive(Debug, Clone)]
    pub enum ParameterKind {
        Required,
        Optional,
        Rest,
        Keyword,
        Block,
    }
    
    #[derive(Debug, Clone)]
    pub struct Assignment {
        pub target: String,
        pub value: Expression,
    }
    
    #[derive(Debug, Clone)]
    pub struct MethodCall {
        pub receiver: Option<Box<Expression>>,
        pub method: String,
        pub arguments: Vec<Expression>,
        pub block: Option<Block>,
    }
    
    #[derive(Debug, Clone)]
    pub struct IfStatement {
        pub condition: Expression,
        pub then_branch: Block,
        pub else_branch: Option<Block>,
    }
    
    #[derive(Debug, Clone)]
    pub struct UnlessStatement {
        pub condition: Expression,
        pub body: Block,
    }
    
    #[derive(Debug, Clone)]
    pub struct WhileStatement {
        pub condition: Expression,
        pub body: Block,
    }
    
    #[derive(Debug, Clone)]
    pub struct ForStatement {
        pub variable: String,
        pub collection: Expression,
        pub body: Block,
    }
    
    #[derive(Debug, Clone)]
    pub struct ControllerFile {
        pub class_name: String,
        pub parent: String,
        pub filters: Vec<Filter>,
        pub actions: Vec<Action>,
    }
    
    #[derive(Debug, Clone)]
    pub struct Filter {
        pub kind: FilterKind,
        pub name: String,
        pub options: HashMap<String, Value>,
    }
    
    #[derive(Debug, Clone)]
    pub enum FilterKind {
        Before,
        After,
        Around,
    }
    
    #[derive(Debug, Clone)]
    pub struct Action {
        pub name: String,
        pub body: Block,
    }
    
    #[derive(Debug, Clone)]
    pub struct ViewFile {
        pub template_type: TemplateType,
        pub content: String,
        pub embedded_ruby: Vec<EmbeddedRuby>,
    }
    
    #[derive(Debug, Clone)]
    pub enum TemplateType {
        Html,
        Erb,
        Haml,
        Slim,
    }
    
    #[derive(Debug, Clone)]
    pub struct EmbeddedRuby {
        pub kind: ErbKind,
        pub code: String,
        pub position: usize,
    }
    
    #[derive(Debug, Clone)]
    pub enum ErbKind {
        Expression,  // <%= %>
        Statement,   // <% %>
        Comment,     // <%# %>
    }
    
    #[derive(Debug, Clone)]
    pub struct MigrationFile {
        pub class_name: String,
        pub version: String,
        pub up_method: Option<Block>,
        pub down_method: Option<Block>,
        pub change_method: Option<Block>,
    }
    
    #[derive(Debug, Clone)]
    pub struct RoutesFile {
        pub routes: Vec<Route>,
        pub namespaces: Vec<Namespace>,
        pub resources: Vec<Resource>,
    }
    
    #[derive(Debug, Clone)]
    pub struct Route {
        pub method: HttpMethod,
        pub path: String,
        pub controller_action: String,
        pub constraints: HashMap<String, String>,
    }
    
    #[derive(Debug, Clone)]
    pub enum HttpMethod {
        Get,
        Post,
        Put,
        Patch,
        Delete,
    }
    
    #[derive(Debug, Clone)]
    pub struct Namespace {
        pub name: String,
        pub routes: Vec<Route>,
        pub resources: Vec<Resource>,
    }
    
    #[derive(Debug, Clone)]
    pub struct Resource {
        pub name: String,
        pub only: Option<Vec<String>>,
        pub except: Option<Vec<String>>,
        pub nested: Vec<Resource>,
    }
    
    #[derive(Debug, Clone)]
    pub struct RubyFile {
        pub classes: Vec<Class>,
        pub modules: Vec<Module>,
        pub methods: Vec<Method>,
        pub statements: Vec<Statement>,
    }
    
    #[derive(Debug, Clone)]
    pub struct Class {
        pub name: String,
        pub parent: Option<String>,
        pub body: Block,
    }
    
    #[derive(Debug, Clone)]
    pub struct Module {
        pub name: String,
        pub body: Block,
    }
}

// ================================================================================
// Functional Combinator Library (for Option 1)
// ================================================================================

pub mod combinators {
    use super::*;
    use std::marker::PhantomData;
    
    /// Result type for parser combinators
    pub type ParseResult<'a, T> = Result<(T, &'a str), ParseError>;
    
    #[derive(Debug, Clone, Error)]
    pub enum ParseError {
        #[error("Unexpected end of input")]
        UnexpectedEof,
        
        #[error("Expected {expected}, found {found}")]
        Unexpected { expected: String, found: String },
        
        #[error("Parse failed: {0}")]
        Failed(String),
    }
    
    /// Core parser combinator trait
    pub trait ParserCombinator<'a, T>: Sized {
        fn parse(&self, input: &'a str) -> ParseResult<'a, T>;
        
        /// Map the output of this parser
        fn map<U, F>(self, f: F) -> Map<Self, F>
        where
            F: Fn(T) -> U,
        {
            Map { parser: self, mapper: f }
        }
        
        /// Sequence two parsers
        fn then<U, P>(self, other: P) -> Then<Self, P>
        where
            P: ParserCombinator<'a, U>,
        {
            Then { first: self, second: other }
        }
        
        /// Try this parser, or fallback to another
        fn or<P>(self, other: P) -> Or<Self, P>
        where
            P: ParserCombinator<'a, T>,
        {
            Or { first: self, second: other }
        }
        
        /// Make this parser optional
        fn optional(self) -> Optional<Self> {
            Optional { parser: self }
        }
        
        /// Parse zero or more times
        fn many(self) -> Many<Self> {
            Many { parser: self }
        }
        
        /// Parse one or more times
        fn many1(self) -> Many1<Self> {
            Many1 { parser: self }
        }
    }
    
    /// Map combinator
    pub struct Map<P, F> {
        parser: P,
        mapper: F,
    }
    
    impl<'a, P, F, T, U> ParserCombinator<'a, U> for Map<P, F>
    where
        P: ParserCombinator<'a, T>,
        F: Fn(T) -> U,
    {
        fn parse(&self, input: &'a str) -> ParseResult<'a, U> {
            let (result, rest) = self.parser.parse(input)?;
            Ok((self.mapper)(result), rest))
        }
    }
    
    /// Sequence combinator
    pub struct Then<P1, P2> {
        first: P1,
        second: P2,
    }
    
    impl<'a, P1, P2, T1, T2> ParserCombinator<'a, (T1, T2)> for Then<P1, P2>
    where
        P1: ParserCombinator<'a, T1>,
        P2: ParserCombinator<'a, T2>,
    {
        fn parse(&self, input: &'a str) -> ParseResult<'a, (T1, T2)> {
            let (result1, rest1) = self.first.parse(input)?;
            let (result2, rest2) = self.second.parse(rest1)?;
            Ok(((result1, result2), rest2))
        }
    }
    
    /// Alternative combinator
    pub struct Or<P1, P2> {
        first: P1,
        second: P2,
    }
    
    impl<'a, P1, P2, T> ParserCombinator<'a, T> for Or<P1, P2>
    where
        P1: ParserCombinator<'a, T>,
        P2: ParserCombinator<'a, T>,
    {
        fn parse(&self, input: &'a str) -> ParseResult<'a, T> {
            self.first.parse(input).or_else(|_| self.second.parse(input))
        }
    }
    
    /// Optional combinator
    pub struct Optional<P> {
        parser: P,
    }
    
    impl<'a, P, T> ParserCombinator<'a, Option<T>> for Optional<P>
    where
        P: ParserCombinator<'a, T>,
    {
        fn parse(&self, input: &'a str) -> ParseResult<'a, Option<T>> {
            match self.parser.parse(input) {
                Ok((result, rest)) => Ok((Some(result), rest)),
                Err(_) => Ok((None, input)),
            }
        }
    }
    
    /// Zero or more combinator
    pub struct Many<P> {
        parser: P,
    }
    
    impl<'a, P, T> ParserCombinator<'a, Vec<T>> for Many<P>
    where
        P: ParserCombinator<'a, T>,
    {
        fn parse(&self, input: &'a str) -> ParseResult<'a, Vec<T>> {
            let mut results = Vec::new();
            let mut rest = input;
            
            while let Ok((result, new_rest)) = self.parser.parse(rest) {
                results.push(result);
                rest = new_rest;
            }
            
            Ok((results, rest))
        }
    }
    
    /// One or more combinator
    pub struct Many1<P> {
        parser: P,
    }
    
    impl<'a, P, T> ParserCombinator<'a, Vec<T>> for Many1<P>
    where
        P: ParserCombinator<'a, T>,
    {
        fn parse(&self, input: &'a str) -> ParseResult<'a, Vec<T>> {
            let (first, mut rest) = self.parser.parse(input)?;
            let mut results = vec![first];
            
            while let Ok((result, new_rest)) = self.parser.parse(rest) {
                results.push(result);
                rest = new_rest;
            }
            
            Ok((results, rest))
        }
    }
    
    /// Basic parsers
    pub fn tag<'a>(expected: &'a str) -> impl ParserCombinator<'a, &'a str> {
        Tag { expected }
    }
    
    pub struct Tag<'a> {
        expected: &'a str,
    }
    
    impl<'a> ParserCombinator<'a, &'a str> for Tag<'a> {
        fn parse(&self, input: &'a str) -> ParseResult<'a, &'a str> {
            if input.starts_with(self.expected) {
                Ok((self.expected, &input[self.expected.len()..]))
            } else {
                Err(ParseError::Unexpected {
                    expected: self.expected.to_string(),
                    found: input.chars().next().map(|c| c.to_string()).unwrap_or_default(),
                })
            }
        }
    }
    
    /// Parse an identifier
    pub fn identifier<'a>() -> impl ParserCombinator<'a, &'a str> {
        Identifier
    }
    
    pub struct Identifier;
    
    impl<'a> ParserCombinator<'a, &'a str> for Identifier {
        fn parse(&self, input: &'a str) -> ParseResult<'a, &'a str> {
            let end = input
                .char_indices()
                .find(|(i, c)| {
                    if *i == 0 {
                        !c.is_alphabetic() && *c != '_'
                    } else {
                        !c.is_alphanumeric() && *c != '_'
                    }
                })
                .map(|(i, _)| i)
                .unwrap_or(input.len());
            
            if end == 0 {
                Err(ParseError::Failed("Expected identifier".to_string()))
            } else {
                Ok((&input[..end], &input[end..]))
            }
        }
    }
}

// ================================================================================
// Testing Infrastructure
// ================================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{Arbitrary, Gen, QuickCheck};
    use criterion::{black_box, criterion_group, criterion_main, Criterion};
    
    /// Property-based testing for parsers
    #[test]
    fn test_parser_roundtrip() {
        fn prop_roundtrip(ast: TestAst) -> bool {
            let printed = ast.to_string();
            let parsed = TestParser.parse(printed.clone());
            match parsed {
                Ok(parsed_ast) => parsed_ast == ast,
                Err(_) => false,
            }
        }
        
        QuickCheck::new()
            .tests(1000)
            .quickcheck(prop_roundtrip as fn(TestAst) -> bool);
    }
    
    /// Performance benchmarks
    fn benchmark_parsing(c: &mut Criterion) {
        let input = include_str!("../testdata/large_file.c");
        
        c.bench_function("parse_c_10k_loc", |b| {
            b.iter(|| {
                let parser = c::CParser::new();
                black_box(parser.parse(input.to_string()));
            });
        });
    }
    
    /// Stub types for testing
    #[derive(Debug, Clone, PartialEq)]
    struct TestAst {
        nodes: Vec<TestNode>,
    }
    
    #[derive(Debug, Clone, PartialEq)]
    struct TestNode {
        kind: String,
        value: String,
    }
    
    impl Arbitrary for TestAst {
        fn arbitrary(g: &mut Gen) -> Self {
            TestAst {
                nodes: Vec::arbitrary(g),
            }
        }
    }
    
    impl std::fmt::Display for TestAst {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            // Simple serialization for testing
            write!(f, "{:?}", self)
        }
    }
    
    struct TestParser;
    
    impl Parser for TestParser {
        type Input = String;
        type Output = TestAst;
        type Error = std::io::Error;
        type Config = ();
        
        fn parse(&self, input: Self::Input) -> Result<Self::Output, Self::Error> {
            // Stub implementation
            Ok(TestAst { nodes: vec![] })
        }
        
        fn parse_with_config(&self, _input: Self::Input, _config: Self::Config) 
            -> Result<Self::Output, Self::Error> {
            Ok(TestAst { nodes: vec![] })
        }
    }
}
