"use client";

import { useEffect, useMemo, useState } from "react";
import {
  bind,
  bundle,
  encodeString,
  similarity,
  unbind,
  type Hypervector,
} from "kanerva";

const D = 4096;

export function HypervectorDemo() {
  const [role, setRole] = useState("name");
  const [filler, setFiller] = useState("alice");
  const [recordRoles, setRecordRoles] = useState([
    { role: "name", filler: "alice" },
    { role: "age", filler: "thirty" },
    { role: "city", filler: "boston" },
  ]);
  const [query, setQuery] = useState("name");
  const [recoverCandidates] = useState([
    "alice", "bob", "carol", "thirty", "twentyfive", "forty", "boston", "denver", "chicago",
  ]);

  const roleHv = useMemo(() => encodeString(`role:${role}`, D), [role]);
  const fillerHv = useMemo(() => encodeString(`filler:${filler}`, D), [filler]);
  const boundHv = useMemo(() => bind(roleHv, fillerHv), [roleHv, fillerHv]);
  const recoveredFiller = useMemo(() => unbind(boundHv, roleHv), [boundHv, roleHv]);
  const fidelity = useMemo(
    () => similarity(recoveredFiller, fillerHv),
    [recoveredFiller, fillerHv],
  );

  const record = useMemo(() => {
    const parts = recordRoles.map(({ role, filler }) =>
      bind(encodeString(`role:${role}`, D), encodeString(`filler:${filler}`, D)),
    );
    return bundle(parts);
  }, [recordRoles]);

  const queryResults = useMemo(() => {
    const roleHv = encodeString(`role:${query}`, D);
    const queried = unbind(record, roleHv);
    return recoverCandidates
      .map((c) => ({
        candidate: c,
        sim: similarity(encodeString(`filler:${c}`, D), queried),
      }))
      .sort((a, b) => b.sim - a.sim)
      .slice(0, 4);
  }, [record, query, recoverCandidates]);

  return (
    <div className="border border-[color:var(--color-rule)] bg-[color:var(--color-ink-soft)]">
      <div className="border-b border-[color:var(--color-rule)] px-6 py-3 flex items-baseline justify-between">
        <span className="spec-tag">Figure 1 · bind / unbind in the wild</span>
        <span className="font-mono text-[0.72rem] muted">D = {D}</span>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-x-10 gap-y-8 px-6 py-8">
        <section>
          <h3 className="font-display text-[1rem] mb-3 text-[color:var(--color-paper-bright)]">
            <span className="accent">a.</span> Bind a single role to a single filler
          </h3>
          <p className="muted text-[0.92rem] leading-relaxed mb-5">
            Type a role and a filler. Each is encoded into a {D}-bit hypervector; we bind them by
            XOR and unbind by XORing again with the role. The recovered filler should match the
            original at similarity <span className="accent">1.0</span>, exactly.
          </p>

          <div className="grid grid-cols-2 gap-3">
            <LabeledInput label="role" value={role} onChange={setRole} />
            <LabeledInput label="filler" value={filler} onChange={setFiller} />
          </div>

          <div className="mt-6 space-y-2">
            <HvBar label="role"    hv={roleHv} />
            <HvBar label="filler"  hv={fillerHv} />
            <HvBar label="bound"   hv={boundHv} variant="accent" />
            <HvBar label="recovered" hv={recoveredFiller} />
          </div>

          <div className="mt-5 flex items-baseline justify-between font-mono text-[0.78rem]">
            <span className="muted">sim( recovered , filler )</span>
            <span className="accent text-[1.05rem] tabular">{fidelity.toFixed(6)}</span>
          </div>
        </section>

        <section>
          <h3 className="font-display text-[1rem] mb-3 text-[color:var(--color-paper-bright)]">
            <span className="accent">b.</span> A record bundled from three pairs
          </h3>
          <p className="muted text-[0.92rem] leading-relaxed mb-5">
            Bundle three role-filler bindings into one hypervector. Query it by role; the cleanup
            memory recovers the correct filler from a fixed candidate set, above the
            ~0.5 random-pair baseline.
          </p>

          <div className="space-y-2 mb-5">
            {recordRoles.map((pair, i) => (
              <div key={i} className="grid grid-cols-2 gap-3">
                <LabeledInput
                  label={`role ${i + 1}`}
                  value={pair.role}
                  onChange={(v) =>
                    setRecordRoles((rs) => rs.map((r, j) => (j === i ? { ...r, role: v } : r)))
                  }
                />
                <LabeledInput
                  label={`filler ${i + 1}`}
                  value={pair.filler}
                  onChange={(v) =>
                    setRecordRoles((rs) => rs.map((r, j) => (j === i ? { ...r, filler: v } : r)))
                  }
                />
              </div>
            ))}
          </div>

          <div className="border-t border-[color:var(--color-rule)] pt-5">
            <div className="grid grid-cols-2 gap-3 mb-4">
              <LabeledInput label="query role" value={query} onChange={setQuery} />
              <div className="flex items-end font-mono text-[0.7rem] muted">
                ranked candidates ↓
              </div>
            </div>

            <table className="w-full font-mono text-[0.82rem]">
              <tbody>
                {queryResults.map((r, i) => (
                  <tr
                    key={r.candidate}
                    className="border-t border-[color:var(--color-rule)]"
                  >
                    <td className="py-2 pr-3 w-8 muted tabular">{i + 1}.</td>
                    <td className="py-2 pr-3">
                      <span
                        className={
                          i === 0
                            ? "text-[color:var(--color-paper-bright)]"
                            : "text-[color:var(--color-paper-muted)]"
                        }
                      >
                        {r.candidate}
                      </span>
                    </td>
                    <td className="py-2 text-right tabular">
                      <span className={i === 0 ? "accent" : "muted"}>
                        {r.sim.toFixed(4)}
                      </span>
                    </td>
                    <td className="py-2 pl-3 w-32">
                      <SimBar value={r.sim} highlight={i === 0} />
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </section>
      </div>

      <div className="border-t border-[color:var(--color-rule)] px-6 py-3 flex items-baseline justify-between font-mono text-[0.72rem] muted">
        <span>running the <span className="text-[color:var(--color-paper-bright)]">kanerva</span> package, in your browser, in real time.</span>
        <span>↳ SPEC.md §3.3</span>
      </div>
    </div>
  );
}

function LabeledInput({
  label,
  value,
  onChange,
}: {
  label: string;
  value: string;
  onChange: (v: string) => void;
}) {
  return (
    <label className="block">
      <span className="block spec-tag mb-1.5">{label}</span>
      <input
        value={value}
        onChange={(e) => onChange(e.target.value)}
        className="w-full bg-[color:var(--color-ink)] border border-[color:var(--color-rule)] focus:border-[color:var(--color-accent)] outline-none px-2.5 py-1.5 font-mono text-[0.86rem] text-[color:var(--color-paper-bright)] transition-colors"
      />
    </label>
  );
}

function HvBar({
  label,
  hv,
  variant = "default",
}: {
  label: string;
  hv: Hypervector;
  variant?: "default" | "accent";
}) {
  const bars = useMemo(() => downsampleToBars(hv, 96), [hv]);
  const color =
    variant === "accent" ? "var(--color-accent)" : "var(--color-paper)";

  return (
    <div className="flex items-center gap-3">
      <span className="font-mono text-[0.7rem] muted w-20 text-right">{label}</span>
      <div className="flex-1 flex items-end h-7 gap-[1px]">
        {bars.map((b, i) => (
          <span
            key={i}
            style={{
              height: `${Math.max(8, b * 100)}%`,
              backgroundColor: color,
              opacity: variant === "accent" ? 0.92 : 0.55,
              width: "calc((100% - 95px) / 96)",
              minWidth: "2px",
              display: "inline-block",
            }}
          />
        ))}
      </div>
    </div>
  );
}

function SimBar({ value, highlight }: { value: number; highlight: boolean }) {
  const pct = Math.max(0, Math.min(1, value)) * 100;
  return (
    <div className="h-[6px] bg-[color:var(--color-rule)] relative">
      <div
        className="h-full"
        style={{
          width: `${pct}%`,
          backgroundColor: highlight ? "var(--color-accent)" : "var(--color-paper-muted)",
        }}
      />
    </div>
  );
}

function downsampleToBars(hv: Hypervector, buckets: number): number[] {
  const step = hv.length / buckets;
  const out = new Array<number>(buckets).fill(0);
  for (let b = 0; b < buckets; b++) {
    const start = Math.floor(b * step);
    const end = Math.floor((b + 1) * step);
    let sum = 0;
    for (let i = start; i < end; i++) sum += hv[i] ?? 0;
    out[b] = sum / (end - start);
  }
  return out;
}
