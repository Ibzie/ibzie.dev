import { ChapterMarker } from '../components/FX'
import { profile } from '../data/projects'
import { useReveal } from '../hooks/useReveal'

const channels = [
  { label: 'LINKEDIN', value: 'linkedin.com/in/ibrahim-akhtar', href: profile.linkedin },
  { label: 'X', value: 'x.com/Ibziiee', href: profile.x },
  { label: 'GITHUB', value: 'github.com/ibzie', href: profile.github },
  { label: 'LOCATION', value: profile.location, href: null },
  { label: 'STATUS', value: profile.availability, href: null },
]

export default function Contact() {
  const ref = useReveal<HTMLDivElement>()

  return (
    <div ref={ref} className="max-w-6xl mx-auto px-5 pt-28 pb-32">
      <ChapterMarker chapter="CH.03" title="TRANSMISSION" />

      <h1 className="reveal mt-10 font-display uppercase leading-[0.8] chroma" style={{ fontSize: 'clamp(3.5rem, 11vw, 9rem)' }}>
        OPEN<br />COMMS
      </h1>

      <p className="reveal mt-6 max-w-xl text-lg text-[var(--ink)]/85">
        Research collaboration, compute access, or interesting problems? Reach out on LinkedIn or X
        if you need to get in touch. The fastest way to reach me is through LinkedIn or X.
      </p>

      <div className="mt-10 border-2 border-[var(--ink)] hard-shadow" style={{ background: 'var(--paper-2)' }}>
        <div className="border-b-2 border-[var(--ink)] px-4 py-1.5 font-display text-base tracking-[0.25em] text-[var(--ink-dim)]">
          CHANNEL.LIST
        </div>
        {channels.map((c) => (
          <div
            key={c.label}
            className="flex flex-wrap items-baseline justify-between gap-2 px-4 py-4 border-b border-[var(--ink)]/10 last:border-b-0"
          >
            <span className="font-display text-lg tracking-[0.25em] text-[var(--ink-dim)]">{c.label}</span>
            {c.href ? (
              <a
                href={c.href}
                target="_blank"
                rel="noreferrer"
                className="font-display text-2xl md:text-3xl tracking-[0.08em] transition-all duration-150 hover:translate-x-1 chroma-sm"
                style={{ color: 'var(--blue)' }}
              >
                {c.value} ▸
              </a>
            ) : (
              <span className="font-display text-2xl md:text-3xl tracking-[0.08em]">{c.value}</span>
            )}
          </div>
        ))}
      </div>

      <p className="reveal mt-8 font-display text-lg tracking-[0.15em] text-[var(--ink-dim)]">
        ▸ FASTEST RESPONSE ON LINKEDIN / X · SEE CHANNEL.LIST ABOVE
      </p>
    </div>
  )
}
