# Afterlives: Code Design

> Status: **draft**. Game rules live in [DESIGN.md](DESIGN.md) (decisions 1–62); this document covers how the code is built.
> Last updated: 2026-11-13

## 1. Overview

A local desktop app. Rust runs the simulation, Bun (TypeScript) makes the LLM calls, Preact draws the UI. They talk over plain HTTP on localhost with protobuf bodies.

```
 Tauri app (window + packaging only)
  │  launches two sidecar processes
  │
  ├─ sidecar: Rust sim server (axum, :<sim_port>) ── owns the run (single source of truth)
  │      ▲                         │
  │      │ fetch / WebSocket       │ reqwest: POST /job  (Job → JobResult)
  │      │ (protobuf)              ▼
  ├─ webview: Preact UI        sidecar: Bun LLM server (Elysia, :<llm_port>)
  │    2D map, panels               OpenAI SDK, Anthropic SDK, fetch
  │
  └─ runs/<id>/  JSONL logs + snapshots (written by the sim server)
```

| Side | Server | Client |
|---|---|---|
| Rust | `axum` (UI API, run control, live updates over WebSocket) | `reqwest` (sends LLM jobs to Bun) |
| Bun | `Elysia` (`POST /job`) | built-in `fetch` (web search for personas, any HTTP besides the SDKs) |

**Principles**

- **Two servers, one owner.** Rust and Bun each run an HTTP server, but only Rust holds game state. Every LLM call is one HTTP request from Rust to Bun (`POST /job`), so each job is a single request/response that is easy to trace.
- **State, rules and dice in Rust; words in Bun.** Rust owns world state, rule checks, energy and every random roll, and it sends jobs as structured data. Bun owns everything about language: it renders AI briefs and GM situations into prompts, calls the model, and parses and validates what comes back. Bun keeps no game state between jobs and can be restarted mid-run.
- **Rust drives time.** The server advances the clock and blocks on outstanding LLM jobs; no client can move the world forward.
- **The sim core is pure.** The `sim` module inside the server binary uses no HTTP, Tauri, network or file I/O, so it can be unit-tested and run headless.
- **Retraceable, not replayable.** Logs at several levels make every decision traceable (DESIGN §11); exact replay is not a goal.
- **Hard systems can start in the GM.** Any rule that is too hard to code can be a text description the GM rules on (DESIGN decision 45) and move into Rust later. The code must make that swap easy.

## 2. Repository layout

```
typhon/
  proto/                    shared schema (buf module)
    typhon/v1/*.proto
  buf.yaml  buf.gen.yaml    buf config (module path: proto)
  typhon-server/            Cargo workspace members
    server/                 binary: simulation + axum server
      src/main.rs
      src/sim/              world, people, rules, AI seats (no I/O)
      src/…                 HTTP routes, run loop, job client, logs
    app/                    Tauri shell (window, sidecar launch)
  typhon/                   Bun LLM server (@typhon/worker, Elysia)
    src/
    ui/                     Preact spectator UI (@typhon/ui)
  runs/                     run output (gitignored)
  Cargo.toml                Cargo workspace root
  package.json              Bun workspace root (typhon, typhon/ui)
  DESIGN.md  ARCHITECTURE.md
```

**Codegen**

- Rust: `typhon-server/server/build.rs` compiles `proto/` with `protox` (pure Rust, no `protoc`) and generates message types with `prost-build`. There are no services; routes are plain axum handlers.
- TypeScript: `bun run gen` runs `buf generate` with `protoc-gen-es` into `typhon/src/gen/` and `typhon/ui/src/gen/`.
- `buf lint` and `buf breaking` guard schema changes.

## 3. Components

### 3.1 `proto/`: the contract

Package `typhon.v1`: **messages only**. The proto files define request and response bodies; the HTTP routes below say where they go. Bodies are binary protobuf (`Content-Type: application/x-protobuf`), encoded with `prost` in Rust and `toBinary`/`fromBinary` from `@bufbuild/protobuf` in TypeScript.

**Routes**

| Server | Route | Body → Response | Caller |
|---|---|---|---|
| Bun (Elysia) | `POST /job` | `Job` → `JobResult` | Rust (`reqwest`) |
| Bun (Elysia) | `GET /health` | — → status | Rust, Tauri |
| Rust (axum) | `POST /run/create`, `/run/start`, `/run/pause`, `/run/resume` | `RunCmd` → `RunStatus` | UI |
| Rust (axum) | `GET /run/status` | — → `RunStatus` | UI |
| Rust (axum) | `POST /view/map`, `/view/region`, `/view/person`, `/view/seat`, `/view/log` | query → view message | UI |
| Rust (axum) | `GET /live` (WebSocket) | binary frames of `LiveUpdate` | UI |

**Job messages** (sketch)

```proto
message Job {
  string job_id = 1;          // unique, also used in every log line
  string run_id = 2;
  string seat = 3;            // empty for GM jobs
  ModelRef model = 4;         // provider + model id + params
  Limits limits = 5;          // deadline, steps left, time left
  GameTime time = 6;          // year, month, day, step index
  oneof payload {             // structured data only, no prompt text
    AiStep ai_step = 10;      // persona, brief, prior steps, tool specs
    GmScheme gm_scheme = 11;  // scheme text, actor's persona card, situation
    GmBattle gm_battle = 12;  // battle sheet, orders, computed odds, roll result
    GmEvent gm_event = 13;    // candidate events with odds, free-event budget
    PersonaCreate persona_create = 14;
    RebirthPick rebirth_pick = 15;  // family tree only
  }
}

message JobResult {
  string job_id = 1;
  RenderedPrompt prompt = 2;  // exactly what was sent, for the logs
  oneof outcome {
    AiStepResult ai_step = 10;        // tool calls + private reasoning text
    GmRuling gm_ruling = 11;          // requirements, odds, narration, notable tag
    PersonaCard persona = 12;
    RebirthChoice rebirth = 13;
    JobError error = 20;              // refusal, timeout, invalid output after retries
  }
  Usage usage = 21;                   // tokens, cost, latency
}
```

**Jobs carry data, results carry typed answers.** Rust never builds prompt text or sees provider formats. Bun never decides game outcomes: a `GmRuling` proposes odds and narration, and Rust rolls and applies. Tool specs (name, parameters as JSON Schema, energy cost) are defined in Rust, which validates every call; Bun only words their descriptions.

**Battles:** Rust computes the odds and rolls first. The GM job then includes the roll result, so the GM explains an upset rather than deciding one (decision 23). **Schemes:** the GM proposes requirements and odds, then Rust rolls and sends a short follow-up job to narrate the outcome (decision 36).

### 3.2 `typhon-server/server/src/sim/`: the simulation core

A module of the server binary. Only the module boundary separates it from the server code, and it must stay free of I/O.

A plain library with one `World` struct that owns all state, and systems as free functions over it (`fn step_economy(world: &mut World, rng: &mut Rng)`). No traits for layering, no `*Manager` types.

| Module | Owns | DESIGN refs |
|---|---|---|
| `grid` | flat triangle grid wrapping left–right: indexing, neighbors, edges, distances, paths | §10.4, decision 19 |
| `worldgen` | terrain, fertility, water, deposits, initial states, settlements, named people | decisions 18, 49, 62 |
| `clock` | months, NPC half-months, day-ordered effect queue (`BinaryHeap` by arrival day, random tie-break) | decisions 20, 28, 30 |
| `people` | tiers 0–3, promotion/demotion, families and descent | §7, decision 31 |
| `npc` | needs → duties → ambitions, traits, morale vs. expectations, opinions, plots | §7.1, decision 29 |
| `economy` | production per triangle, consumption, stockpiles, local prices, taxes, merchants, routes | §10.3, decisions 55–58 |
| `land` | owner state + holder per triangle, borders | decision 56 |
| `politics` | laws, succession, regency | §10.8, decisions 50, 53 |
| `tech` | fixed trees, per-settlement levels, spread and loss | §10.1, decisions 13–16, 52 |
| `war` | armies, battle sheets, odds, sieges, raids, naval battles | §10.7, decisions 23, 61 |
| `events` | rule-generated event candidates with odds | §10.6, decision 37 |
| `news` | news objects carried by travelers, distortion | decision 59 |
| `seats` | AI seats: persona cards, bodies, energy, wake checks, memory log, rebirth | §4, §6, §8.1 |
| `brief` | gathers what an AI's character knows, as structured data (Bun renders it) | decision 27 |
| `tools` | AI tool specs, validation, energy costs, execution | decisions 24, 26 |
| `gm` | assembles GM situations (battle sheets, schemes, event candidates), checks rulings against bounds, rolls, applies | §9, decisions 36, 44 |
| `rng` | seeded RNG (`rand_chacha`), one stream per system | — |

**LLM boundary.** The sim never calls an LLM. When it needs one, it returns a request (`NeedsLlm { seat, payload }`) holding structured data, and the server turns that into a `Job`. Typed results come back through `sim::apply_result(world, job, result)`. This keeps `sim` testable with scripted results.

**Logging boundary.** The sim pushes structured records into an in-memory `Vec<LogRecord>` on `World`. The server drains it after each step and writes the files. The sim does no file I/O.

### 3.3 `typhon-server/server/src/`: run loop and HTTP

- **Run loop** (one tokio task per run):
  1. `sim::step_month(&mut world)`: two NPC half-month steps, economy, events, news, then wake checks.
  2. Collect `NeedsLlm` requests and turn them into `Job`s.
  3. Send each job to Bun as its own `reqwest` POST, all in parallel, each with a timeout equal to the job's deadline (decision 43). When a step's response arrives, call `apply_result`, which may produce the next step's request (same turn) or end the turn.
  4. When every job for this month is done or timed out, apply the day-ordered effects and drain the logs to disk.
  5. Push a `LiveUpdate` to WebSocket subscribers and loop.
- **Job client:** one shared `reqwest::Client` (connection pooling), a concurrency limit, and retries with backoff when Bun is unreachable (e.g. restarting). Pending jobs are tracked by `job_id` with deadline, retry count and seat.
- **HTTP server:** `axum` routes for run control and views, plus a WebSocket for live updates. CORS is limited to the app origin.
- **Storage:** writes `runs/<id>/` (see §5).
- **Standalone binary:** launched by the Tauri app as a sidecar, or run alone (`cargo run -p server`) for headless runs and debugging in a normal browser.

### 3.4 `typhon-server/app`: Tauri shell

- Picks two free localhost ports and a per-launch auth token.
- Launches the Bun LLM server (built with `bun build --compile`) and the Rust sim server as sidecars. Both get the ports and token through environment variables; the sim server gets Bun's URL.
- Opens the webview on the bundled UI, passing the sim server's port and the token in the URL.
- No Tauri commands or events are used; everything else goes over HTTP.

### 3.5 `typhon/`: Bun LLM server and GM

- An `Elysia` server on localhost with `POST /job`: decode the `Job`, process it, and reply with an encoded `JobResult`. Every request must carry the per-launch token.
- Outbound HTTP besides the SDKs (e.g. web search for persona creation) uses Bun's built-in `fetch`.
- **Providers:** `anthropic` (`@anthropic-ai/sdk`) and `openai` (`openai`). An OpenAI-compatible base URL covers other providers (DeepSeek, Kimi, Qwen, …) where their APIs match.
- **Prompt rendering:** templates turn each payload into text: the AI system prompt (persona, fiction framing for decision 42, in-world text wrapping for decision 41), the brief, the steps and time left with warnings (decision 43), and the GM prompts. All prompt wording lives here.
- **Injection check:** in-world text (letters, speeches) is pattern-checked before rendering. Flagged text is reported back to Rust, which applies the rule (decision 41).
- **Output handling:** parse and validate each answer into its typed result, with schema checks for GM rulings, persona cards and rebirth picks. Malformed output is retried once (decision 42). After that, Bun returns `JobError`.
- **Per job:** render the prompt, make one SDK call (or one call plus a format retry), and return the typed result with the rendered prompt and usage.
- **Concurrency:** a per-provider limit; retries with backoff on rate limits and 5xx errors; the job deadline enforced with an `AbortController` (also aborted if Rust drops the request).
- **Refusals:** reported as `JobError{REFUSAL}`. The sim decides whether to re-prompt (decision 42).
- **Web search** is available only to `PERSONA_CREATE` jobs (decision 38).
- API keys come from the environment and are never sent to the sim server.

### 3.6 `typhon/ui/`: Preact spectator UI

- Preact + TypeScript, bundled by Bun into Tauri's frontend assets.
- Talks to the Rust sim server with `fetch` (protobuf bodies, types generated from `proto/`) and one WebSocket (`/live`) for live updates.
- **Views:**
  - **Map:** 2D canvas/WebGL (e.g. PixiJS), triangle grid wrapping horizontally, colored by terrain, owner or holder; roads along edges; settlement and AI markers.
  - **Timeline:** month scrubber; GM-tagged highlights (decision 46).
  - **Seat panel:** current body, energy, private reasoning vs. public words, past lives, family tree.
  - **Log viewer:** filter by seat, kind or `job_id`.
  - **Run control:** create, start, pause; live status and spend.

## 4. One month, end to end

```
server                     sim                     Bun (Elysia)        UI
  │ step_month ───────────▶│ NPC ½-month ×2, economy,
  │                         │ events, news, wake checks
  │◀──── NeedsLlm × n ──────│
  │ POST /job (ai_step s1) ─────────────────────────▶│ render, SDK call, parse
  │ POST /job (gm_battle)  ─────────────────────────▶│ render, SDK call, validate
  │◀──────────────────────────────── 200 JobResult ──│
  │ apply_result ─────────▶│ run tools, charge energy,
  │                         │ queue actions by arrival day
  │◀──── NeedsLlm (s2) ─────│  (next step of the same turn)
  │ ... until every turn ends (done / step cap / deadline)
  │ resolve_month ────────▶│ apply effects in day order
  │ write logs, snapshot?   │
  │ LiveUpdate (WebSocket /live) ───────────────────────────────────▶│
```

## 5. Run output

```
runs/<run_id>/
  config.toml            seats, models, seed, years, limits
  world.jsonl            world events (month, day, kind, ids, summary)
  ai/<seat>.jsonl        AI turns: brief hash, tool calls, results, energy
  llm.jsonl              every job: rendered prompt (sent back by Bun), response, tokens, cost, latency
  gm.jsonl               rulings: odds, roll, narration, notable tag
  snapshots/y010.json.zst
```

Every line carries `run_id`, game time, and `job_id` where relevant, so `rg <job_id> runs/<id>/` follows one call across every file.

**Run config:** a TOML file listing seats (persona slot, provider, model), the GM model, seed, year limit and turn limits. It is copied into the run directory at start.

## 6. Build steps (milestones)

Each step ends with something runnable. The early steps target the riskiest parts first.

| # | Milestone | Done when |
|---|---|---|
| M0 | **Walking skeleton** | Workspaces and codegen set up. Rust POSTs a dummy `Job` to Bun's `/job` and decodes the `JobResult`; the UI fetches `/run/status` from axum and receives a `LiveUpdate` over `/live`; Tauri opens the window and launches both sidecars. |
| M1 | **World and map** | `worldgen` builds a wrapping triangle grid with terrain, water, deposits and states; the UI draws it with pan, zoom and wrap. |
| M2 | **Headless world, no LLMs** | People (tiers, families), the NPC model, economy, trade, news and events run 150 years headless in seconds to minutes. Logs written; a perf check at ~50k people. |
| M3 | **One AI seat** | Brief, tools, energy, multi-step turns, step and time caps, memory log with `recall`. A scripted fake provider first, then one real model. |
| M4 | **Game master** | Schemes (odds, roll, narration), battles, sieges and raids, grounded and free events, notable tags. |
| M5 | **Life cycle** | Death, card reveal, 30-year wait, rebirth pick, body merge, elimination. Meets MVP success criterion 2. |
| M6 | **Society** | Tech trees and progress, laws, succession and regency. |
| M7 | **Full MVP run** | Persona creation (with web search), multiple providers, 6 seats × 150 years. Costs measured (criterion 1). |
| M8 | **Spectator polish** | Timeline scrubber, seat panels, highlight feed, log viewer (criterion 3). |

## 7. Deferred

- Tracing beyond IDs in logs (e.g. OpenTelemetry).
- App packaging, code signing and auto-update.
- Exact step cap and turn time limit (from M3/M7 measurements).
- Snapshot interval and format details.
