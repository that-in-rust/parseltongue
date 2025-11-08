; C++ entity extraction queries - IMPROVED VERSION
; Based on tree-sitter-cpp grammar
; Target: 99-100% entity coverage for real-world C++ codebases
; Date: 2025-11-08

; ============================================================================
; BASIC ENTITIES (from original implementation)
; ============================================================================

; Free functions
(function_definition
  declarator: (function_declarator
    declarator: (identifier) @name)) @definition.function

; Classes
(class_specifier
  name: (type_identifier) @name) @definition.class

; Structs
(struct_specifier
  name: (type_identifier) @name) @definition.struct

; Enums
(enum_specifier
  name: (type_identifier) @name) @definition.enum

; Namespaces
(namespace_definition
  name: (identifier) @name) @definition.namespace


; ============================================================================
; TEMPLATES (Critical for modern C++)
; ============================================================================

; Template class declaration
(template_declaration
  (class_specifier
    name: (type_identifier) @name)) @definition.template_class

; Template struct declaration
(template_declaration
  (struct_specifier
    name: (type_identifier) @name)) @definition.template_struct

; Template function
(template_declaration
  (function_definition
    declarator: (function_declarator
      declarator: (identifier) @name))) @definition.template_function

; Field template (template inside class)
(template_declaration
  (field_declaration
    declarator: (function_declarator
      declarator: (field_identifier) @name))) @definition.template_method


; ============================================================================
; MEMBER FUNCTIONS / METHODS
; ============================================================================

; Method declaration in class (with identifier)
(field_declaration
  declarator: (function_declarator
    declarator: (field_identifier) @name)) @definition.method

; Method declaration with qualified identifier
(field_declaration
  declarator: (function_declarator
    declarator: (qualified_identifier
      name: (identifier) @name))) @definition.method

; Out-of-class method definition
(function_definition
  declarator: (function_declarator
    declarator: (qualified_identifier
      name: (identifier) @name))) @definition.method


; ============================================================================
; CONSTRUCTORS & DESTRUCTORS
; ============================================================================

; Constructor/Destructor (special tree-sitter node)
(function_definition
  declarator: (function_declarator
    declarator: (destructor_name) @name)) @definition.destructor

; Constructor (identified by matching class name)
(function_definition
  declarator: (function_declarator
    declarator: (qualified_identifier
      name: (identifier) @name))) @definition.constructor


; ============================================================================
; OPERATORS
; ============================================================================

; Operator overload
(function_definition
  declarator: (function_declarator
    declarator: (operator_name) @name)) @definition.operator

; Operator in class
(field_declaration
  declarator: (function_declarator
    declarator: (operator_name) @name)) @definition.operator

; Conversion operator
(function_definition
  declarator: (operator_cast_declarator) @name) @definition.operator_cast


; ============================================================================
; TYPE ALIASES
; ============================================================================

; using alias (modern C++)
(alias_declaration
  name: (type_identifier) @name) @definition.type_alias

; typedef (traditional C)
(type_definition
  declarator: (type_identifier) @name) @definition.typedef

; Template type alias
(template_declaration
  (alias_declaration
    name: (type_identifier) @name)) @definition.template_type_alias


; ============================================================================
; ENUMERATIONS (Enhanced)
; ============================================================================

; Enum class (C++11)
(enum_specifier
  name: (type_identifier) @name
  (#match? @body "enum class")) @definition.enum_class

; Scoped enum
(enum_specifier
  name: (type_identifier) @name
  (#match? @body "enum struct")) @definition.enum_struct

; Enum constant
(enumerator
  name: (identifier) @name) @definition.enum_constant


; ============================================================================
; VARIABLES & FIELDS
; ============================================================================

; Static member variable
(field_declaration
  declarator: (field_identifier) @name
  (#match? @storage "static")) @definition.static_member_variable

; Member field
(field_declaration
  declarator: (field_identifier) @name) @definition.field

; Global variable
(declaration
  declarator: (identifier) @name) @definition.variable


; ============================================================================
; MODERN C++ FEATURES
; ============================================================================

; Lambda expression (capture function objects)
(lambda_expression) @definition.lambda

; Constexpr function
(function_definition
  (#match? @qualifier "constexpr")
  declarator: (function_declarator
    declarator: (identifier) @name)) @definition.constexpr_function

; Constexpr variable
(declaration
  (#match? @qualifier "constexpr")
  declarator: (identifier) @name) @definition.constexpr_variable

; Concept definition (C++20)
(concept_definition
  name: (identifier) @name) @definition.concept


; ============================================================================
; NAMESPACES (Enhanced)
; ============================================================================

; Namespace alias
(namespace_alias_definition
  name: (identifier) @name) @definition.namespace_alias

; Inline namespace
(namespace_definition
  name: (identifier) @name
  (#match? @inline "inline")) @definition.inline_namespace


; ============================================================================
; FRIEND DECLARATIONS
; ============================================================================

; Friend class
(friend_declaration
  (declaration
    type: (type_identifier) @name)) @definition.friend_class

; Friend function
(friend_declaration
  (function_definition
    declarator: (function_declarator
      declarator: (identifier) @name))) @definition.friend_function


; ============================================================================
; SPECIAL DECLARATIONS
; ============================================================================

; Static assertion
(static_assert_declaration) @definition.static_assert

; Using declaration
(using_declaration
  (identifier) @name) @definition.using_declaration

; Extern "C" block
(linkage_specification
  (#match? @linkage "\"C\"")) @definition.extern_c


; ============================================================================
; VIRTUAL & OVERRIDE
; ============================================================================

; Virtual function
(field_declaration
  (#match? @virtual "virtual")
  declarator: (function_declarator
    declarator: (field_identifier) @name)) @definition.virtual_function

; Pure virtual function (= 0)
(field_declaration
  (#match? @virtual "virtual")
  declarator: (function_declarator
    declarator: (field_identifier) @name)
  (#match? @pure "= 0")) @definition.pure_virtual_function

; Override function
(field_declaration
  declarator: (function_declarator
    declarator: (field_identifier) @name)
  (#match? @override "override")) @definition.override_function

; Final function
(field_declaration
  declarator: (function_declarator
    declarator: (field_identifier) @name)
  (#match? @final "final")) @definition.final_function


; ============================================================================
; UNIONS
; ============================================================================

; Union
(union_specifier
  name: (type_identifier) @name) @definition.union

; Anonymous union
(union_specifier
  body: (field_declaration_list)) @definition.anonymous_union
