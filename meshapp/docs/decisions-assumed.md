# Decisions assumed for milestone 1 (2026-10-15)

Answers the owner gave when the build started, plus defaults adopted for the items BRIEF_v2 §9 leaves open. Change any of these by editing the named constant or module; nothing else depends on them silently.

| # | Item | Choice | Where in code |
|---|---|---|---|
| 1 | Code home | `meshapp/` in the `automation` repo, branch `claude/blissful-euler-jlj7w2` | — |
| 2 | Crypto library (brief §5.2, gate 1) | Both behind one trait. **libsignal-protocol v0.105.0** (git, AGPL-3.0-only) is the implemented default. **vodozemac 0.11.1** (Apache-2.0) is a feature-gated stub (`backend-vodozemac`); its skipped-key cap (40) needs a fork to 2000 before it is usable. Licence decision still with the owner. | `crates/meshcore/src/crypto/backend/` |
| 3 | Scope of this milestone | Rust core + multi-node simulator only. No Android/iOS code yet (design in `docs/DESIGN.md` §8–9). | — |
| 4 | SOS (brief §2, owner decision 5) | Not built at P0. Class C0 stays reserved in the wire format. | `wire::frame::Class::C0` |
| 5 | Who may create tasks (owner decision 6) | Any member. A roster `Setting` can restrict it to admins later. | `tasks::state` |
| 6 | Task conflict tie-break (owner decision 6) | For the concurrent conflict set `C`: `R = SHA256(sorted op hashes of C)`, `score(k) = SHA256("meshapp.v1.tiebreak" ‖ task_id ‖ R ‖ k)`, lowest score wins. When one claim causally saw the other it is a second claim ("2 people on this"), not a conflict. Not grindable by key choice: keys are fixed in the roster before any task exists and every task re-randomises the ranking. | `tasks::tiebreak` |
| 7 | "Needs someone" timeout (owner decision 6) | 6 h of powered-on time without hearing from the holder (assumption, tune in field tests). | `tasks::pool::NEEDS_SOMEONE_AFTER_ASM` |
| 8 | Admin add threshold (owner decision 7) | 2 distinct admins, ≥1 of whom verified the newcomer in person (brief §5.4 as applied). | `roster::eval` |
| 9 | Key change (owner decision 1) | Same-name marker (applied option). Signed retirement statements remain a P1 candidate. | `people::samename` |
| 10 | News (owner decision 2) | P2; nothing built. | — |
| 11 | Public B-build votes (owner decision 3) | Applied option (off by default, admin opt-in) is a roster `Setting`; votes themselves are P1. Milestone 1 builds the vote types and tally only because scenario S4 needs them. | `votes::*`, `objects::roster_op::RosterBody::Setting` |
| 12 | Signature policy (owner decision 4) | As applied: identity signatures only on roster ops, vote-opens, ballots; group posts carry the per-chain Sender Key signature; 1:1 uses Double Ratchet authentication. | `objects::*`, `groups::group` |

Other constants marked as assumptions in the brief or DECISIONS.md are named `*_ASM` in the code and listed in `docs/testing.md` for the gate-2 spike.
