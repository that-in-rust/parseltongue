
# Esssence

Current token lenght of models = 200k tokens
Current size of code = 5000k tokens
You canNOT reason properly with such context length

imagine if your ceo asked for insights of last 1 week of data which at granular level exists at 1 minute intervals = 1 x 60 x 24 x  7 = 10080 data points
- do you show him each 10080 data points for insights to help him take decisions
- you can show him it to be aggregated percentile distribution or mean or sum of
    - 168 data points 


## Definitions
- Define interfaces : struct i.e. class, type; functions or generics ; statements; libraries which are basically functions; modules which are basically functions; files behave as modules;
- Code is written in filepath-filename-line-numbers
- Code in action by compiler is actually a graph of interfaces connected to each other via multiple relationships

## Why not graph databases to store code?
- Since the fundamental operations of a compiler are multi-relationship manipulators, a code should exist in a graph database
- An incremental way to think of it is, for the sake of humans, keep the files in tree lije filesystems, but to run a compiler immediately ingest them into a a graphdatabase for faster higher quality compilation
    - Level 1 - write code in filesystem - CURRENT SYTEM
    - Level 2 - ingest code in graphdatabase - run a faster x higher quality compiler to check what the agent did or you did almost like a REPL
    - Level 3 - before final binary run a traditional proven existing compiler

## Why lines are the addresses of functions in a file?


# User Journey




``` mermaid


```