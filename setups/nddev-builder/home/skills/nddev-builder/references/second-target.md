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


**A setup cannot carry one of these.** A setup is installed into one
target and its payload is relative to that target, so a component
for this scope is installed by the consumer against that root -- not
by a setup aimed at the configuration home. If you are looking for
where to put one by hand, it is the path above joined to the root
above, and nowhere under the home.

**The root is shared, and that changes what removal means.** Several
products read it. Under this scope `remove`, the backup and a
restore act on the files this provider recorded writing rather than
on the directory whole, so a neighbour's files are never captured
into a slot here and never reverted out of one.

