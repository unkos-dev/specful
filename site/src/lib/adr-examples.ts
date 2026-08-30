import { getCollection, type CollectionEntry } from 'astro:content';

// The reference examples this site promises: every accepted ADR in docs/adr/ at the time this
// list was last reviewed. Adding a new ADR to the repository means adding its id here in the
// same change, so the build fails loudly instead of silently omitting it from the site.
export const EXPECTED_ADR_IDS = [
  'SPECFUL-ADR-0001',
  'SPECFUL-ADR-0002',
  'SPECFUL-ADR-0003',
  'SPECFUL-ADR-0004',
] as const;

export async function getAdrExamples(): Promise<CollectionEntry<'adrExamples'>[]> {
  const entries = await getCollection('adrExamples');
  const foundIds = new Set(entries.map((entry) => entry.data.id));
  const missing = EXPECTED_ADR_IDS.filter((id) => !foundIds.has(id));
  if (missing.length > 0) {
    throw new Error(
      `Expected ADR reference example(s) not found under docs/adr/: ${missing.join(', ')}. ` +
        'Either the source file moved or EXPECTED_ADR_IDS in src/lib/adr-examples.ts is stale.',
    );
  }
  return entries.slice().sort((a, b) => a.data.id.localeCompare(b.data.id));
}

export function slugFor(id: string): string {
  return id.toLowerCase();
}
