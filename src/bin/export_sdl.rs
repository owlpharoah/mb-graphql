use mb_graphql::graphql::build_schema_export;

fn main() {
    let schema = build_schema_export();
    std::fs::write("schema/schema.graphql", schema.sdl())
        .expect("failed to write schema/schema.graphql");
    println!("SDL written to schema/schema.graphql");
}
