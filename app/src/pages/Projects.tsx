import { useState } from 'react'
import { projects, profile, type Project } from '../data/projects'
import { ChapterMarker } from '../components/FX'
import ProjectModal from '../components/ProjectModal'
import { useReveal } from '../hooks/useReveal'

const statusColor: Record<Project['status'], string> = {
  'PRE-PRINT': 'var(--blue)',
  'COMPUTE-BLOCKED': 'var(--red)',
  VALIDATED: 'var(--green)',
  RELEASED: 'var(--green)',
}

export default function Projects() {
  const [open, setOpen] = useState<Project | null>(null)
  const ref = useReveal<HTMLDivElement>()

  return (
    <div ref={ref} className="max-w-6xl mx-auto px-5 pt-28 pb-32">
      <ChapterMarker chapter="CH.01" title="PROJECT INDEX" />

      <div className="mt-6 flex flex-wrap items-baseline justify-between gap-2 reveal">
        <h1 className="font-display text-5xl md:text-7xl leading-[0.85] chroma">ALL PROJECTS</h1>
        <p className="font-display text-lg tracking-[0.15em] text-[var(--ink-dim)]">
          {projects.length} ENTRIES · {profile.github.replace('https://', '').toUpperCase()} · PRE-PRINT ARCHIVE
        </p>
      </div>

      {/* column header */}
      <div className="mt-10 hidden md:grid grid-cols-[3rem_5rem_1fr_14rem_6rem] gap-4 font-display text-base tracking-[0.2em] text-[var(--ink-dim)] border-b-2 border-[var(--ink)] pb-2 reveal">
        <span>Nº</span>
        <span>YEAR</span>
        <span>TITLE</span>
        <span>TYPE</span>
        <span className="text-right">STATUS</span>
      </div>

      {/* index rows: siblings dim on hover */}
      <div className="group/list">
        {projects.map((p) => (
          <button
            key={p.id}
            onClick={() => setOpen(p)}
            className="project-row reveal w-full text-left cursor-pointer border-b border-[var(--ink)]/15 py-5 md:py-6
                       grid grid-cols-[2.5rem_1fr_auto] md:grid-cols-[3rem_5rem_1fr_14rem_6rem] gap-4 items-center
                       transition-all duration-200 hover:translate-x-1.5 hover:bg-[var(--yellow)]/25
                       group-hover/list:opacity-40 hover:!opacity-100"
          >
            <span className="font-display text-xl text-[var(--ink-dim)]">{p.num}</span>
            <span className="hidden md:block font-display text-xl text-[var(--ink-dim)]">{p.year}</span>
            <span className="min-w-0">
              <span className="project-title block font-display text-3xl md:text-4xl leading-none uppercase truncate">
                {p.title}
              </span>
              <span className="block mt-1 text-sm text-[var(--ink-dim)] truncate">{p.tagline}</span>
              <span className="md:hidden block mt-1 font-display text-base tracking-[0.15em]" style={{ color: 'var(--red)' }}>
                {p.type} · {p.year}
              </span>
            </span>
            <span className="hidden md:block font-display text-lg tracking-[0.12em]" style={{ color: 'var(--red)' }}>
              {p.type}
            </span>
            <span className="font-display text-base md:text-lg tracking-[0.15em] text-right" style={{ color: statusColor[p.status] }}>
              ● {p.status}
              <span className="block text-[var(--ink-dim)]">
                OPEN FILE ▸ {p.video && p.paper ? '[VID+PDF]' : p.video ? '[VID]' : p.paper ? '[PDF]' : '[—]'}
              </span>
            </span>
          </button>
        ))}
      </div>

      <p className="mt-8 font-display text-lg tracking-[0.15em] text-[var(--ink-dim)] reveal">
        ▸ SELECT AN ENTRY TO OPEN ITS FILE · DEMO REEL + FULL WRITE-UP
      </p>

      <div className="mt-6 reveal border-2 border-[var(--ink)] hard-shadow" style={{ background: 'var(--paper-2)' }}>
        <div className="flex flex-wrap items-center justify-between gap-4 px-4 py-4">
          <p className="text-base md:text-lg text-[var(--ink)]/85 max-w-xl">
            This index is only a slice of the work. For experiments, tools, and one-off builds that
            didn't make the cut, browse the full archive on GitHub.
          </p>
          <a
            href={profile.github}
            target="_blank"
            rel="noreferrer"
            className="btn-primary inline-block whitespace-nowrap"
          >
            MORE PROJECTS ON GITHUB ↗
          </a>
        </div>
      </div>

      {open && <ProjectModal project={open} onClose={() => setOpen(null)} />}
    </div>
  )
}
