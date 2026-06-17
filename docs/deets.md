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

## Choices
1.`Primary Type` is exposed as `Type`.
2. `Comment` is exposed as `Disambiguation`.

# Week 4

## TODOS
1. clean asin url to only return asin code
