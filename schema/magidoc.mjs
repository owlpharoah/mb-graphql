export default {
  introspection: {
    type: "sdl",
    paths: ["./schema.graphql"],
  },
  website: {
    template: "carbon-multi-page",
    output: "./docs",
    options: {
      siteRoot: "",
      appTitle: "MusicBrainz GraphQL API Docs",

      queryGenerationFactories: {
        UUID: "550e8400-e29b-41d4-a716-446655440000",
      },
    },
  },
};
