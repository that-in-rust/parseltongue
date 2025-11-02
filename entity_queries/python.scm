; Python entity extraction queries
; Based on tree-sitter-python grammar

; Classes
(class_definition
  name: (identifier) @name) @definition.class

; Functions
(function_definition
  name: (identifier) @name) @definition.function

; Methods (functions inside classes)
(class_definition
  body: (block
    (function_definition
      name: (identifier) @name))) @definition.method
