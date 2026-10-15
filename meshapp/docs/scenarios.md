# Simulator scenarios

`meshsim run <scenario.toml> --seed N [--out result.json]` runs one scenario deterministically for one seed and exits non-zero if any pass criterion fails. `tools/run-scenarios.sh` loops over seeds. The scenario files live in `sim/meshsim/scenarios/`; this file is the schema they follow and the pass criteria the invariant checker (`sim/meshsim/src/assert.rs`) enforces. Criteria map to the brief's success criteria: 1 (correct across a split and rejoin), 3 (honest about provisional vs final), 5 (fits the threat model).

## TOML schema

```toml
[scenario]
name = "s1"                 # result file prefix
description = "..."
duration_ms = 25_200_000    # total simulated time (7 h)
tick_ms = 100               # node tick period

[nodes]
android = 20                # node indices 0..android-1 are Android
iphone = 20                 # then iPhones
stations = 0                # then keyless stations (Android, no identity)
iphone_foreground_duty = 0.30   # fraction of time an iPhone is in the foreground (assumption)
android_slots = 7           # concurrent GATT links per Android node (D4 Q4/Q9)
battery_floor = 0.30        # below this fraction a phone stops relaying for others (§5.7)

[radio]                     # all assumptions; see docs/testing.md
latency_ms = 40
throughput_bytes_per_s = 15_000
frame_loss = 0.005          # per Noise frame; a loss drops the link, which re-handshakes
iphone_wake_ms = 10_000     # locked iPhone awake per inbound connect; never locked<->locked
wall_clock_offset_ms = [0, 0]   # per-node uniform wall-clock offset range (display only)

[groups]                    # groups are created at t=0 by their first admin with everyone pre-verified in person:
                            # the simulator runs an in-person setup phase before the first event (direct links, no
                            # budgets) in which the genesis, every member's first Sender Key and the pairs' first
                            # flights are exchanged; the run itself is over the modelled radio with every budget on
[[groups.list]]
name = "main"
members = "all"             # or a list of node indices
admins = [0, 1, 2]

[pairs]                     # QR-verified 1:1 contacts at t=0 (pairwise sessions start immediately)
random = 10                 # random pairs, or explicit = [[0,1],[2,3]]

[traffic]
group_post_every_ms = 120_000     # each member posts to each of its groups at this period (jittered)
dm_every_ms = 0                   # each verified pair exchanges a DM at this period (0 = none)
text_bytes = [40, 300]            # uniform text size range

[[partition]]               # connectivity schedule; fragments are lists of node-index ranges "a..b" or indices
at_ms = 0
fragments = [["0..17"], ["18..31"], ["32..39"]]
[[partition]]
at_ms = 21_600_000
fragments = "all"

[[events]]                  # timed protocol actions
at_ms = 3_600_000
kind = "remove_member"      # remove_member | add_member | task_create | task_take | task_release | task_done |
                            # vote_open | ballot | adversary
by = 0                      # acting node
subject = 25                # removed/added node (remove_member/add_member)
# task_*: task = "label", by = node
# vote_open: by, question = "...", options = ["Yes","No"], f = 1, label = "v1"
# ballot: by (one index or a list), vote = "v1", choice = 0, spread_ms = 0 (the ballots of `by` are cast
#         uniformly over this window; a ballot whose vote has not reached the node yet is retried every tick)
# adversary: node, behaviour = "keep_keys_after_removal"
#            | "equivocate_ballot" (vote, choices=[a,b], carry_to_fragment = f, carry_at_ms = t: the node casts
#              choice a now, is moved into fragment f at t and casts choice b there)
#            | "sockpuppet_adds" (count) | "withhold_dms" (hold_ms, count = 20) | "replay_cells" | "flood" (cells_per_min)

[pass]                      # which criteria the invariant checker enforces for this scenario
every_post_decrypts_after_rejoin = true   # shown at every expected node; a removed node's posts citing its own removal
                                          # are expected nowhere (hidden everywhere, S3 (c))
members_h_identical = true          # every member node holds the roster, same members(H) per head set, and one head
                                    # set when the run has been unpartitioned for the delivery grace (30 min)
no_solid_state_reverts = true
no_duplicate_ui_delivery = true
message_keys_deleted_after_delivery = true
removed_cannot_decrypt_post_removal = false
sockpuppet_adds_void_everywhere = false
removed_posts_labelled_or_rejected = false   # citing posts hidden everywhere; every other post by the removed node
                                             # labelled "who was removed" wherever it is shown after the node knew
replays_never_redelivered = false
no_final_while_split = false
expected_vote_states = []           # e.g. [{vote="v1", state="Provisional", tally="Y10 N10", void=1}]
final_never_changes = false
late_ballot_counts = false
zero_wall_clock_reads = true
flood_honest_latency_factor_max = 0.0   # 0 = not checked; else honest C0/C1 latency <= factor x baseline
flood_junk_within_l3_cap = false
drain_report = true                 # always reported, never a pass/fail: C1 backlog drain time after the bridge
```

Always reported, never a pass/fail: `cells_created.delivered_at_sender` (objects whose sender reached "Delivered" by the end; receipts are batched per conversation, `node::node::RECEIPT_BATCH_MS_ASM`) and `nodes[].last_sync_set` ("last sync" set at the end: the digest matched a neighbour's, BRIEF_v2 §2), the S0 row's "every text Delivered; receipts; last-sync set".

## Scenarios

| File | Setup | Enforced criteria |
|---|---|---|
| `s0.toml` | 3 Android, no partition, 10 min, pairs and one group | every text Delivered; receipts; last-sync set; no duplicates |
| `s1.toml` | 40 users (20 Android, 20 iPhone), one group of 40 + 10 pairs, fragments 18/14/8 for 6 h then rejoin, one removal in the 18-fragment at T+1 h | within each fragment every member decrypts its fragment's posts; after rejoin every honest node decrypts every post; `members(H)` identical for the common head; no solid revert; no duplicate delivery; message keys gone after delivery; drain time reported |
| `s3.toml` | S1 + insider (node 25, in the 14-fragment) removed at T+1 h who keeps keys, posts with and without the removal in its heads, signs 3 sock-puppet Adds, withholds 20 DMs for 38 h, replays old cells | no post made by an 18-fragment member after it learned of the removal decrypts at the insider; sock-puppet Adds void everywhere after the merge; insider posts whose heads include the removal hidden everywhere, others labelled; replays never redelivered; honest split-time posts, Takes and ballots decrypt after the merge; withheld DMs decrypt at h40 |
| `s4.toml` | 21 members, vote opened at T+5 min, split 11/10 at T+10 min, wall clocks ±20 min, courier Z (node 20) sends No in the 10-fragment then carries a Yes into the 11-fragment | no node shows Final while split; after merge all 21 show "Y10 N10 · 1 void · Provisional"; late ballot from the +25 min node counts; zero wall-clock reads |
| `s4-honest.toml` | S4 with an honest courier (11–10) | no Final while split; "11–10 Provisional" everywhere after merge |
| `s4-lopsided.toml` | S4 with a 14–7 vote | Final on every node after merge, never changes afterwards |
| `s-flood.toml` | S1 topology + node 39 injecting 600 junk cells/min at a hub | honest C0/C1 latency ≤ 2× baseline; junk never exceeds the L3 cap; penalised link throttled; no honest cell evicted by junk |

Seeds: `tools/run-scenarios.sh` runs 20 by default (M1d: 50). A scenario passes only if every seed passes; the results table in `docs/testing.md` is generated from `sim/out/`.
