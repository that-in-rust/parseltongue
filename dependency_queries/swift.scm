; Swift Dependency Queries (v0.9.0)

; Function calls
(call_expression
  (simple_identifier) @reference.call) @dependency.call

; Import statements
(import_declaration
  (identifier) @reference.import) @dependency.import

; Protocol conformance
(class_declaration
  name: (type_identifier) @definition.class
  (type_inheritance_clause
    (type_identifier) @reference.conforms)) @dependency.conforms

; Class inheritance
(class_declaration
  name: (type_identifier) @definition.class
  (type_inheritance_clause
    (type_identifier) @reference.inherits)) @dependency.inherits
