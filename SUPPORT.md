# Support

## Before opening anything

`--help` states what this build does and does not do. `status --target <dir>
--json` reports what it found in a target without changing it, and its output is
safe to share: it carries identities and digests, never secret values.

## Where to go

| You have | Go to |
| --- | --- |
| A defect | [Issues](../../issues) — use the defect template |
| A question about behaviour | [Issues](../../issues) — a blank issue is fine |
| A vulnerability | [Security advisories](../../security/advisories/new), privately |

Never open a public issue for a vulnerability, and never paste credentials,
tokens, or the contents of a backup slot anywhere in this repository. A backup
slot holds whatever the target held when it was captured.

## What this build does, and what it does not

The software lifecycle — installing, updating and removing the product
itself — is declared and does work. `plan` names the exact bytes offline,
whoever holds the network fetches them, and `apply` verifies and installs
with the network gone.

`launch` is declared. It starts the exact executable a software install
placed under `--prefix`, never a name found on `PATH`, and points the
product at `--target` through the environment variable its own
documentation names.

A provider that advertised an operation it cannot perform would let a caller ask
for something that cannot be honoured, which is worse than not offering it.

All five core operations do work: `backup`, `restore`, `remove`, `install` and
`replace`, both from the local setup catalog and from an `ai-stp-bundle/1`
arriving over the wire.

## What this build owns inside a target

Everything else in the target is a sibling overlay and is preserved
verbatim. Each row cites the vendor page it was read from, and the same
table is bound to the declaration by a test, so this cannot drift from
what `provider-info` publishes.

Configuration home as the product documents it: `~/.claude`.

| Path | Component kinds routed here | Decided by |
| --- | --- | --- |
| `CLAUDE.md` | `instruction` | [source](https://code.claude.com/docs/en/memory) |
| `settings.json` | `setting`, `plugin` | [source](https://code.claude.com/docs/en/settings) |
| `skills` | `skill` | [source](https://code.claude.com/docs/en/skills) |
| `agents` | `agent` | [source](https://code.claude.com/docs/en/sub-agents) |
| `commands` | `command` | [source](https://code.claude.com/docs/en/slash-commands) |

A path routing no component kind is owned so a setup can carry it;
nothing compiles a component to it.

### Considered and not owned

Everything named here is left exactly as it was found, like any
other file beside a target.

**`.mcp.json`** -- Claude Code stores MCP servers in ~/.claude.json -- user scope at the top level, local scope under projects[<path>].mcpServers -- and project scope in a .mcp.json at the project root. There is no ~/.claude/.mcp.json. ~/.claude.json is in never_touch, so no MCP surface is ownable inside this target and the kind is not declared. ([source](https://code.claude.com/docs/en/mcp))

**`hooks`** -- Hooks are configured under a "hooks" key in settings.json. ~/.claude/hooks/ is a convention for the scripts a hook command points at, not a directory Claude Code reads. It is declared the day a setup here ships hook scripts and not before. ([source](https://code.claude.com/docs/en/hooks))

**`plugins`** -- Claude Code owns plugins/known_marketplaces.json, plugins/marketplaces, plugins/cache and plugins/data. Held in never_touch: a plugin component projects through settings.json, which this provider does own. ([source](https://code.claude.com/docs/en/plugin-marketplaces))

## Response

One maintainer. Defects are triaged as time allows; security reports are
acknowledged first.
