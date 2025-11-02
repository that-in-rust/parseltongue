; Swift entity extraction queries
; Based on tree-sitter-swift grammar

; Functions
(function_declaration
  name: (simple_identifier) @name) @definition.function

; Classes
(class_declaration
  name: (type_identifier) @name) @definition.class

; Structs
(struct_declaration
  name: (type_identifier) @name) @definition.struct

; Protocols
(protocol_declaration
  name: (type_identifier) @name) @definition.interface

; Enums
(enum_declaration
  name: (type_identifier) @name) @definition.enum
