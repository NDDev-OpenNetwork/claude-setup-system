# The second target this harness owns

## `target_scope: project`, rooted at `project root`

**`project root` is not this product's configuration home.** It is a
different target, reached by a consumer naming the scope on the
request, and every path below is relative to that root rather than
to the home -- writing the root into the path again would nest it
twice, which is a mistake this estate has made and shipped.

| path | routes | decided by | exercised by |
|---|---|---|---|
| `CLAUDE.md` | instruction | <https://code.claude.com/docs/en/memory; re-read 2026-09-03 for Claude Code 2.1.259> | *nothing -- a page* |
### `CLAUDE.md`, as measured

Anthropic's current scope table names both ./CLAUDE.md and ./.claude/CLAUDE.md as team-shared project instructions. This first profile owns only the repository-root spelling: the immutable adaptation selects it, while claiming both would give one instruction kind two destinations without a selector.


Considered under this scope and not owned:

- **`.claude/CLAUDE.md`** — The product reads it, but the first explicit project adaptation selects repository-root CLAUDE.md. Owning both would make one component kind ambiguous until its immutable adaptation names the alternate member.
- **`CLAUDE.local.md`** — The vendor defines this as private local project preference and recommends gitignore. A public setup must not replace a person's local override.


**A complete setup may include these scoped components.** Each
provider request still reaches one root. The consumer coordinates
the roots with `ai-stp install transaction plan`, exact digest
approval, apply and recovery. A shipped configuration-home preset
cannot reach this root by nesting a path inside its home payload.
Declare the component's actual scope and bind the matching root
explicitly in the transaction.

**The root is shared, and that changes what removal means.** Several
products read it. Under this scope `remove`, the backup and a
restore act on the files this provider recorded writing rather than
on the directory whole, so a neighbour's files are never captured
into a slot here and never reverted out of one.

