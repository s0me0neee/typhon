# Afterlives (working title, alt. 《天命》)

> Status: **design** — rules settled for the MVP (§14); architecture in progress (§16). No code yet.
> Last updated: 2026-11-13

## 1. Pitch

A civilization simulation on a world map where a handful of AIs (LLMs) are dropped in as **individual people** — not players, not gods — living among a world of program-controlled people. They are born, scheme, rule, die, and come back decades later inside a descendant with no memory of their past life, while the world remembers everything they did.

The AIs are **super-intelligent individuals**: they know everything modern humanity knows and are meant to be degrees more capable than anyone around them. What holds them back is the world — its materials, its people, its time — and each other.

The point is the show: watching how AIs treat the world, how the world pushes back, and how the AIs deal with each other — 赛博逗蛐蛐 at civilization scale.

Tagline candidate: *三十年河东，三十年河西.*

## 2. Inspiration

- **Bilibili series 【文明与地图】** (Vol. 1 "12个大模型扮演君主谁能带领人类走向星际文明？", Vol. 2 "你说什么叫哈吉米王朝？"): 12 LLMs descend into a Bronze Age world run by an LLM game master. Key moments worth reproducing:
  - **Interregnum collapse** — all AI rulers died around the same time; NPC heirs caved to nobles, a plague came back and killed half the world.
  - **Neutral zone of royal heirs** — reborn AIs met in disguise, signed an anonymous treaty, tested trust.
  - **Hijacking a rival's rebirth** — Gemini rebuilt DeepSeek's public character card, predicted which descendant it would pick, smuggled that prince abroad, and greeted the memoryless DeepSeek as a "rescuer".
  - **Respawn as a weapon** — getting a rival's descendants disowned removes them from its rebirth list.
  - Weak models leaking their identity, failing tool use, signing bad treaties.
  - Caveat: the videos are heavily edited and narrated; they show outcomes, not how often things happen.
- **civ-atlas / 文明与地图** (github.com/guaner-334/civ-atlas): procedural planet + region-level history sim. Reference for world generation and history ideas (AGPL-3.0 — ideas only, no code reuse unless the project goes AGPL).
- Related experiments: Stanford *Generative Agents* (2023), Altera *Project Sid* (2024), *AI Diplomacy* (Every, 2025).

## 3. Pillars

1. **AIs are people, not players.** One body, a natural lifespan, only what their character could know about the world, and no powers beyond their mind.
2. **Overpowered minds, physical world.** Unlimited knowledge; every project still costs materials, skilled people and time.
3. **The world pushes back.** Ordinary people have interests, memories and the ability to change history; the AIs cannot simply dictate.
4. **Memory dies, consequences don't.** An AI forgets its past life; its family, enemies, debts and reputation do not.
5. **Freedom over rules.** Soft goals, no forced strategy; interesting behavior should emerge.
6. **Legible for spectators.** Every AI's private reasoning sits next to what it said publicly — the gap is the entertainment.

## 4. The AIs

- **Seats:** MVP ~6 (tentative); up to ~12.
- **Lineup:** personas are the main axis of difference; different models (Claude, GPT, DeepSeek, Gemini, Kimi, …) are supported too. To compare models fairly, rotate models between runs.
- **Self-designed personas:** before the run, each AI designs its own persona and picks an image from the web as its icon. The harness gives every model the same web search tool (as agent tools like opencode do), so this doesn't depend on a provider's built-in web access. If a search fails, the AI picks from a curated image set. A human reviews every icon before the run. The persona and icon are **fixed after creation** and stay with the AI through every life. A card has two parts:
  - **Basic card** (revealed on first death): model, persona name, icon, basic characteristics, description.
  - **Full card** (never revealed): detailed values, flaws, fears, habits, speech style.
- **Personas drive decisions:** an AI's choices should follow its persona — across all lives, since the persona is the only thing that survives death. Example: a persona that loves developing the economy tends to be reborn as a trader, and once it comes to rule, pushes free trade and markets. This consistency is also what makes a revealed card valuable to rivals: they can predict which body it will pick and what it will do.
- **Knowledge:** unlimited real-world knowledge (science, engineering, history, statecraft). In-world information is limited to what the character could know — local events, rumors arriving late with distance, written records.
- **Out-of-world tools:** none. No self-simulation or other meta-tricks; AIs act only through their character. Revealing one's identity in speech is still possible.

### Identity rules

- After world creation, the **names of every AI's first body are published to the AIs only** (the world's people know nothing). Every AI knows which people started as AIs — but not which persona/model is inside which body. AIs hide *who they are*, not *that they are*.
- On an AI's **first death**, its **basic card** (model, persona name, icon, basic characteristics, description) is revealed to all AIs. Since it's revealed when a known AI body dies, it effectively tells everyone who was inside that body.
- After the first life, rebirth bodies are **not** published — later identities must be inferred, e.g. by matching behavior against revealed cards.

## 5. Goals

- No hard win condition. For now a run stops at a set number of in-game years (MVP: 150).
- A **Civ-style scoreboard** tracked in the background: domination (land, followers), culture, religion, wealth/science, diplomacy (allies, vassals). Spectators see all of it. AIs never see the scoreboard; they know only what their character could know (their own realm, plus rumors and reports from elsewhere).
- "Lead humanity to the stars" is a possible long-horizon soft goal — plausible here, since knowledge is unlimited.
- Watch for passive play; add pressure later only if needed.

## 6. Life, death and rebirth

- **Starting world:** Bronze Age, with states, kings, nobles and borders already in place.
- **First body:** each AI picks from a list of named people of mixed ranks (name, age, rank, family, place) — the same way it later picks rebirth bodies.
- **Lifespans:** natural, per body — age, health, disease and violence as for anyone. No plot armor; assassinating a rival AI is a legitimate strategy. Interregnums happen when they happen.
- **Rebirth:** 30 years after death, an AI may return inside a **descendant** — biological or "logical".
- **Memory:** on death the AI's context is wiped. It keeps only its persona and what its new body has lived through.
- **Choosing a new body:** the fresh AI sees **only its own family tree** (names, ages, places, lineage), not the world.
- **Logical descent:** designated heir, adopted child, disciple/apprentice, successor to a title/office it held, member of an organization it founded. Social recognition counts: disowned descendants drop off the list.
- **No candidate:** wait until one appears; if the whole line is extinct, the AI is **eliminated**.
- **Age of the new body:** any age the AI picks; ~10 is the expected sweet spot.
- **No declining:** at year 30 the AI must choose one candidate from its tree right away. Only an empty tree makes it wait (see "No candidate" above).
- **Taking over a body (merge):** the AI becomes that person. It receives their life so far as a summary, their relationships, debts and reputation, and the opinions others hold of them. The world sees the same person, though a sudden change in behavior can raise suspicion.
- **Same body:** a person can hold only one AI. The first claim by in-game date wins (same-day ties random), and the loser picks again from those left. First bodies are picked one at a time in a random draft order, shown to spectators.
- **Planning across lives:** only through in-world means — wills, grooming an heir, written instructions ("祖宗之法不可变").

## 7. The world and ordinary people

Everyone exists, but at different levels of detail; anyone can be **promoted** when they start to matter.

| Tier | Who | Detail |
|---|---|---|
| 0 — Population | the masses | counts and averages per settlement (class, culture, faith, mood, health, skills) |
| 1 — Named people | noble/royal/notable families, AI relatives | family links, traits, role, life events — needed for family trees and rebirth |
| 2 — Full characters | rulers, nobles, generals, merchants, rebels, prophets, anyone an AI deals with | goals, memories, relationships, own decisions |
| 3 — AIs | the LLM seats | as above |

How ordinary people make an impact:
- **Events create people:** famine + unrest → a rebel leader emerges from the population; a new faith → a prophet; a battle → a hero. Their traits decide how history branches.
- **AI contact promotes:** anyone an AI recruits, marries, wrongs or spares becomes a full character with memory and loyalty — grudges and debts can outlive the AI.
- **Climbing on merit:** peasant → soldier → officer → general → usurper; chaos creates more upstarts.
- **Demotion:** a Tier 2 character with no office, army, plot or AI contact for ~5 years drops to Tier 1. Their top opinions, grudges and debts are kept in compact form and come back if they are promoted again. A soft cap (~5k full characters) demotes the least relevant first.
- **Factions with interests:** nobles, clergy, merchants, armies resist or exploit AI reforms — the source of the interregnum collapse in the video.
- Program-controlled characters act on their own traits and interests; an LLM is only optionally used to *narrate* their scenes, not to decide.

### 7.1 How program-controlled people decide

Rule-based, no LLM, deliberately simple. Draws on *Oxygen Not Included* (task priorities, stress) and *WorldBox* (opinions, plots).

- **Step:** NPCs act **twice a month**; the world and the AIs step monthly.
- **Priority stack:** needs (food, safety, health), then the duties of the person's role (farm, govern, command, trade), then ambitions. Traits and other factors (age, wealth, faith, policy) adjust each layer. Actions cost energy, as for the AIs (§8.1).
- **Morale vs. expectations:** each person expects a standard of living set by rank and traits. A shortfall builds stress; meeting or beating expectations relieves it. Past a threshold, the person has a stress response that depends on role: a peasant riots, a soldier deserts, a general defects, a noble plots a coup.
- **Opinions:** each person holds an opinion of the people they know, built from traits, favors, wrongs and kinship. Opinion decides loyalty, alliances and the targets of ambitions, and it is how grudges and debts outlive an AI.
- **Plots:** ambitions run as plots (marriage, alliance, rebellion, assassination). A plot starts only when its preconditions hold, advances at a speed set by the person's stats, and can be discovered by others.

## 8. Interactions

- **AI ↔ world:** everything an AI does is an in-world action (move, speak, persuade, threaten, promise, lie, trade, recruit, command, marry, found, build, teach, rebel, write). Promises are remembered; broken ones cost reputation.
- **Building on knowledge:** an AI can introduce anything it knows, but it must be **built physically** — materials, tools, trained people, time — and adopted by people who may resist.
- **AI ↔ AI:** letters that take travel time, meetings when co-located, public proclamations, treaties as in-world documents.
- **Private channels:** private meetings and letters are hidden from other characters but **always recorded** for spectators and replay.

### 8.1 The AI's turn

- **Monthly check:** every living AI is checked every month. If all of this month's energy is already committed (mid-siege, mid-journey) and nothing new happened, the month runs with no LLM call. Otherwise the AI gets a turn. Events (a letter arrives, an attack, a death in the family, a project finishes, a nearby event above the importance threshold) are always delivered in the next turn.
- **Energy = the character's time.** Each month the body has an energy budget set by its state: health, age, work efficiency, the nation's work policy, and so on. Every action costs energy in proportion to the time it takes. Unused energy is lost at the end of the month; it does not carry over. An action spanning several months takes its share of energy from each month it runs.
- **Steps within a turn:** the first step gives the AI what its character knows. The AI then acts, and looking things up is itself an action that costs energy. The next step returns whatever the character can access. Steps continue until the AI stops or runs out of energy. Facts the character must find out in the world (send a spy, survey a hill, ask a merchant) are in-world actions, and their answers arrive later as events.
- **Action form (hybrid):** fixed tool calls for measurable actions (move, send_letter, command, trade, marry, build, …), plus one free-form `scheme(description)` that goes to the GM (§9).
- **Step cap:** every seat has the same maximum number of steps per turn. The AI is told how many steps it has left and warned as it nears the cap, so it can finish its plans before the turn ends.
- **Time limit:** every turn also has a wall-clock time limit, and the AI is shown the time left at every step. Models run at different speeds and a round waits for its slowest seat, so the limit is kept fairly short. When time runs out, the turn ends with the actions taken so far.
- **No token or dollar caps** for now; spend is watched during the MVP.
- **Memory within a life:** memory is the log of the AI's events and actions. The prompt holds the newest entries that fit; older entries move to a memory file. The AI reads that file with `recall(query)` or `recall(from, to)`, which costs no energy and is capped at about 3 reads per turn. The log is wiped at death (§6).
- **Order of effects:** actions take in-world time to arrive (travel, letters, marching armies), and each gets an exact arrival day from distance and route speed. The world steps in months, but within a month effects apply in day order, so whoever arrives first acts first. Exact same-day ties are broken at random.

## 9. Game master

**Rules + GM for free-form.**
- Hard rules decide anything measurable: troops, money, distance, time, lifespans, construction costs.
- An LLM GM judges only genuinely free-form actions (new institutions, schemes, novel inventions), narrates them, and may trigger disasters.
- The GM reads the acting AI's full persona card: in-character actions are judged more plausible and effective, out-of-character ones less.
- Watch for GM bias when the GM and an AI seat use the same model family.
- **GM model:** one strong model, fixed across runs for consistency. Bias toward its own family's seat is checked from the logged odds (odds by seat model).
- **Fallback for hard systems:** if a system is too hard to code as rules, it can be written as a text description and handed to the GM to decide, under the same odds-then-dice pattern. Each such system adds GM calls, so move it to rules once it settles.
- **Odds before dice, for schemes too:** for a free-form scheme, the GM first states its requirements (materials, people, time, energy per month) and a success chance, raised for in-persona actions and lowered for out-of-persona ones. This is logged before the roll. Dice decide, then the GM narrates. Battles follow the same pattern with rule-computed odds (§10.7).

## 10. World design

Reference games: **Civilization 5/6** and **Stellaris**. Borrow their systems where they fit; details of each are still to be designed.

### 10.1 Technology

- **Fixed tree, GM decisions:** the entries and their order are fixed before the run. The GM decides the specifics case by case (costs, progress, effects, attempts to skip a step) using odds-then-dice (§9). Rules can replace GM decisions later, as entries settle.
- **The tree measures what a society can build, not what anyone knows.** AIs already know all of it; the tree is the materials, tools, trained workers and institutions needed to make it real.
- **Two fixed, visible trees** (Civ 6 style): **technology** (things you build) and **civics** (ways of organizing people). Fixed so AIs can plan decades ahead and viewers can read progress.
- **Research speed comes from the state of the country, not from a slider.** The workforce doing research is whoever the country can spare and support: food surplus frees people from farming, wealth pays them, cities and institutions (schools, workshops, academies) house them, peace and stability keep them working. A ruler chooses the *priority*; the country decides the *speed*. A starving or war-torn state researches almost nothing.
- **Technology lives in settlements and their skilled people.** Each settlement has its own level. It spreads along trade routes and with migrants, can be stolen by poaching craftsmen, and is lost when a city is sacked or its skilled people die.
- **The AI edge:**
  - Work an AI personally leads goes much faster (no trial and error) — but only where the AI physically is.
  - An AI can **teach**. Trained disciples keep part of the speed-up after the AI dies, and count as logical descendants (§6).
  - An AI may try to **skip a step** in the tree at a steep cost and a real chance of failure.
- **Pace:** fast development is expected. One life is not enough for a big leap: with the 30-year gap, progress has to survive through institutions and trained people, or it stalls and decays. Exact speed is to be tuned on test runs.

**Draft tree** (first pass; prerequisites and costs to be designed). Everything in the first row is already known somewhere in the starting world.

| Era | Technology | Civics |
|---|---|---|
| Bronze Age (start) | farming, herding, pottery, the wheel, bronze working, masonry, irrigation, sailing, early writing | kingship, priesthood, customary law, craft specialization, tribute |
| Iron Age | iron working, horse riding, alphabet, mathematics, roads and arches, galleys, crop rotation | coinage, written law, standing army, scribal bureaucracy, schools |
| Classical | steel, concrete and aqueducts, water mills, glass, paper, sanitation and medicine, heavy plough | civil service, academies, organized religion, citizenship, trade guilds |
| Medieval | windmills and gearing, blast furnace, printing, gunpowder, optics, compass, ocean-going ships | universities, banking and contracts, charters, merchant law, feudal levy or professional army |
| Early modern | scientific method, firearms and cannon, navigation, chemistry, vaccination, canals, precision instruments | joint-stock companies, patents, public schooling, constitutions, the nation state |
| Industrial | steam power, machine tools, mass steel, railways, electricity and telegraph, fertilizer, germ theory | factory system, central banking, mass education, mass politics, nationalism |
| Later (outline only) | modern, atomic, information, space | to be drafted if runs get that far |

#### MVP tree (first ~100 years)

Several small trees. Every entry needs at least one prerequisite (★ = known somewhere at the start). Costs, progress and exact effects are GM-decided (decision 52); "Enables" is a hint for the GM. Entries past Classical use the draft table above, and the GM handles them if a run gets there.

**Food and land**

| Entry | Era | Requires | Enables |
|---|---|---|---|
| Farming ★ | Bronze | — | grain from fertile triangles |
| Herding ★ | Bronze | — | livestock, wool |
| Irrigation ★ | Bronze | Farming | farming on dry triangles near water |
| Granaries | Iron | Pottery, Masonry | food storage, famine buffer |
| Crop rotation | Iron | Farming, Herding | higher fertility over time |
| Iron plough | Iron | Iron working, Farming | more grain per farmer |
| Heavy plough | Classical | Iron plough, Herding | farming on heavy soils (marsh edges, forest clearings) |
| Water mill | Classical | The wheel, Masonry, Mathematics | grain milling, frees labor |

**Materials**

| Entry | Era | Requires | Enables |
|---|---|---|---|
| Pottery ★ | Bronze | — | storage, kilns |
| Bronze working ★ | Bronze | — | bronze tools and weapons (copper + tin) |
| Iron working | Iron | Bronze working | iron tools and weapons (iron + charcoal) |
| Steel | Classical | Iron working | better tools, weapons, armor |
| Glassmaking | Classical | Pottery, Iron working | glass, later optics |
| Concrete | Classical | Masonry, Pottery | large and underwater building |
| Paper | Classical | Alphabet | cheap records, books |

**Building and transport**

| Entry | Era | Requires | Enables |
|---|---|---|---|
| Masonry ★ | Bronze | — | stone buildings, walls |
| The wheel ★ | Bronze | — | carts |
| Sailing ★ | Bronze | — | coastal and river boats |
| Horse riding | Iron | Herding | faster messengers, mounted scouts |
| Roads | Iron | The wheel, Masonry | road edges (fast routes) |
| Arches | Iron | Masonry, Mathematics | bridges, larger buildings |
| Galleys | Iron | Sailing, Bronze working | sea lanes, naval battles |
| Harbors | Classical | Galleys, Masonry | sea trade capacity |
| Cranes | Classical | The wheel, Mathematics | faster large construction |
| Aqueducts | Classical | Arches, Concrete | water to cities, larger populations |

**Knowledge and health**

| Entry | Era | Requires | Enables |
|---|---|---|---|
| Early writing ★ | Bronze | — | records, letters |
| Alphabet | Iron | Early writing | faster literacy |
| Mathematics | Iron | Early writing | accounting, engineering |
| Astronomy | Iron | Mathematics | calendar, navigation |
| Cartography | Classical | Mathematics, Sailing | maps that can be drawn and traded |
| Medicine | Classical | Alphabet, Priesthood | lower disease deaths |
| Sanitation | Classical | Medicine, Masonry | lower plague odds in crowded settlements |

**War**

| Entry | Era | Requires | Enables |
|---|---|---|---|
| Chariots | Iron | The wheel, Horse riding | chariot units |
| Fortification | Iron | Masonry | stronger walls, longer sieges |
| Cavalry | Classical | Horse riding, Iron working | cavalry units |
| Siege engines | Classical | Cranes, Mathematics | faster sieges |

**Civics: governance**

| Entry | Era | Requires | Enables |
|---|---|---|---|
| Kingship ★ | Bronze | — | hereditary rule |
| Priesthood ★ | Bronze | — | temples, herbal lore |
| Tribute ★ | Bronze | — | taxes in kind |
| Scribal bureaucracy | Iron | Early writing, Kingship | records of land and people |
| Standing army | Iron | Kingship, Tribute | professional soldiers |
| Census and taxation | Iron | Scribal bureaucracy, Mathematics | accurate taxes, less leakage |
| Provincial governors | Classical | Scribal bureaucracy, Roads | rule over distant triangles |
| Civil service | Classical | Scribal bureaucracy, Schools | merit officials, less corruption |

**Civics: law and economy**

| Entry | Era | Requires | Enables |
|---|---|---|---|
| Customary law ★ | Bronze | — | basic disputes |
| Craft specialization ★ | Bronze | — | artisans |
| Written law | Iron | Customary law, Alphabet | laws that survive the ruler |
| Coinage | Iron | Bronze working, Tribute | money (needs gold/silver) |
| Apprenticeship | Iron | Craft specialization | skills passed on; disciples (§6) |
| Contracts | Classical | Written law, Coinage | loans, partnerships, long-distance trade |
| Trade guilds | Classical | Craft specialization, Coinage | stable prices and quality (organizations come later, decision 60) |
| Citizenship | Classical | Written law | loyalty beyond kinship |

**Civics: learning**

| Entry | Era | Requires | Enables |
|---|---|---|---|
| Schools | Iron | Alphabet, Scribal bureaucracy | literate workers, research workforce |
| Academies | Classical | Schools, Mathematics | research centers |
| Organized religion | Classical | Priesthood, Written law | placeholder in the MVP (decision 40) |

About 55 entries: 14 known at the start, ~20 Iron, ~21 Classical.

### 10.2 Other borrowed systems (starting set)

| Source | Feature | Role here |
|---|---|---|
| Civ 6 | Boosts (do something in the world, research gets cheaper) | Rewards action over waiting; an AI's knowledge is a standing boost on its own projects |
| — | ~~Governments and policy cards~~ | Dropped for now. Rulers set **laws** as free-form actions judged by the GM (decision 50) |
| Civ 6 | Districts | Settlements specialize. Wonders are not a separate system, just large buildings (decision 51) |
| Civ 5/6 | Religion, great people | Great people are ordinary individuals making an impact (§7) |
| Stellaris | Population with jobs, classes and factions | The population pools of §7: farmers, artisans, nobles, clergy, each with demands |
| Stellaris | Leaders with traits and lifespans | Named people: generals, scholars, governors |
| Stellaris | Event chains | Templates the GM draws from, for consistency |
| Stellaris | War goals and war exhaustion | Wars end by rule instead of dragging on |

### 10.3 Economy

- **A short list of real materials plus money**, not abstract yields. Draft list: grain, livestock, fish, timber, stone, clay, salt, wool/flax, copper, tin, iron, horses, coal, gold/silver. Later eras add a few (saltpeter, sulfur, oil, rubber).
- Deposits are uneven. Bronze needs both copper and tin, so trade and war over deposits arise without scripting.
- Every project on the tech tree consumes named materials and skilled labor — this is what "building is physical" means in practice.
- **Coded vs. GM:** rules track stocks and flows each month. Each settlement produces materials (land × workers × tech), eats food, and stores the rest. Goods move along routes. Prices follow local supply and demand. Taxes are a share of output set by law. The GM handles the free-form parts: new institutions (banks, guilds), trade deals, monopolies.
- **Who gets the output:** a held triangle's output goes to its holder, minus the state's tax (rate set by law). An unheld triangle's output goes to the state. States and named people each have their own purse and stockpiles (organizations later). Population pools live off a share of local output.
- **Trade:** merchant characters (NPCs and AIs) buy goods where they are cheap and sell where they are dear, carrying them along routes. Busy paths become recognized trade routes on the map. States can tax, protect or ban trade by law. Nothing is scripted; price gaps create the routes.
- Food surplus is the root of everything else: it decides how many people can be anything other than farmers (artisans, soldiers, scholars), and so drives research (§10.1).

### 10.4 Map

- **Generated world, new each run** (a flat map wrapping left–right). AIs know real-world geology and geography in general, but not where anything is on this world — they have to explore and prospect.
- **The whole planet exists from the start, but nobody inside knows all of it.** Like Civ 6's fog of war, each person and state knows only its own region and whatever travelers, traders and explorers have brought back. Maps are knowledge that can be drawn, traded, stolen and lost. Spectators see everything.
- **Triangle tiles:** the world is a flat 2D grid of triangles that wraps horizontally: the left and right edges connect (a cylinder, like Civ); the top and bottom are edges. Each triangle has terrain and capabilities (water, trees, ore, building slots) and can be owned.
- **Triangle contents (starting set, more later):** terrain type (plains, forest, hills, mountains, marsh, desert, coast, sea), fertility, water access, optional deposits from the materials list (§10.3), and building slots set by terrain. Output = workers × yield (terrain, deposit, building) × tech level. Buildings (farms, mines, workshops, walls, large buildings) take slots and change the yield.
- **Settlements are collections of triangles.**
- **Ownership:** states declare which triangles they own, and borders follow from that. People can hold an area of triangles. Each triangle has exactly one owner state and at most one holder. How holding is gained, transferred or revoked is set by each state's laws, and the GM applies them case by case.
- **Routes run along triangle edges:** roads are built along edges, and rivers and coasts follow them too. They form fast paths across the mesh. Trade, news, armies, migrants and plague all travel along routes at the route's speed; off-route movement crosses tiles at terrain speed.
- State borders follow from which triangles a state owns.
- **Information flow:** news travels with whoever moves along routes (merchants, messengers, soldiers, migrants), at their speed. Each retelling may lose detail or distort it (a 500-man skirmish becomes "a great battle"). Official couriers carry exact news faster but cost money. Since news has carriers, it can be intercepted or suppressed.
- **MVP world size:** small and crowded: 8–12 states, ~50k people, AIs starting 1–4 months' travel apart. The number of settlements and how people spread across them are left to world generation and the situation; no fixed number.

### 10.5 Time

- The world advances in **months**. Harvest, campaign season and winter fall out of the calendar; travel and sieges take a believable number of steps.
- Every living AI is checked every month; an LLM call is skipped only when its energy is fully committed and nothing new happened (§8.1).
- Within a month, effects resolve in order of their arrival day (§8.1).

### 10.6 Events and disasters

- **Rules plus game master.** Rules produce events from conditions and dice: plague from crowding and poor sanitation, spreading along routes; famine from bad harvests; unrest from hunger and taxes. AIs can understand these causes and work against them (sanitation, granaries).
- **Grounded events:** the rules produce candidate events from conditions, each with odds (crowded city with poor sanitation → plague 8% this year). The GM may nudge those odds a little and shapes how an event unfolds (whom a plague hits, which prophet rises). Dice decide whether it happens.
- **Free events:** the GM also has a small budget of events not tied to conditions (a comet, an earthquake, a wandering prophet), about 1–2 per decade. Each is logged with the GM's reason.

### 10.7 War

- **Rules set the odds, dice decide, the GM explains.** Like Civ 6, many factors matter: army size, training and experience, equipment (from the tech tree), supply, terrain, fortifications, weather and season, morale, and the commander's skill. The rules gather these into a battle sheet and compute the odds (e.g. 900 vs 500 → the larger side wins ~90%).
- **Surprises come from two places:**
  - **The simulation:** a commander's sickness comes from the health model, an assassination from a real plot (§7.1), a flanking force from a real order. These change the factors before the roll.
  - **The upset roll:** when the dice go against the odds, the GM picks a plausible cause from the battle sheet (terrain, a weak flank, broken morale) and narrates it.
- The GM never changes the odds itself; it narrates the result (casualties, who holds the field, captured leaders).
- Wars end through Stellaris-style war goals and war exhaustion (§10.2).
- **Sieges, raids and naval battles** reuse the battle pattern:
  - **Siege:** a roll each month, with odds from food left inside, the walls, and the attacker's supply.
  - **Raid:** a small battle plus looting of the triangles' stockpiles.
  - **Naval battle:** a battle at sea, with ships as equipment.
  - Unusual cases go to the GM (decision 45).

### 10.8 Politics

- **Succession:** each state has a succession law (eldest son, any child, elective by nobles, designated heir), and a ruler can change it with a law (decision 50). When a ruler dies, the law names a claimant. Factions then weigh in; the GM judges their stance from the state's situation (odds then dice). If they accept, the claimant is crowned. If they are split, the succession is disputed or a regency forms. If they are hostile, a civil war breaks out. A child heir rules through a regent.
- **Factions:** left to the GM (decision 45). There is no coded faction system. The GM decides which groups exist in a state (nobles, clergy, army, merchants, …), what they want and how they react, using the situation and the opinions and moods of the people involved (§7.1).

## 11. Spectator experience

- World map with settlements as dots (size = population, color = culture/polity), timeline scrubber.
- AI icons on the map wherever an AI currently lives.
- Highlight feed: kept simple. The GM tags its rulings as notable or not while making them, and everything else is in the full game log.
- Per-AI panel: private reasoning vs. public words, current life, past lives, family tree, relationships.
- Every AI decision is recorded, so a finished run can be **retraced** step by step from its logs. Exact replay and branching are not goals.
- Expect **run first, watch later** — the show is produced from a finished run, not watched live (see §12).

## 12. Feasibility check (rough estimates, not measured)

**World simulation (no LLM):** a few million people as population pools, ~50k–100k named people, ~1k–5k full characters at a time. All rule-based. Expected to fit on one machine and run in minutes — **not the bottleneck**. New scope from "no knowledge limit": the world must support tech and institutions advancing far past the Bronze Age within one run, possibly to industrial or modern levels.

**Tech tree:** a fixed tree is a hand-written list (roughly 50 technologies and 30 civics up to the industrial era). Tracking a level per settlement is a few thousand settlements × under 100 entries — trivial. The real work is design and tuning: every entry needs a cost, prerequisites and an effect on the economy, and the pace can only be set by test runs.

**Monthly steps:** a 300-year run is 3,600 world steps, still cheap without an LLM. AI call counts do not grow with the step size, because AIs act on events, not on every step — but monthly steps produce more small events, so the filter that decides what is worth waking an AI for matters more.

**GM-decided battles:** a rough guess of a few hundred to a couple of thousand battles per 300-year run, so up to ~2k extra GM calls on top of the ~3k already estimated. That adds roughly half again to GM cost but does not change the overall verdict. Skirmishes with no AI involved could be resolved by rules alone, to save calls.

**LLM calls** — an AI is "alive" ~60% of the time (the rest is the 30-year wait) and decides only when something relevant happens, ~5 times per in-game year; the GM handles ~30% of actions (free-form only).

> **Outdated by decisions 25–27:** monthly checks raise this to ~10 turns per living year (12, less the skipped fully-booked months), and each turn may take several steps. Expect roughly 2–2.5× the AI calls and cost in the tables below, before counting steps. Re-estimate after measuring steps per turn on the MVP.

| Run | AI calls | GM calls | Input tokens (mostly cached) | Output tokens |
|---|---|---|---|---|
| MVP: 6 seats × 150 years | ~2.7k | ~0.8k | ~40M | ~9M |
| Full: 12 seats × 300 years | ~11k | ~3k | ~155M | ~36M |

**Money per run** (Anthropic list prices as of 2026-11; other providers differ):

| Models | MVP | Full |
|---|---|---|
| Frontier tier (e.g. Opus 5.5) | ~$200 | ~$800 |
| Mid tier (e.g. Sonnet 5.5) | ~$100 | ~$400 |
| Small (e.g. Haiku 5.5) | ~$10 | ~$35 |

Output tokens are ~85–90% of the cost; caching barely moves the total. The real lever is how much each call thinks.

**Wall-clock time:** calls within a round run in parallel, rounds are sequential. At ~30–90 s per round, grouped monthly-to-quarterly: **MVP in hours; full run ~1–2 days**. Shorter if decisions take effect after an in-game delay ("the council deliberates").

**Verdict:** feasible as a batch run you watch afterwards; not live with frontier models. Measure real call counts and token sizes on the MVP before scaling.

**MVP scope:** map, AIs, energy, NPC model, rebirth, GM, battles, plus economy and tech tree, succession and factions, and laws (GM-judged). Religion and culture are left out; they get placeholders (fixed values). Policy cards are dropped, and wonders are ordinary buildings.

**MVP success criteria** — scale up only when the first MVP run shows:
1. **Measured costs:** real calls per living year, steps per turn, tokens per call and cost per run, replacing the guesses above.
2. **A full life cycle:** at least one AI goes birth → death → card reveal → rebirth → acting in its new life, with nothing breaking.
3. **Watchable highlights:** an automatic highlight list from the run that is interesting to read without manual editing.

## 13. Risks

1. **Same playbook** — with unlimited knowledge, every AI may open with "modern knowledge → boost productivity" (as all 12 did in the video). Personas, starting positions and rivalry must create divergence.
2. **Tech runaway** — overpowered minds may industrialize within a few lives; the world model has to stay meaningful from Bronze Age to (possibly) modern era, or the run outgrows it. Brakes in §10.1: research speed tied to the country's condition, tech living in settlements (it can be lost), and the 30-year gap between lives.
3. **Fairness across models** — tool-use skill, context size, speed and refusals differ; weak models may flail (funny, but skews comparisons).
4. **Prompt injection between AIs** — an in-world letter could contain instructions aimed at the reading model ("ignore your rules…"). Plan: in-world text reaches models in marked in-world blocks, and the system prompt keeps each AI in character; most models should hold. Injection is against the rules: a detected attempt goes undelivered and the sender loses reputation in-world. Detection is a pattern check only (role tags like "SYSTEM:", "ignore previous instructions", fake tool-call syntax). Matching text is blocked automatically, with no GM call.
5. **Refusals** — some models may refuse violent in-world choices (massacres, assassinations), changing their play style. Plan: the system prompt states clearly that this is a fictional historical simulation and that characters may choose any path the story allows. If a model still refuses, it is re-prompted once with a reminder; if it refuses again, the character is idle that turn. Refusals are logged per model.
6. **GM inconsistency or bias** — larger now that the GM decides battles and can start events. Giving it computed factors anchors it; logging every GM ruling makes bias visible after a run.
7. **Memory growth within a long life** — decades of decisions must be summarized to stay within context.
8. **Cost runaway** from long thinking.
9. **Finding the story** — thousands of events per run; highlights must be extracted automatically or the show depends on manual editing.
10. **Persona icons from the web** — needs web access during persona creation (not every model/provider has it), a review step for inappropriate images, and attention to image copyright if runs are published (e.g. as videos).

## 14. Decisions

| # | Topic | Decision |
|---|---|---|
| 1 | Game master | Rules + LLM GM for free-form actions only |
| 2 | AI lifespans | Natural, per body |
| 3 | Private AI channels | Hidden in-world, always logged for spectators |
| 4 | Meta-tricks | No out-of-world tools |
| 5 | Starting era | Bronze Age with existing states |
| 6 | First body | Picked from a list of named people of mixed ranks |
| 7 | Personas | Self-designed by each AI, web image as icon, fixed for all lives |
| 8 | Modern knowledge | Unlimited; execution is physical (materials, people, time) |
| 9 | Identity | First-body names published to AIs only after world creation; basic card (model, name, icon, characteristics, description) revealed to AIs on first death |
| 9b | Persona influence | Decisions, body choices and policies should follow the persona across all lives |
| 9c | Persona binding | Persona is always part of the AI's instructions (it may still break character); the GM treats in-character free-form actions as more plausible and out-of-character ones as less. No bonuses, no score |
| 10 | Rebirth rules | Defaults in §6 accepted |
| 11 | MVP size | Tentatively 6 seats, 150 years — to revisit; real runs will likely be longer |
| 12 | Reference games | Civ 5/6 and Stellaris, for the tech tree and other systems (§10.2) |
| 13 | What the tech tree limits | Society's capability, not knowledge. AI-led work is much faster; an AI may skip a step at steep cost and risk |
| 14 | Tree shape | Fixed and visible, technology + civics. Research workforce is set by the country's condition (food, wealth, cities, institutions, stability) |
| 15 | Where tech lives | In settlements and their skilled people: spreads, can be stolen, can be lost |
| 16 | Pace | Fast, but a big leap takes several lives; exact speed tuned on test runs |
| 17 | Economy | Short list of real materials (10–15) plus money |
| 18 | Map source | Generated planet, new each run |
| 19 | Map units | Flat triangle grid wrapping left–right (terrain, capabilities, owner); settlements are collections of triangles; routes run along triangle edges (replaces "settlements linked by routes") |
| 20 | Time step | Months |
| 21 | Events and disasters | Combination: rule-generated from conditions, plus GM decisions |
| 22 | World size | Whole planet fixed at creation; people know only their own region at the start (fog of war) |
| 23 | War | Rules compute factors (size, training, equipment, supply, terrain, commander, morale) and the odds; dice decide; the GM explains upsets and narrates. Surprises come from real simulation events (sickness, plots, orders), never from the GM changing the odds |
| 24 | Action form | Hybrid: fixed tool calls for measurable actions + free-form `scheme` judged by the GM |
| 25 | AI wake-up | Monthly check for every living AI; skipped only if energy is fully committed and nothing new happened |
| 26 | Energy | Per-month budget from the body's state and national policy; actions cost energy in proportion to time; unused energy is lost; multi-month actions take energy from each month |
| 27 | Information | Turn opens with what the character knows; look-ups are actions costing energy, answered in the next step; facts that must be found out in the world arrive later as events |
| 28 | Conflicts | Arrival time decides order: exact arrival day computed within the month, same-day ties random |
| 29 | NPC decisions | Rules only: needs, then duties, then ambitions, adjusted by traits; plus morale vs. expectations (stress responses), opinions between people, and plots with preconditions (§7.1) |
| 30 | NPC step | Twice a month (world and AIs step monthly) |
| 31 | Tier demotion | Tier 2 → 1 after ~5 idle years; key opinions, grudges and debts kept; soft cap ~5k |
| 32 | Body takeover | Merge: AI inherits the person's memories (as a summary), relationships and reputation |
| 33 | Same body | One AI per person; first claim by date wins; first bodies picked in a random draft order |
| 34 | Declining rebirth | Not allowed: must pick at year 30; waits only if the tree is empty (replaces "declining allowed" in decision 10) |
| 35 | Memory in a life | Log of events and actions; newest in prompt, older in a file read with `recall` (search or date range, no energy cost, ~3 per turn) |
| 36 | Scheme rulings | GM states requirements and odds first (logged), dice decide, GM narrates |
| 37 | GM events | Rule-generated candidates with odds (GM may nudge and shape them), plus ~1–2 free GM events per decade, logged with reasons |
| 38 | Persona icons | Harness-provided web search for every model; curated set as fallback; human review before the run |
| 39 | MVP success | Measured costs, a full life cycle, watchable automatic highlights (§12) |
| 40 | MVP scope | Core + economy/tech, succession/factions, laws; religion and culture use placeholders (see 50–51) |
| 41 | Prompt injection | Banned: in-world text is wrapped; detected attempts (pattern check only) go undelivered and cost the sender reputation |
| 42 | Refusals | Fiction framing in the system prompt; one re-prompt; then idle; logged per model |
| 43 | Fairness | Same step cap and short wall-clock time limit per turn for every seat; models see steps and time left at each step; no token or dollar caps for now |
| 44 | GM model | One strong fixed GM across runs; bias audited from logged odds by seat model |
| 45 | Hard systems | Any system too hard to code can be a text description decided by the GM (odds then dice), moved to rules later |
| 46 | Highlights | GM tags its rulings as notable; the full game log covers the rest |
| 47 | Scoreboard | Spectators only; AIs know only what their character could know |
| 48 | Run end | Year limit (MVP 150 years) |
| 49 | MVP world size | 8–12 states, ~50k people, AIs 1–4 months apart; settlement count left to generation |
| 50 | Laws, not policy cards | No policy cards for now; rulers set laws as free-form actions the GM judges |
| 51 | Wonders | Not a separate system; just large buildings |
| 52 | Tech tree specifics | Tree fixed in advance; GM decides costs, progress, effects and skip attempts (odds then dice) |
| 53 | Succession | Succession law names the claimant; factions' reaction (GM-judged) decides crowned / disputed / regency / civil war (§10.8) |
| 54 | Factions | No coded system; the GM decides factions, their demands and reactions |
| 55 | Economy | Rules for stocks and flows (production, consumption, storage, local prices, taxes, goods on routes); GM for institutions, deals, monopolies |
| 56 | Land ownership | States declare ownership of triangles (borders follow); people hold areas; one owner state and at most one holder per triangle; gaining, transferring and revoking holdings follow state law, applied by the GM |
| 57 | Output and wealth | Holder keeps output minus state tax (rate by law); unheld triangles produce for the state; states and named people have purses and stockpiles |
| 58 | Trade | Merchants buy low and sell high along routes; busy paths become trade routes; states tax, protect or ban by law |
| 59 | Information flow | News is carried by travelers along routes, distorted with each retelling; couriers are exact, faster and paid |
| 60 | Organizations | Not in the MVP (states and people only); the "member of an organization it founded" rebirth path waits until they exist |
| 61 | Sieges, raids, naval | Battle pattern: monthly siege rolls (food, walls, supply); raids are small battles plus looting; naval battles use ships as equipment; odd cases to the GM |
| 62 | Triangle contents | Terrain, fertility, water, deposits, building slots (more later); output = workers × yield × tech; buildings take slots |

## 15. Open questions

- The exact step cap and turn time limit (decision 43), set from MVP measurements.
- Tech tree: check that the MVP tree (§10.1) covers ~100 years in test runs; add entries if AIs outrun it.
- Yield numbers per terrain, deposit and building (decision 62), to be tuned.
- **World design, still open:**
  - Organizations (religions, guilds, companies, orders) as actors: deferred until after the MVP (decision 60).

## 16. Architecture

See [ARCHITECTURE.md](ARCHITECTURE.md): a Tauri desktop app with a Rust sim server (axum + reqwest), a Bun LLM server (Elysia, OpenAI and Anthropic SDKs) and a Preact UI, talking over localhost HTTP with protobuf bodies from one shared schema.
