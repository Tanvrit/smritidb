export function Footer() {
  return (
    <footer className="border-t border-[color:var(--color-rule)] mt-32">
      <div className="mx-auto max-w-[78rem] px-6 lg:px-10 py-10 grid grid-cols-1 md:grid-cols-4 gap-8 text-[0.85rem]">
        <div>
          <p className="spec-tag mb-3">colophon</p>
          <p className="muted leading-relaxed">
            Set in Spectral and IBM Plex Mono. Drafted in <span className="font-mono">SPEC.md</span>.
            Compiled to bits.
          </p>
        </div>
        <div>
          <p className="spec-tag mb-3">substrate</p>
          <p className="muted leading-relaxed">
            Binary HDC with sparse-distributed-memory backing. Bit-exact across bindings via the
            Kanerva Memory Format (KMF).
          </p>
        </div>
        <div>
          <p className="spec-tag mb-3">lineage</p>
          <p className="muted leading-relaxed">
            Kanerva 1988 · Plate 1995 · Kanerva 2009.
            See <span className="font-mono">CITATION.cff</span>.
          </p>
        </div>
        <div>
          <p className="spec-tag mb-3">license</p>
          <p className="muted leading-relaxed">
            Implementations <span className="font-mono">MIT</span>.
            Specification <span className="font-mono">CC-BY-4.0</span> once frozen.
          </p>
        </div>
      </div>
      <div className="border-t border-[color:var(--color-rule)] py-6">
        <div className="mx-auto max-w-[78rem] px-6 lg:px-10 flex flex-col md:flex-row items-baseline justify-between gap-4 text-[0.72rem] font-mono">
          <p className="muted" style={{ letterSpacing: "0.18em", textTransform: "uppercase" }}>
            an open associative-memory standard
          </p>
          <p className="faint">
            specification draft · MMXXVI · revision 0.1.0
          </p>
        </div>
      </div>
    </footer>
  );
}
