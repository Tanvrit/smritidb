import Link from "next/link";
import { HypervectorDemo } from "../components/HypervectorDemo";

export default function Home() {
  return (
    <>
      <Hero />
      <Properties />
      <Demo />
      <Stance />
      <CallToRead />
    </>
  );
}

function Hero() {
  return (
    <section className="border-b border-[color:var(--color-rule)]">
      <div className="mx-auto max-w-[78rem] px-6 lg:px-10 pt-24 pb-28">
        <p className="spec-tag rise mb-10" style={{ animationDelay: "0.05s" }}>
          ⁄ an open associative-memory standard · v0.1.0-draft · MMXXVI
        </p>

        <h1
          className="font-display font-light leading-[0.96] tracking-[-0.02em] text-[clamp(2.6rem,6.6vw,5.6rem)] max-w-[16ch]"
          style={{ fontFamily: "var(--font-display)" }}
        >
          <span className="rise block" style={{ animationDelay: "0.15s" }}>
            Storage that
          </span>
          <span className="rise block" style={{ animationDelay: "0.3s" }}>
            <em className="not-italic accent">remembers</em> like you do.
          </span>
        </h1>

        <div
          className="mt-12 max-w-[58ch] rise"
          style={{ animationDelay: "0.55s" }}
        >
          <p className="text-[1.08rem] leading-[1.65] text-[color:var(--color-paper)]">
            Kanerva is a biology-inspired associative-memory layer for every platform.
            It treats <em>meaning</em> as a first-class addressing primitive — recall by partial
            cue, degrade like a hologram, consolidate the way the cortex consolidates while
            you sleep. An open standard for the missing layer in the storage stack.
          </p>
        </div>

        <div
          className="mt-12 flex flex-wrap items-baseline gap-x-10 gap-y-3 font-mono text-[0.82rem] rise"
          style={{ animationDelay: "0.75s" }}
        >
          <Link href="/manifesto" className="no-underline group">
            <span className="accent">read the manifesto</span>
            <span aria-hidden className="ml-1 accent inline-block group-hover:translate-x-0.5 transition-transform">→</span>
          </Link>
          <Link href="/docs" className="no-underline">
            <span>specification</span>
          </Link>
          <a
            href="https://github.com/REPLACE_WITH_OWNER/kanerva"
            target="_blank"
            rel="noreferrer"
            className="no-underline muted"
          >
            github ↗
          </a>
          <span className="font-mono text-[0.74rem] faint">
            npm install kanerva &nbsp;·&nbsp; coming with phase 1
          </span>
        </div>
      </div>
    </section>
  );
}

function Properties() {
  return (
    <section className="border-b border-[color:var(--color-rule)]">
      <div className="mx-auto max-w-[78rem] px-6 lg:px-10 pt-20 pb-24">
        <header className="mb-14 flex items-baseline justify-between">
          <h2 className="font-display text-[1.6rem] tracking-tight">
            <span className="accent">§ 1.</span> Three properties. All required.
          </h2>
          <span className="spec-tag hidden md:inline">load-bearing</span>
        </header>

        <ol className="grid grid-cols-1 md:grid-cols-3 gap-x-12 gap-y-10">
          <Property
            num="i"
            title="Fuzzy content-addressing"
            inspiration="The brain's cue-based recall — a smell, a glimpse, the whole memory comes back."
            body="Look up data by similarity, not by exact hash. Partial cues, near matches, semantic queries — all native."
          />
          <Property
            num="ii"
            title="Holographic distribution"
            inspiration="Cortical memory — each item spread across many synapses, no single address."
            body="Lose a chunk of the substrate; lose no specific item. Everything degrades a little, together. The math fades like a hologram, not like a disk."
          />
          <Property
            num="iii"
            title="Hebbian consolidation"
            inspiration="Hippocampus to cortex during sleep. Items that fire together, bind together."
            body="Frequently co-accessed items get pulled closer. Cold items summarize. The index reshapes itself by how you actually use it."
          />
        </ol>
      </div>
    </section>
  );
}

function Property({
  num,
  title,
  inspiration,
  body,
}: {
  num: string;
  title: string;
  inspiration: string;
  body: string;
}) {
  return (
    <li className="list-none">
      <span className="font-mono text-[0.78rem] accent block mb-3 tabular">{num}.</span>
      <h3 className="font-display text-[1.25rem] leading-snug text-[color:var(--color-paper-bright)] mb-3">
        {title}
      </h3>
      <p className="text-[0.92rem] muted italic mb-4 leading-relaxed">
        {inspiration}
      </p>
      <p className="text-[0.98rem] leading-[1.6]">{body}</p>
    </li>
  );
}

function Demo() {
  return (
    <section className="border-b border-[color:var(--color-rule)]">
      <div className="mx-auto max-w-[78rem] px-6 lg:px-10 pt-20 pb-24">
        <header className="mb-10">
          <h2 className="font-display text-[1.6rem] tracking-tight">
            <span className="accent">§ 2.</span> The math is real.
          </h2>
          <p className="muted mt-3 max-w-[64ch]">
            This demo runs the reference TypeScript implementation directly in your browser —
            no server, no mock. The same package you can <span className="font-mono">npm install</span>.
          </p>
        </header>

        <HypervectorDemo />
      </div>
    </section>
  );
}

function Stance() {
  return (
    <section className="border-b border-[color:var(--color-rule)]">
      <div className="mx-auto max-w-[78rem] px-6 lg:px-10 pt-20 pb-24 grid grid-cols-1 lg:grid-cols-[1fr_2fr] gap-10 items-start">
        <div>
          <h2 className="font-display text-[1.6rem] tracking-tight">
            <span className="accent">§ 3.</span> A position.
          </h2>
          <p className="spec-tag mt-4">on what this is for</p>
        </div>
        <div className="body-prose">
          <p>
            Every storage system you have used is built on the same unstated assumption: the
            question you ask later will look exactly like the address you wrote earlier.
            Filesystems, key-value stores, relational databases, object stores, content-addressed
            stores — every layer of the modern stack inherits that fifty-year-old bet.
          </p>
          <p>
            The data we store now is meaning-shaped. The questions we ask are partial, fuzzy,
            compositional, time-decaying. We have spent a decade gluing vector indexes onto
            byte-addressed storage and calling it AI infrastructure.
          </p>
          <p>
            It works. It is also the <em>least</em> native abstraction we could have picked.
          </p>
          <p>
            <Link href="/manifesto" className="accent">Read the manifesto</Link>
            &nbsp;<span className="muted">— or skim the </span>
            <Link href="/docs">specification</Link>
            <span className="muted">.</span>
          </p>
        </div>
      </div>
    </section>
  );
}

function CallToRead() {
  return (
    <section>
      <div className="mx-auto max-w-[78rem] px-6 lg:px-10 pt-20 pb-28 grid grid-cols-1 md:grid-cols-2 gap-10">
        <ReadCard
          tag="recommended reading"
          title="The manifesto"
          body="The position paper. Why the layer is missing, why the math is settled, why now."
          href="/manifesto"
        />
        <ReadCard
          tag="for implementers"
          title="The specification"
          body="The cross-binding contract. Math, data model, the canonical API, KMF wire format."
          href="/docs"
        />
      </div>
    </section>
  );
}

function ReadCard({
  tag,
  title,
  body,
  href,
}: {
  tag: string;
  title: string;
  body: string;
  href: string;
}) {
  return (
    <Link
      href={href}
      className="block group no-underline border border-[color:var(--color-rule)] hover:border-[color:var(--color-accent)] px-7 py-8 transition-colors"
    >
      <p className="spec-tag mb-4">{tag}</p>
      <h3 className="font-display text-[1.5rem] mb-3 text-[color:var(--color-paper-bright)]">
        {title}
      </h3>
      <p className="text-[0.95rem] muted leading-relaxed mb-6">{body}</p>
      <p className="font-mono text-[0.78rem] accent">
        continue
        <span aria-hidden className="inline-block ml-1 group-hover:translate-x-0.5 transition-transform">→</span>
      </p>
    </Link>
  );
}
