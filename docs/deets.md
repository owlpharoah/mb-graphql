# Week 1

1. PgPool Config (db/mod): 
    - max: 20 
    - min: 2
    - timeout if not acquired: 5 sec
    - timeout if idle: 600 sec

2. Schema Config (graphql/mod):
    - max depth: 5
    - max complexity: 200

3. Schema Generation:
    - Generation Code: bin/export_sdl.rs
    - Output: schema/schema.graphql
