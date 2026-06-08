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

# Week 3 

1. The primaryType,secondaryTypes and similar others under releasegroup is in the materialized table `artist_release_group` and should be defined in complex object
2. The dataloader - ReleaseGroupByArtist will take in artist_id not mbid

## Choices/TODO
Q. should comment be exposed as comment or disambiguation
Q. release_group_type == primary type , then ill expose it as primary type
T. convert primary/secondary type into string (rn its a number)
