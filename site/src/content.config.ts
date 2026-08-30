import { defineCollection, z } from 'astro:content';
import { docsLoader } from '@astrojs/starlight/loaders';
import { docsSchema } from '@astrojs/starlight/schema';
import { glob } from 'astro/loaders';

// The reference examples the site shows are the repository's own live ADRs, loaded straight
// from docs/adr/ at build time. There is exactly one authored copy of each; the site only
// renders it. adrExamples.ts asserts every id in EXPECTED_ADR_IDS was found, so a missing or
// renamed source file fails the build instead of silently shrinking the reference set.
// YAML parses an unquoted ISO date as a Date, not a string; frontmatter carries dates that way,
// so both shapes are accepted and normalised to the YYYY-MM-DD string the pages render.
const isoDate = z
  .union([z.string(), z.date()])
  .transform((value) => (value instanceof Date ? value.toISOString().slice(0, 10) : value));

const adrFrontmatter = z
  .object({
    type: z.literal('ADR'),
    'profile-version': z.literal(1),
    id: z.string(),
    title: z.string(),
    status: z.enum(['proposed', 'accepted', 'deprecated', 'superseded']),
    'recorded-on': isoDate,
    'decided-on': isoDate.optional(),
    'decision-makers': z.array(z.string()).optional(),
    consulted: z.array(z.string()).optional(),
    informed: z.array(z.string()).optional(),
    supersedes: z.array(z.string()).optional(),
    'superseded-by': z.array(z.string()).optional(),
  })
  .passthrough();

export const collections = {
  docs: defineCollection({ loader: docsLoader(), schema: docsSchema() }),
  adrExamples: defineCollection({
    // Numeric-prefixed files only: docs/adr/README.md is index prose, not an ADR artifact.
    loader: glob({ pattern: '[0-9]*.md', base: '../docs/adr' }),
    schema: adrFrontmatter,
  }),
};
