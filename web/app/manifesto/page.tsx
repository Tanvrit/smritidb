import fs from "node:fs/promises";
import path from "node:path";
import { marked } from "marked";

export const metadata = {
  title: "The Kanerva Manifesto",
  description:
    "Associative memory is the missing layer of the modern computing stack. This is the open standard that fills it.",
};

export default async function ManifestoPage() {
  const file = path.join(process.cwd(), "..", "docs", "MANIFESTO.md");
  const md = await fs.readFile(file, "utf8");
  const html = marked.parse(md, { async: false }) as string;

  return (
    <article>
      <header className="border-b border-[color:var(--color-rule)]">
        <div className="mx-auto max-w-[72rem] px-6 lg:px-10 pt-20 pb-14">
          <p className="spec-tag mb-8">⁄ position paper · MMXXVI · revision draft</p>
          <h1 className="font-display font-light tracking-[-0.015em] text-[clamp(2.2rem,4.4vw,3.6rem)] leading-[1.06] max-w-[20ch]">
            The Kanerva Manifesto
          </h1>
          <p className="muted font-mono text-[0.82rem] mt-7" style={{ letterSpacing: "0.12em" }}>
            ~ 9 min read · {wordCount(md)} words · approachable to anyone who has typed{" "}
            <span className="text-[color:var(--color-paper-bright)]">embedding = model.embed(x)</span>
          </p>
        </div>
      </header>

      <div className="mx-auto max-w-[72rem] px-6 lg:px-10 py-16 grid grid-cols-1 lg:grid-cols-[1fr_3fr] gap-x-12 gap-y-10">
        <aside className="lg:sticky lg:top-12 self-start">
          <p className="spec-tag mb-3">contents</p>
          <ol className="font-mono text-[0.78rem] space-y-2 muted">
            <li>1. The unstated assumption</li>
            <li>2. What biology already solved</li>
            <li>3. The shape of what is missing</li>
            <li>4. Three properties, all required</li>
            <li>5. Why now</li>
            <li>6. What we are building</li>
            <li>7. What revolution looks like</li>
            <li>8. The invitation</li>
          </ol>
        </aside>

        <div className="body-prose drop-cap" dangerouslySetInnerHTML={{ __html: html }} />
      </div>
    </article>
  );
}

function wordCount(text: string): number {
  return text
    .replace(/```[\s\S]*?```/g, " ")
    .split(/\s+/)
    .filter(Boolean).length;
}
