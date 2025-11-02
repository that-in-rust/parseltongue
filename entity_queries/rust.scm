; Rust entity extraction queries
; Based on tree-sitter-rust grammar

; Functions
(function_item
  name: (identifier) @name) @definition.function

; Structs
(struct_item
  name: (type_identifier) @name) @definition.struct

; Enums
(enum_item
  name: (type_identifier) @name) @definition.enum

; Traits
(trait_item
  name: (type_identifier) @name) @definition.trait

; Impl blocks
(impl_item
  type: (type_identifier) @name) @definition.impl

; Modules
(mod_item
  name: (identifier) @name) @definition.module
