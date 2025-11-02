; C entity extraction queries
; Based on tree-sitter-c grammar

; Functions
(function_definition
  declarator: (function_declarator
    declarator: (identifier) @name)) @definition.function

; Structs
(struct_specifier
  name: (type_identifier) @name) @definition.struct

; Enums
(enum_specifier
  name: (type_identifier) @name) @definition.enum

; Typedefs
(type_definition
  declarator: (type_identifier) @name) @definition.typedef
