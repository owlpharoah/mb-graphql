use mb_graphql::graphql::build_schema_export;

#[test]
fn export_sdl() {
    let schema = build_schema_export();
    let sdl = schema.sdl();
    std::fs::write("schema/schema.graphql", sdl).expect("failed to write SDL");
}
