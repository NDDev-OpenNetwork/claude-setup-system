# Writing a command for this harness

Generated from the vendor's own reference and the pinned binary. Do not edit: the next render overwrites it, and a correction belongs in the source this file is derived from.

**Where it goes**: `~/.claude/commands/<name>.md`

**Decided by**: https://code.claude.com/docs/en/skills

**How it runs**: `/<name>`

## Frontmatter

| field | required | what it does |
|---|---|---|
| `description` | no | Shown in the `/` menu. |
| `argument-hint` | no | Autocomplete hint. |
| `name` | — | **Accepted and ignored.** The product names it as a field it does not act on. |
| `paths` | — | **Accepted and ignored.** The product names it as a field it does not act on. |

## What bites

- **`name` and `paths` are read in a skill and dropped in a command file.** The vendor says so where the two are now documented together: a command is invoked by its filename, so a `name` here is a second place to be wrong with nothing reading it, and a `paths` glob scopes a skill and is ignored on a command. This is the asymmetry this table exists for -- the same frontmatter, honoured in one directory and decoration in the one beside it.
- **Custom commands were merged into skills.** `.claude/commands/deploy.md` and `.claude/skills/deploy/SKILL.md` both create `/deploy` and work the same way; existing command files keep working, and where both exist under one name the **skill wins**. Nothing here ships a command and a skill under one name, and that is now a thing to keep true rather than an accident.
- This surface is the older half of the skill mechanism above and still works. Prefer a skill when the component needs supporting files or wants Claude to load it without being asked.

## The same file on the other harnesses

Generated from the same rows as the section above, for every harness in this estate that routes this kind. `—` means the product's own reference does not name the field, and **dropped** means it names it as one it accepts and does not act on.

| field | `claude` | `codex` | `pi` | `opencode` | `cursor` | `antigravity` |
|---|---|---|---|---|---|---|
| `description` | yes | yes | yes | yes | yes | yes |
| `argument-hint` | yes | yes | yes | — | — | — |
| `name` | **dropped** | — | — | — | yes | — |
| `paths` | **dropped** | — | — | — | — | — |
| `agent` | — | — | — | yes | — | — |
| `model` | — | — | — | yes | — | — |
| `subtask` | — | — | — | yes | — | — |
| `title` | — | — | — | — | — | yes |

**The part that travels**: `description`. Everything else is a bet on one product.

**The part that does not, and says nothing when it does not**: a field absent from a column is not rejected there -- it is read past. Nothing warns, no run fails, and the component behaves differently with the same bytes. Where the field was carrying a restriction, the restriction is simply gone. Check the column before relying on one.

## Before you ship one

- **The surface is declared, so the component is a promise.** Every kind   this provider declares is a promise of a rollback. A component written   to a path the declaration does not carry is installed by nobody and   removed by nobody.
- **Name it once.** Where the product derives identity from the directory   or the filename, the frontmatter `name` is either redundant or a second   place to be wrong. Keep them equal.
- **Read it back.** After an install, look at the file where the product   reads it, not at the step that put it there.
