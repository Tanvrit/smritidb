import Link from "next/link";

export function Nav() {
  return (
    <header className="border-b border-[color:var(--color-rule)]">
      <div className="mx-auto max-w-[78rem] px-6 lg:px-10 flex items-baseline justify-between h-16">
        <Link href="/" className="no-underline group">
          <span
            className="font-mono text-[0.78rem]"
            style={{ letterSpacing: "var(--tracking-hairline)", textTransform: "uppercase" }}
          >
            <span className="text-[color:var(--color-paper-bright)]">Kanerva</span>
            <span className="text-[color:var(--color-paper-muted)]"> ⁄ </span>
            <span className="text-[color:var(--color-paper-muted)]">v0.1.0-draft</span>
          </span>
        </Link>
        <nav className="flex items-baseline gap-7">
          <NavLink href="/manifesto">Manifesto</NavLink>
          <NavLink href="/docs">Spec</NavLink>
          <NavLink href="https://github.com/REPLACE_WITH_OWNER/kanerva" external>
            Repository
          </NavLink>
        </nav>
      </div>
    </header>
  );
}

function NavLink({
  href,
  external = false,
  children,
}: {
  href: string;
  external?: boolean;
  children: React.ReactNode;
}) {
  const className = "font-mono text-[0.78rem] no-underline tracking-wider lowercase smcp text-[color:var(--color-paper)] hover:text-[color:var(--color-accent)] transition-colors";
  return external ? (
    <a className={className} href={href} target="_blank" rel="noreferrer">
      {children} <span aria-hidden className="text-[color:var(--color-paper-muted)]">↗</span>
    </a>
  ) : (
    <Link className={className} href={href}>
      {children}
    </Link>
  );
}
