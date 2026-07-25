import http from "k6/http";
import { check, sleep } from "k6";
import { Trend, Rate } from "k6/metrics";

const BASE_URL = "http://localhost:8080/graphql";

// MBIDs from tests/queries/mbids.rs
const ARCTIC_MONKEYS_ARTIST_MBID = "ada7a83c-e3e1-40f1-93f9-3e73dbc9298a";
const YE_ARTIST_MBID = "164f0d73-1234-4e2c-8743-d77bf2191051";
const EMINEM_ARTIST_MBID = "b95ce3ff-3d05-4e87-9e01-c97b66af13d4";
const FAVOURITE_WORST_NIGHTMARE_RELEASE_MBID =
  "f68c985d-f18b-4f4a-b7f0-87837cf3fbf9";
const FIVE_OH_FIVE_RECORDING_MBID = "8dee0224-bcf9-4023-a805-9562bafd3450";
const GOOD_MUSIC_LABEL_MBID = "e36fde8b-924d-4ed0-9c8b-1fcb70ce42ec";

const BATCH_MBIDS = [
  ARCTIC_MONKEYS_ARTIST_MBID,
  YE_ARTIST_MBID,
  EMINEM_ARTIST_MBID,
];

const gqlErrors = new Rate("graphql_errors");
const simpleLookupTime = new Trend("simple_lookup_duration", true);
const batchLookupTime = new Trend("batch_lookup_duration", true);
const artistFullDiscoTime = new Trend("artist_full_disco_duration", true);
const artistFullProfileTime = new Trend("artist_full_profile_duration", true);
const multiEntityCrossJoinTime = new Trend(
  "multi_entity_cross_join_duration",
  true,
);
const releaseDeepDiveTime = new Trend("release_deep_dive_duration", true);
const labelCatalogFullTime = new Trend("label_catalog_full_duration", true);
const recordingReverseLookupTime = new Trend(
  "recording_reverse_lookup_duration",
  true,
);

function safeJson(body) {
  try {
    return JSON.parse(body);
  } catch {
    return null;
  }
}

function gql(query, variables, tags) {
  const res = http.post(BASE_URL, JSON.stringify({ query, variables }), {
    headers: { "Content-Type": "application/json" },
    tags,
  });
  const parsed = res.status === 200 ? safeJson(res.body) : null;
  const hasErrors = !parsed || !!parsed.errors;
  if (hasErrors) {
    console.log(tags.name);
    console.log("status:", res.status);
    console.log(res.body);
  }
  gqlErrors.add(hasErrors ? 1 : 0);
  check(res, {
    "status 200": (r) => r.status === 200,
    "no graphql errors": () => !hasErrors,
  });
  return res;
}

const Q_SIMPLE_LOOKUP = `
  query ArtistBasic($mbid: [String!]!) {
      artist(mbid: $mbid) {
          mbid
          name
          gender
          ended
          beginDate { year month day }
      }
  }
`;

const Q_BATCH_LOOKUP = `
  query ArtistBatch($mbid: [String!]!) {
      artist(mbid: $mbid) {
          mbid
          name
          gender
      }
  }
`;

const Q_ARTIST_FULL_DISCOGRAPHY = `
  query ArtistFullDiscography($mbid: [String!]!) {
      artist(mbid: $mbid) {
          name
          sortName
          releaseGroups(first: 3) {
              name
              type
              firstReleaseDate { year month day }
              releases(first: 3) {
                  name
                  status
                  date { year }
                  medium(first: 2) {
                      name
                      trackCount
                      tracks(first: 2) {
                          name
                          position
                          length
                      }
                  }
              }
          }
      }
  }
`;

const Q_ARTIST_FULL_PROFILE = `
  query ArtistFullProfile($mbid: [String!]!) {
      artist(mbid: $mbid) {
          mbid
          name
          sortName
          disambiguation
          type
          gender
          ended
          beginDate { year month day }
          endDate { year month day }
          tags { name count }
          genres(first: 25) { mbid name disambiguation }
          rating { value votesCount }
          annotation
          area { mbid name }
          beginArea { mbid name }
          endArea { mbid name }
          alias { name sortName type locale primary beginDate { year } endDate { year } ended }
          ipis
          isnis
          releaseGroups(first: 5) { name type }
          releases(first: 5) { name status }
      }
  }
`;

const Q_MULTI_ENTITY_CROSS_JOIN = `
  query MultiEntityCrossJoin($mbid1: [String!]!, $mbid2: [String!]!) {
      a0: artist(mbid: $mbid1) { ...ArtistFull }
      a1: artist(mbid: $mbid2) { ...ArtistFull }
  }
  fragment ArtistFull on Artist {
      mbid
      name
      sortName
      tags { name count }
      genres(first: 10) { name }
      rating { value votesCount }
      releaseGroups(first: 2) {
          name
          type
          releases(first: 2) {
              name
              date { year }
              artistCredit { name joinPhrase }
          }
      }
  }
`;

const Q_RELEASE_DEEP_DIVE = `
  query ReleaseDeepDive($mbid: [String!]!) {
      release(mbid: $mbid) {
          mbid
          name
          status
          date { year month day }
          country
          releaseGroup { name type firstReleaseDate { year } }
          labelInfo { catalogNumber label { name area { name } } }
          releaseEvents { date { year month } country }
          artistCredit { name joinPhrase artist { name } }
          medium(first: 5) {
              name
              position
              trackCount
              tracks(first: 2) {
                  name
                  position
                  length
                  recording {
                      name
                      length
                      video
                      artistCredit { name artist { name } }
                  }
              }
          }
      }
  }
`;

const Q_LABEL_CATALOG_FULL = `
  query LabelCatalogFull($mbid: [String!]!) {
      label(mbid: $mbid) {
          mbid
          name
          type
          area { name isoCode1 }
          rating { value votesCount }
          genres(first: 10) { name }
          alias { name sortName }
          release(first: 3) {
              name
              status
              date { year }
              artistCredit { name artist { name } }
              medium(first: 2) {
                  trackCount
                  tracks(first: 2) {
                      name
                      recording { name length }
                  }
              }
          }
      }
  }
`;

const Q_RECORDING_REVERSE_LOOKUP = `
  query RecordingReverseLookup($mbid: [String!]!) {
      recording(mbid: $mbid) {
          mbid
          name
          length
          video
          firstReleaseDate { year month day }
          isrc
          rating { value votesCount }
          genres(first: 10) { name }
          annotation
          alias { name }
          artistCredit { name joinPhrase artist { name sortName } }
          release(first:2) {
              name
              status
              date { year }
              releaseGroup {
                  name
                  type
                  artistCredit { name artist { name } }
                  genres(first: 5) { name }
              }
          }
      }
  }
`;

export function simpleLookup() {
  const res = gql(
    Q_SIMPLE_LOOKUP,
    { mbid: [ARCTIC_MONKEYS_ARTIST_MBID] },
    { name: "simple_lookup" },
  );
  simpleLookupTime.add(res.timings.duration);
  sleep(0.2);
}

export function batchLookup() {
  const res = gql(
    Q_BATCH_LOOKUP,
    { mbid: BATCH_MBIDS },
    { name: "batch_lookup" },
  );
  batchLookupTime.add(res.timings.duration);
  sleep(0.2);
}

export function artistFullDisco() {
  const res = gql(
    Q_ARTIST_FULL_DISCOGRAPHY,
    { mbid: [YE_ARTIST_MBID] },
    { name: "artist_full_disco" },
  );
  artistFullDiscoTime.add(res.timings.duration);
  sleep(0.5);
}

export function artistFullProfile() {
  const res = gql(
    Q_ARTIST_FULL_PROFILE,
    { mbid: [ARCTIC_MONKEYS_ARTIST_MBID] },
    { name: "artist_full_profile" },
  );
  artistFullProfileTime.add(res.timings.duration);
  sleep(0.3);
}

export function multiEntityCrossJoin() {
  const res = gql(
    Q_MULTI_ENTITY_CROSS_JOIN,
    { mbid1: [YE_ARTIST_MBID], mbid2: [ARCTIC_MONKEYS_ARTIST_MBID] },
    { name: "multi_entity_cross_join" },
  );
  multiEntityCrossJoinTime.add(res.timings.duration);
  sleep(0.5);
}

export function releaseDeepDive() {
  const res = gql(
    Q_RELEASE_DEEP_DIVE,
    { mbid: [FAVOURITE_WORST_NIGHTMARE_RELEASE_MBID] },
    { name: "release_deep_dive" },
  );
  releaseDeepDiveTime.add(res.timings.duration);
  sleep(0.4);
}

export function labelCatalogFull() {
  const res = gql(
    Q_LABEL_CATALOG_FULL,
    { mbid: [GOOD_MUSIC_LABEL_MBID] },
    { name: "label_catalog_full" },
  );
  labelCatalogFullTime.add(res.timings.duration);
  sleep(0.5);
}

export function recordingReverseLookup() {
  const res = gql(
    Q_RECORDING_REVERSE_LOOKUP,
    { mbid: [FIVE_OH_FIVE_RECORDING_MBID] },
    { name: "recording_reverse_lookup" },
  );
  recordingReverseLookupTime.add(res.timings.duration);
  sleep(0.3);
}

export function weightedMixed() {
  const r = Math.random();

  if (r < 0.4) simpleLookup();
  else if (r < 0.6) batchLookup();
  else if (r < 0.75) artistFullProfile();
  else if (r < 0.85) recordingReverseLookup();
  else if (r < 0.92) artistFullDisco();
  else if (r < 0.96) releaseDeepDive();
  else if (r < 0.99) labelCatalogFull();
  else multiEntityCrossJoin();
}

export const options = {
  scenarios: {
    mixed: {
      executor: "ramping-vus",
      exec: "weightedMixed",
      startVUs: 0,
      stages: [
        { duration: "30s", target: 25 },
        { duration: "30s", target: 75 },
        { duration: "30s", target: 150 },
        { duration: "20s", target: 0 },
      ],
    },
  },
};

// export const options = {
//   scenarios: {
//     simple_lookup: {
//       executor: "ramping-vus",
//       exec: "simpleLookup",
//       startVUs: 0,
//       stages: [
//         { duration: "30s", target: 20 },
//         { duration: "30s", target: 50 },
//         { duration: "15s", target: 0 },
//       ],
//     },
//     batch_lookup: {
//       executor: "ramping-vus",
//       exec: "batchLookup",
//       startVUs: 0,
//       startTime: "1m20s",
//       stages: [
//         { duration: "30s", target: 15 },
//         { duration: "30s", target: 40 },
//         { duration: "15s", target: 0 },
//       ],
//     },
//     artist_full_disco: {
//       executor: "ramping-vus",
//       exec: "artistFullDisco",
//       startVUs: 0,
//       startTime: "2m40s",
//       stages: [
//         { duration: "30s", target: 5 },
//         { duration: "30s", target: 15 },
//         { duration: "15s", target: 0 },
//       ],
//     },
//     artist_full_profile: {
//       executor: "ramping-vus",
//       exec: "artistFullProfile",
//       startVUs: 0,
//       startTime: "4m",
//       stages: [
//         { duration: "30s", target: 10 },
//         { duration: "30s", target: 25 },
//         { duration: "15s", target: 0 },
//       ],
//     },
//     multi_entity_cross_join: {
//       executor: "ramping-vus",
//       exec: "multiEntityCrossJoin",
//       startVUs: 0,
//       startTime: "5m20s",
//       stages: [
//         { duration: "30s", target: 5 },
//         { duration: "30s", target: 10 },
//         { duration: "15s", target: 0 },
//       ],
//     },
//     release_deep_dive: {
//       executor: "ramping-vus",
//       exec: "releaseDeepDive",
//       startVUs: 0,
//       startTime: "6m40s",
//       stages: [
//         { duration: "30s", target: 5 },
//         { duration: "30s", target: 15 },
//         { duration: "15s", target: 0 },
//       ],
//     },
//     label_catalog_full: {
//       executor: "ramping-vus",
//       exec: "labelCatalogFull",
//       startVUs: 0,
//       startTime: "8m",
//       stages: [
//         { duration: "30s", target: 5 },
//         { duration: "30s", target: 15 },
//         { duration: "15s", target: 0 },
//       ],
//     },
//     recording_reverse_lookup: {
//       executor: "ramping-vus",
//       exec: "recordingReverseLookup",
//       startVUs: 0,
//       startTime: "9m20s",
//       stages: [
//         { duration: "30s", target: 5 },
//         { duration: "30s", target: 20 },
//         { duration: "15s", target: 0 },
//       ],
//     },
//   },
//   thresholds: {
//     "http_req_duration{name:simple_lookup}": ["p(95)<50"],
//     "http_req_duration{name:batch_lookup}": ["p(95)<150"],
//     "http_req_duration{name:artist_full_disco}": ["p(95)<500"],
//     "http_req_duration{name:artist_full_profile}": ["p(95)<300"],
//     "http_req_duration{name:multi_entity_cross_join}": ["p(95)<800"],
//     "http_req_duration{name:release_deep_dive}": ["p(95)<600"],
//     "http_req_duration{name:label_catalog_full}": ["p(95)<700"],
//     "http_req_duration{name:recording_reverse_lookup}": ["p(95)<400"],
//     graphql_errors: ["rate<0.01"],
//     http_req_failed: ["rate<0.01"],
//   },
// };
