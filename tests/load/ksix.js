import http from "k6/http";
import { check, sleep } from "k6";
import { Trend, Rate } from "k6/metrics";

const BASE_URL = "http://localhost:8080/graphql";

const ARCTIC_MONKEYS_ARTIST_MBID = "ada7a83c-e3e1-40f1-93f9-3e73dbc9298a";
const YE_ARTIST_MBID = "164f0d73-1234-4e2c-8743-d77bf2191051";
const COLLEGE_DROPOUT_RELEASE_GROUP_MBID =
  "8a01217e-6947-3927-a39b-6691104694f1";
const GOOD_MUSIC_LABEL_MBID = "e36fde8b-924d-4ed0-9c8b-1fcb70ce42ec";

const BATCH_MBIDS = JSON.parse(
  `["${ARCTIC_MONKEYS_ARTIST_MBID}","${YE_ARTIST_MBID}"]`,
);

const gqlErrors = new Rate("graphql_errors");
const simpleLookupTime = new Trend("simple_lookup_duration", true);
const batchLookupTime = new Trend("batch_lookup_duration", true);
const paginationTime = new Trend("pagination_duration", true);
const labelCatalogTime = new Trend("label_catalog_duration", true);
const artistDiscographyTime = new Trend("artist_discography_duration", true);
const multiArtistCompareTime = new Trend("multi_artist_compare_duration", true);

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

//---

const Q_SIMPLE_LOOKUP = `
  query SimpleLookup($mbid: [String!]!) {
    artist(mbid: $mbid) {
      name
      sortName
      disambiguation
    }
  }`;

const Q_BATCH_LOOKUP = `
  query BatchLookup($mbids: [String!]!) {
    artist(mbid: $mbids) {
      mbid
      name
    }
  }`;
const Q_ARTIST_DISCOGRAPHY = `
  query ArtistDiscography($mbid: [String!]!) {
    artist(mbid: $mbid) {
      name
      releaseGroups(first: 25) {
        name
        releases(first: 5) { name }
      }
    }
  }`;

const Q_MULTI_ARTIST_COMPARE = `
  query MultiArtistCompare($mbid1: [String!]!, $mbid2: [String!]!) {
    a0: artist(mbid: $mbid1) { ...F }
    a1: artist(mbid: $mbid2) { ...F }
  }
  fragment F on Artist {
    name
    releaseGroups(first: 10) {
      name
      releases(first: 3) { name }
    }
  }`;

const Q_PAGINATION_EDGE = `
  query PaginationEdge($mbid: [String!]!, $first: Int!) {
    releaseGroup(mbid: $mbid) {
      name
      releases(first: $first) {
        name
        cursor
      }
    }
  }`;

const Q_LABEL_CATALOG = `
  query LabelCatalog($mbid: [String!]!, $first: Int!) {
    label(mbid: $mbid) {
      name
      release(first: $first) {
        name
        cursor
      }
    }
  }`;

//----

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
    { mbids: BATCH_MBIDS },
    { name: "batch_lookup" },
  );
  batchLookupTime.add(res.timings.duration);
  sleep(0.2);
}

export function paginationEdge() {
  const first = [25, 50, 100][Math.floor(Math.random() * 3)];
  const res = gql(
    Q_PAGINATION_EDGE,
    { mbid: [COLLEGE_DROPOUT_RELEASE_GROUP_MBID], first },
    { name: "pagination_edge" },
  );
  paginationTime.add(res.timings.duration);
  sleep(0.2);
}

export function labelCatalog() {
  const res = gql(
    Q_LABEL_CATALOG,
    { mbid: [GOOD_MUSIC_LABEL_MBID], first: 100 },
    { name: "label_catalog" },
  );
  labelCatalogTime.add(res.timings.duration);
  sleep(0.2);
}

export function artistDiscography() {
  const res = gql(
    Q_ARTIST_DISCOGRAPHY,
    { mbid: [YE_ARTIST_MBID] },
    { name: "artist_discography" },
  );
  artistDiscographyTime.add(res.timings.duration);
  sleep(0.3);
}

export function multiArtistCompare() {
  const res = gql(
    Q_MULTI_ARTIST_COMPARE,
    { mbid1: [YE_ARTIST_MBID], mbid2: [ARCTIC_MONKEYS_ARTIST_MBID] },
    { name: "multi_artist_compare" },
  );
  multiArtistCompareTime.add(res.timings.duration);
  sleep(0.3);
}

export const options = {
  scenarios: {
    simple_lookup: {
      executor: "ramping-vus",
      exec: "simpleLookup",
      startVUs: 0,
      stages: [
        { duration: "30s", target: 10 },
        { duration: "30s", target: 50 },
        { duration: "30s", target: 100 },
        { duration: "15s", target: 0 },
      ],
    },
    batch_lookup: {
      executor: "ramping-vus",
      exec: "batchLookup",
      startVUs: 0,
      startTime: "1m45s",
      stages: [
        { duration: "30s", target: 10 },
        { duration: "30s", target: 50 },
        { duration: "15s", target: 0 },
      ],
    },
    artist_discography: {
      executor: "ramping-vus",
      exec: "artistDiscography",
      startVUs: 0,
      startTime: "3m",
      stages: [
        { duration: "30s", target: 5 },
        { duration: "30s", target: 20 },
        { duration: "15s", target: 0 },
      ],
    },
    pagination_edge: {
      executor: "ramping-vus",
      exec: "paginationEdge",
      startVUs: 0,
      startTime: "4m30s",
      stages: [
        { duration: "30s", target: 10 },
        { duration: "30s", target: 30 },
        { duration: "15s", target: 0 },
      ],
    },
    multi_artist_compare: {
      executor: "ramping-vus",
      exec: "multiArtistCompare",
      startVUs: 0,
      startTime: "5m45s",
      stages: [
        { duration: "30s", target: 5 },
        { duration: "30s", target: 15 },
        { duration: "15s", target: 0 },
      ],
    },
    label_catalog: {
      executor: "ramping-vus",
      exec: "labelCatalog",
      startVUs: 0,
      startTime: "7m",
      stages: [
        { duration: "30s", target: 5 },
        { duration: "30s", target: 20 },
        { duration: "15s", target: 0 },
      ],
    },
  },
  thresholds: {
    "http_req_duration{name:simple_lookup}": ["p(95)<50"],
    "http_req_duration{name:batch_lookup}": ["p(95)<200"],
    "http_req_duration{name:artist_discography}": ["p(95)<200"],
    "http_req_duration{name:pagination_edge}": ["p(95)<300"],
    "http_req_duration{name:multi_artist_compare}": ["p(95)<300"],
    "http_req_duration{name:label_catalog}": ["p(95)<400"],
    graphql_errors: ["rate<0.01"],
    http_req_failed: ["rate<0.01"],
  },
};
