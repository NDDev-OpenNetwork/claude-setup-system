# What This Harness Owns

Generated from `references/claude-baseline.json` by
`tools/build_nddev_builder.py`. Do not edit: the next render overwrites
it, and the baseline is where a correction belongs.

Every row below was decided by a source, and the source is named. Where
this file and the binary disagree, the binary is right -- ask it with
`claude-setup-system provider-info`.

**Configuration home**: `~/.claude`
**Environment override**: `CLAUDE_CONFIG_DIR`

## The configuration file

`settings.json` is **json**, and the parser does not accept comments.
The vendor publishes a schema at <https://json.schemastore.org/claude-code-settings.json>, and `tools/validate_setup_schemas.py` checks every shipped file that names it.

Strict JSON. Comment support is an open feature request against the vendor (`anthropics/claude-code` #12688, #17968, #29370) and is not implemented, so a `//` in this file is a parse error rather than a stylistic choice. The schema is SchemaStore's rather than the vendor's own, and `tools/validate_setup_schemas.py` fetches and checks against it.

## Owned surfaces

| path | kinds | shape |
|---|---|---|
| `CLAUDE.md` | instruction | file |
| `settings.json` | setting | file |
| `skills` | skill | directory |
| `agents` | agent | directory |
| `commands` | command | directory |
| `rules` | *(routes no kind)* | directory |
| `workflows` | *(routes no kind)* | directory |

A surface that routes no kind is owned deliberately: a backup captures
it and a restore returns it, and no component is routed there because
the kind it would carry already routes somewhere else. One kind on two
surfaces makes a consumer's route ambiguous, and the guard in
`harness_runtime::surfaces` refuses it by name.

## Considered and not owned

13 rows. Each records what was searched, so the next reader does not repeat the search:

- **`.mcp.json`** — Claude Code stores MCP servers in `.claude.json` -- user scope at the top level, local scope under projects[<path>].mcpServers -- and project scope in a .mcp.json at the project root. There is no .mcp.json under this home. `.claude.json` is in never_touch, so no MCP surface is ownable inside this target and the kind is not declared.
- **`hooks`** — Hooks are configured under a "hooks" key in settings.json. ~/.claude/hooks/ is a convention for the scripts a hook command points at, not a directory Claude Code reads. It is declared the day a setup here ships hook scripts and not before.
- **`plugins`** — Claude Code owns plugins/known_marketplaces.json, plugins/marketplaces, plugins/cache and plugins/data. Held in never_touch: a plugin component projects through settings.json, which this provider does own.
- **`.claude.json`** — Observed, not documented: with CLAUDE_CONFIG_DIR pointing at a target, the product writes this file *inside* that home rather than beside it -- measured 2026-08-28 by installing Claude Code through this provider's own software lifecycle and running `mcp add --scope user` through `launch`. It carries user-scope MCP servers, account state and project history, which the product rewrites on its own schedule. Disclaimed rather than owned: owning it would promise a rollback of an account record.
- **`backups`** — The product's own backup directory, created beside the file it backs up when `mcp add` rewrites `.claude.json`. Measured 2026-08-28. Distinct from this provider's slots, which live under its control directory.
- **`NDDEV-CLAUDE-PROVIDER.json`** — This provider's own state file: which setup is applied, the identity it recorded, and which slot reverses the last operation. Written by every operation and excluded from target identity, because counting it would leave a target different from the identity the operation just wrote. Not a projection surface and never ownable as one.
- **`.claude-setup-system`** — This provider's own control directory: the target lock, the backup slots and their payloads. Kept out of the declaration for the same reason as the state file, and recorded here because the declined list is where a reader looks before opening a file to find out what it is.
- **`keybindings.json`** — A keymap file, the same surface antigravity owns and records. Not owned here, and the asymmetry is deliberate rather than an oversight: no component kind describes a keymap, so owning it buys only backup coverage, and this provider's target already holds five namespaces a person edits by hand. Recorded so the next reader finds the answer rather than the question.
- **`statusline-command.sh`** — A shell script the product runs to render its status line, with a PowerShell sibling `statusline.ps1`. Configuration in the sense that a person chooses it, and never ownable by this provider: a setup that wrote an executable a product runs is a setup that runs code on somebody's machine.
- **`agent-runtime-state`** — One row for the agent subtree beside the owned `agents`: `agent-registry.json`, `agent-memory/<agentType>/` and `agent-memory-local/<agentType>/`. What an agent has learned and which are registered, both the product's lifetime rather than a setup's.
- **`local`** — An installed copy of the product inside its own configuration home -- `local/claude` and `local/node_modules/`. Never ownable: this provider installs programs under a `--prefix` that is deliberately not the target, and owning a directory that holds a binary would let a restore replace an executable.
- **`session-runtime-state`** — One row for the rest: `history.jsonl`, `checkpoints/`, `debug/<session>.txt`, `daemon.json`, `daemon.log`, `assistant-daemon-state.json`, `jobs`, `mailbox/`, `bash-log.txt`, `first-run`, `feedback/drafts/`. Session and daemon lifetime, none of it configuration.
- **`managed-settings`** — Not a path in the target, and named without an extension for that reason: `managed-settings.json` lives at a **system** path, one per operating system, and every recorded path here is relative to the target.
