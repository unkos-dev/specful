# Specful documentation site

This is the Astro/Starlight source for <https://unkos-dev.github.io/specful/>.

## Layout

- `src/content/docs/`: authored pages (adoption, authoring workflow, profiles, CLI reference, validation integration).
- `src/styles/`: vendored `tokens.css` and the `theme.css` overlay that maps `--sf-*` tokens onto Starlight's theme
  variables.
- `src/branding/NOTICE.md`: source, licence, and attribution for the vendored brand material.

## Commands

Run from this directory, through the repository's mise-pinned bun:

| Command | Action |
|---|---|
| `mise exec -- bun install` | Install dependencies from the committed lockfile. |
| `mise exec -- bun run dev` | Start the local authoring preview at `localhost:4321`. |
| `mise exec -- bun run build` | Build the production site to `./dist/`. |
| `mise exec -- bun run preview` | Preview the production build locally. |
