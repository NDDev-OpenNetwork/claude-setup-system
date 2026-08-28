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

## Using this against a home you already have

**An owned namespace is removed whole.** The table below says what this build
owns; `remove` deletes each of those paths entirely, and a backup slot holds
what was there first. That includes content this build never wrote -- if the
product itself put a key in a configuration file this provider owns, `remove`
takes the file, not the keys this provider added to it.

Measured, with the real product: launching Codex through `launch` and running
`mcp add` writes `~/.codex/config.toml` with an `[mcp_servers.*]` entry; a
later `install` captures that file into a slot and replaces it; a later
`remove` deletes it. The entry is not lost -- `backups` lists the slot as
*before install, setup none*, and restoring it returns the file byte for byte
-- but it is not in the target either.

So: point `--target` at a home you are willing to have managed. `backups
--target <dir>` names every earlier state and which setup each preceded, and
`restore --backup <ref>` returns any of them exactly.

## When conformance says this provider is malformed

`ai-stp provider conformance --protocol-version 3` reports each case by name.
If the one that fails is `provider_info_v3_closed`, with a detail about fields
differing from the closed schema, **check the version of the checker before
suspecting this build**.

The v3 capability schema is compared as an exact field set, so a provider that
declares a field the checker predates is reported as malformed rather than as
newer. `scoped_projection_profiles` (`ADR-0125`) is the field this applies to,
and it is omitted entirely when empty -- so a build that declares no scope
satisfies an older checker by accident, and a build that declares one does not.

Two versions, two different answers, both measured:

| checker | result |
| --- | --- |
| `ai-stp-cli` 0.0.3 | five pass; Codex and Antigravity report `conforms=false`, detail *fields differ from the closed v3 schema* |
| `ai-stp-cli` 0.0.7 | six pass 23 of 23; Codex reports `conforms=false`, detail *a scoped projection profile names an unknown target scope* |

The remaining one is not a defect in this build. `0.0.7` carries the field but
its scope enum is `["project"]` alone, while the provider kit this program
vendors and verifies byte-for-byte -- kit `0.2.4`,
`provider-info.schema.json` -- gives `["project", "user_root"]`. The kit is the
artifact a provider is told to build against, so a build that declares
`user_root` is right by the document it was handed and wrong by the checker
shipped beside it. Raised with the consumer, who owns both.

Which is the general rule this section exists for: **check the version of the
checker before suspecting this build**, and prefer the newest, because an older
one reports a wider failure than the one it found.

## What `status` reports, and what it does not

`state` answers **who manages this target**, and never *whether a setup is
installed*. Three values, and the distinction matters most for the fourth
situation, which is not a fourth value:

| | |
| --- | --- |
| `missing` | the directory is empty |
| `unmanaged` | it holds content, none of it this provider's |
| `managed` | this provider's state file is present and current |

`missing` used to be looser -- it asked whether this provider owned anything,
so a directory full of another product's files reported `missing`. A consumer
reads this to decide what it is looking at, and being told a populated
directory is empty invites it to treat the place as free. Emptiness is about
the directory, not about us.

**After a `remove`, `state` stays `managed`, and that is the honest answer.**
The setup is gone -- no file a product reads survives it -- but the control
directory and a backup slot remain, and that slot is what makes the removal
reversible: `restore` brings the setup back. A target reported as `missing`
while a restore is pending would be a lie in the direction that costs someone
their data.

Whether a setup is installed is carried by `setup_stable_id`, which is `null`
exactly when none is. That is the field to test, not this word.
`target_identity_digest` corroborates it -- after a remove it is the digest of
an empty tree -- but the field is the direct answer and the digest is not.

## The network, stated exactly

**This artifact does not link the network, and no local phase can spawn
anything that could.** Two lints hold it rather than a promise: `std::net` is
refused outright, and `std::process::Command` is refused everywhere but two
named places -- the `launch` command, which is declared in `provider-info` and
absent from builds that do not declare it, and a lifecycle probe that drives
this binary's own executable. Adding a `tar` shell-out to ordinary code fails
the build with *only `launch` may spawn, and it is declared*. Every crate that
may be linked is named in `deny.toml`, so a transitive dependency cannot arrive
unread.

Those are claims about the source, and a lint can be wrong, bypassed, or simply
disbelieved. So `ci` reads the shipped binary too: a `boundary` job asks the
import table of the artifact this build produces whether any network symbol is
present, and whether a build declaring no `launch` imports anything that could
spawn. You can run it yourself against a downloaded release --
`nm -D --undefined-only <binary>` on Linux, `nm -u` on macOS -- and it needs no
part of this repository to be trusted.

**What that does not buy, said plainly because the stronger claim is the
tempting one.** This is a dynamically linked program: it imports `syscall` from
libc like any other, so no property of the binary can prove a socket is
unreachable to code that is determined to open one. What is proven is narrower
and still worth having: no code path here reaches for the network, none can be
added without the build refusing, and no local phase can hand the job to a
child process. If your threat model needs the guarantee rather than the
absence, run `plan` and `apply` under whatever sandbox you already trust; both
phases are offline by design, and `apply` verifies the digests it was given
with the network gone.

## What this build owns inside a target

Everything else in the target is a sibling overlay and is preserved
verbatim. Each row cites the vendor page it was read from, and the same
table is bound to the declaration by a test, so this cannot drift from
what `provider-info` publishes.

Configuration home as the product documents it: `~/.claude`.

| Path | Component kinds routed here | Decided by |
| --- | --- | --- |
| `CLAUDE.md` | `instruction` | [source](https://code.claude.com/docs/en/memory) |
| `settings.json` | `setting` | [source](https://code.claude.com/docs/en/settings) |
| `skills` | `skill` | [source](https://code.claude.com/docs/en/skills) |
| `agents` | `agent` | [source](https://code.claude.com/docs/en/sub-agents) |
| `commands` | `command` | [source](https://code.claude.com/docs/en/slash-commands) |
| `rules` | -- | [source](https://code.claude.com/docs/en/memory) |
| `workflows` | -- | [source](https://code.claude.com/docs/en/claude-directory; measured in the pinned 2.1.250 binary) |

A path routing no component kind is owned so a setup can carry it;
nothing compiles a component to it.

### Considered and not owned

Everything named here is left exactly as it was found, like any
other file beside a target.

**`.mcp.json`** -- Claude Code stores MCP servers in `.claude.json` -- user scope at the top level, local scope under projects[<path>].mcpServers -- and project scope in a .mcp.json at the project root. There is no .mcp.json under this home. `.claude.json` is in never_touch, so no MCP surface is ownable inside this target and the kind is not declared. ([source](https://code.claude.com/docs/en/mcp))

**`hooks`** -- Hooks are configured under a "hooks" key in settings.json. ~/.claude/hooks/ is a convention for the scripts a hook command points at, not a directory Claude Code reads. It is declared the day a setup here ships hook scripts and not before. ([source](https://code.claude.com/docs/en/hooks))

**`plugins`** -- Claude Code owns plugins/known_marketplaces.json, plugins/marketplaces, plugins/cache and plugins/data. Held in never_touch: a plugin component projects through settings.json, which this provider does own. ([source](https://code.claude.com/docs/en/plugin-marketplaces))

**`.claude.json`** -- Observed, not documented: with CLAUDE_CONFIG_DIR pointing at a target, the product writes this file *inside* that home rather than beside it -- measured 2026-08-28 by installing Claude Code through this provider's own software lifecycle and running `mcp add --scope user` through `launch`. It carries user-scope MCP servers, account state and project history, which the product rewrites on its own schedule. Disclaimed rather than owned: owning it would promise a rollback of an account record. ([source](measured through launch; no vendor page states the path under a redirected home))

**`backups`** -- The product's own backup directory, created beside the file it backs up when `mcp add` rewrites `.claude.json`. Measured 2026-08-28. Distinct from this provider's slots, which live under its control directory. ([source](measured through launch; no vendor page names it))

**`NDDEV-CLAUDE-PROVIDER.json`** -- This provider's own state file: which setup is applied, the identity it recorded, and which slot reverses the last operation. Written by every operation and excluded from target identity, because counting it would leave a target different from the identity the operation just wrote. Not a projection surface and never ownable as one. ([source](this provider's own contract; no vendor page is involved))

**`.claude-setup-system`** -- This provider's own control directory: the target lock, the backup slots and their payloads. Kept out of the declaration for the same reason as the state file, and recorded here because the declined list is where a reader looks before opening a file to find out what it is. ([source](this provider's own contract; no vendor page is involved))

**`keybindings.json`** -- A keymap file, the same surface antigravity owns and records. Not owned here, and the asymmetry is deliberate rather than an oversight: no component kind describes a keymap, so owning it buys only backup coverage, and this provider's target already holds five namespaces a person edits by hand. Recorded so the next reader finds the answer rather than the question. ([source](measured from the pinned 2.1.250 binary's path literals))

**`statusline-command.sh`** -- A shell script the product runs to render its status line, with a PowerShell sibling `statusline.ps1`. Configuration in the sense that a person chooses it, and never ownable by this provider: a setup that wrote an executable a product runs is a setup that runs code on somebody's machine. ([source](measured from the pinned 2.1.250 binary's path literals))

**`agent-runtime-state`** -- One row for the agent subtree beside the owned `agents`: `agent-registry.json`, `agent-memory/<agentType>/` and `agent-memory-local/<agentType>/`. What an agent has learned and which are registered, both the product's lifetime rather than a setup's. ([source](measured from the pinned 2.1.250 binary's path literals))

**`local`** -- An installed copy of the product inside its own configuration home -- `local/claude` and `local/node_modules/`. Never ownable: this provider installs programs under a `--prefix` that is deliberately not the target, and owning a directory that holds a binary would let a restore replace an executable. ([source](measured from the pinned 2.1.250 binary's path literals))

**`session-runtime-state`** -- One row for the rest: `history.jsonl`, `checkpoints/`, `debug/<session>.txt`, `daemon.json`, `daemon.log`, `assistant-daemon-state.json`, `jobs`, `mailbox/`, `bash-log.txt`, `first-run`, `feedback/drafts/`. Session and daemon lifetime, none of it configuration. ([source](measured from the pinned 2.1.250 binary's path literals))

**`managed-settings`** -- Not a path in the target, and named without an extension for that reason: `managed-settings.json` lives at a **system** path, one per operating system, and every recorded path here is relative to the target.

  * Linux — `/etc/claude-code/managed-settings.json`
  * macOS — `/Library/Application Support/ClaudeCode/managed-settings.json`
  * Windows — `%ProgramData%\\ClaudeCode\\managed-settings.json`

All three are path literals in the pinned product. Grok's own compatibility documentation describes the file, and the description is the reason this row exists: *"a protected policy file for allowlists and defaults. Its values take precedence over user, project and local config **and cannot be overridden**."*

**It bears on the `full-auto` posture.** That setup writes `permissions.defaultMode = bypassPermissions` and `sandbox.enabled = false` into the owned `settings.json`. Under a managed policy those keys are correct, at a correct path, in a file the product reads — and a higher layer overrides them. The setup installs, verifies and restores cleanly and **changes nothing**.

That is the third instance of one defect class in this estate: opencode's bare-string `permission`, antigravity's `toolPermissions`, and now a correct key under a layer that outranks it. The first two were fixed by writing the right key; this one cannot be fixed by writing anything, which is why it is written down instead.

**Nothing to own and nothing to check at runtime.** A provider that read a system policy in order to warn about it would be reading a file it promises not to touch, on a path outside every target it is given. ([source](path literals measured in the pinned product, 2026-08-28; the precedence sentence is from grok 1.0.5's embedded compatibility reference))

## Response

One maintainer. Defects are triaged as time allows; security reports are
acknowledged first.
