import Link from "next/link";
import fs from "node:fs/promises";
import path from "node:path";
import { marked } from "marked";

export const metadata = {
  title: "Specification · Smritidb",
  description:
    "The Smritidb specification — the cross-binding contract every implementation follows.",
};

export default async function DocsPage() {
  const file = path.join(process.cwd(), "..", "SPEC.md");
  const md = await fs.readFile(file, "utf8");
  const html = marked.parse(md, { async: false }) as string;

  return (
    <article>
      <header className="border-b border-[color:var(--color-rule)]">
        <div className="mx-auto max-w-[72rem] px-6 lg:px-10 pt-20 pb-14">
          <p className="spec-tag mb-8">⁄ specification · v0.1.0-draft</p>
          <h1 className="font-display font-light tracking-[-0.015em] text-[clamp(2.2rem,4.4vw,3.6rem)] leading-[1.06]">
            The Specification
          </h1>
          <p className="muted mt-7 max-w-[60ch] leading-relaxed">
            The contract every Smritidb implementation must satisfy. Behavior disagreements between
            an implementation and this document are bugs in the implementation, not the spec. After
            v1.0.0, breaking changes require a major version bump.
          </p>
          <p className="font-mono text-[0.78rem] muted mt-6">
            <Link href="https://github.com/Tanvrit/smritidb/blob/main/SPEC.md" className="accent no-underline">
              source on github ↗
            </Link>
            <span className="mx-3 faint">|</span>
            <Link href="/manifesto" className="no-underline">
              read the manifesto first
            </Link>
          </p>
        </div>
      </header>

      <div className="mx-auto max-w-[72rem] px-6 lg:px-10 py-16">
        <div className="body-prose" dangerouslySetInnerHTML={{ __html: html }} />
      </div>
    </article>
  );
}
