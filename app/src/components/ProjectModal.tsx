import { useEffect } from 'react'
import type { Project } from '../data/projects'
import YouTubeEmbed from './YouTubeEmbed'

const statusColor: Record<Project['status'], string> = {
  'PRE-PRINT': 'var(--blue)',
  'COMPUTE-BLOCKED': 'var(--red)',
  VALIDATED: 'var(--green)',
}

export default function ProjectModal({
  project,
  onClose,
}: {
  project: Project
  onClose: () => void
}) {
  useEffect(() => {
    const onKey = (e: KeyboardEvent) => {
      if (e.key === 'Escape') onClose()
    }
    window.addEventListener('keydown', onKey)
    document.body.style.overflow = 'hidden'
    return () => {
      window.removeEventListener('keydown', onKey)
      document.body.style.overflow = ''
    }
  }, [onClose])

  return (
    <div
      className="fixed inset-0 z-[96] flex items-center justify-center p-3 md:p-8 backdrop-in"
      style={{ background: 'rgba(23,18,23,0.45)', backdropFilter: 'blur(4px)' }}
      onClick={onClose}
    >
      <div
        className="modal-glitch-in relative w-full max-w-5xl max-h-[92vh] overflow-y-auto border-2 border-[var(--ink)] hard-shadow"
        style={{ background: 'var(--paper)' }}
        onClick={(e) => e.stopPropagation()}
        role="dialog"
        aria-modal="true"
        aria-label={project.title}
      >
        {/* title bar */}
        <div className="sticky top-0 z-10 flex items-center justify-between border-b-2 border-[var(--ink)] px-4 md:px-6 py-2"
          style={{ background: 'var(--paper-2)' }}
        >
          <span className="font-display text-lg md:text-xl tracking-[0.2em] text-[var(--ink-dim)]">
            FILE {project.num}.DAT — {project.id.toUpperCase()}
          </span>
          <button
            onClick={onClose}
            className="font-display text-lg md:text-xl tracking-[0.2em] px-3 py-0.5 border-2 border-[var(--ink)] cursor-pointer transition-colors hover:bg-[var(--red)] hover:text-[var(--paper)] hover:border-[var(--red)]"
          >
            ✕ CLOSE
          </button>
        </div>

        <div className="p-4 md:p-8">
          {/* media area — video and/or paper, both optional */}
          {project.video && (
            <div
              className="relative w-full border-2 border-[var(--ink)] overflow-hidden"
              style={{ aspectRatio: '16 / 9', background: 'var(--paper-3)' }}
            >
              {/* corner brackets */}
              <span className="absolute z-10 top-2 left-2 w-5 h-5 border-t-2 border-l-2" style={{ borderColor: 'var(--blue)' }} />
              <span className="absolute z-10 top-2 right-2 w-5 h-5 border-t-2 border-r-2" style={{ borderColor: 'var(--blue)' }} />
              <span className="absolute z-10 bottom-2 left-2 w-5 h-5 border-b-2 border-l-2" style={{ borderColor: 'var(--blue)' }} />
              <span className="absolute z-10 bottom-2 right-2 w-5 h-5 border-b-2 border-r-2" style={{ borderColor: 'var(--blue)' }} />
              {/* REC dot */}
              <span className="absolute z-10 top-3 left-1/2 -translate-x-1/2 font-display text-base tracking-[0.25em] flex items-center gap-2 pointer-events-none" style={{ color: 'var(--red)' }}>
                <span className="blink inline-block w-2.5 h-2.5 rounded-full" style={{ background: 'var(--red)' }} />
                DEMO_REEL
              </span>
              <YouTubeEmbed videoId={project.video.youtubeId} title={project.video.title} />
            </div>
          )}

          {project.paper && (
            <article
              className="relative w-full border-2 border-[var(--ink)] p-6 md:p-10"
              style={{ background: 'var(--paper-2)' }}
            >
              <div className="flex items-center justify-between font-display text-base tracking-[0.25em] text-[var(--ink-dim)] border-b border-[var(--ink)]/20 pb-3">
                <span>PRE_PRINT.PDF</span>
                <span style={{ color: 'var(--red)' }}>NOT PEER-REVIEWED</span>
              </div>
              <h3 className="mt-6 text-xl md:text-2xl font-semibold leading-snug text-center max-w-2xl mx-auto">
                {project.paper.title}
              </h3>
              <p className="mt-3 text-center font-display text-lg tracking-[0.1em] text-[var(--ink-dim)]">
                {project.paper.authors}
              </p>
              <div className="mt-6 max-w-2xl mx-auto">
                <p className="font-display text-base tracking-[0.3em] text-[var(--ink-dim)]">ABSTRACT</p>
                <p className="mt-2 text-sm md:text-base leading-relaxed text-[var(--ink)]/85 text-justify">
                  {project.paper.abstract}
                </p>
              </div>
              <div className="mt-8 text-center">
                <a
                  href={project.paper.href}
                  target="_blank"
                  rel="noreferrer"
                  className="btn-ghost inline-block cursor-pointer"
                >
                  READ FULL PRE-PRINT ▸
                </a>
              </div>
            </article>
          )}

          {/* header */}
          <div className="mt-6 md:mt-8 flex flex-wrap items-end justify-between gap-4">
            <h2 className="font-display text-5xl md:text-7xl leading-[0.85] chroma">
              {project.title}
            </h2>
            <span
              className="font-display text-lg tracking-[0.2em] px-3 py-1 border-2"
              style={{ color: statusColor[project.status], borderColor: statusColor[project.status] }}
            >
              ● {project.status}
            </span>
          </div>

          {/* meta row */}
          <div className="mt-4 flex flex-wrap gap-x-8 gap-y-2 font-display text-lg tracking-[0.15em] text-[var(--ink-dim)] border-y border-[var(--ink)]/15 py-3">
            <span>YEAR <span className="text-[var(--ink)]">{project.year}</span></span>
            <span>TYPE <span style={{ color: 'var(--red)' }}>{project.type}</span></span>
            <span>ID <span className="text-[var(--ink)]">{project.id.toUpperCase()}</span></span>
          </div>

          {/* description */}
          <div className="mt-6 space-y-4 text-base md:text-lg leading-relaxed text-[var(--ink)]/90 max-w-3xl">
            {project.description.map((p, i) => (
              <p key={i}>{p}</p>
            ))}
          </div>

          {/* stack */}
          <div className="mt-6 flex flex-wrap gap-2">
            {project.stack.map((s) => (
              <span
                key={s}
                className="font-display text-base tracking-[0.15em] px-3 py-1 border border-[var(--ink)]/40 text-[var(--ink-dim)]"
              >
                {s}
              </span>
            ))}
          </div>

          {/* links */}
          <div className="mt-8 flex flex-wrap gap-3">
            {project.demo && (
              <a
                href={project.demo.href}
                target="_blank"
                rel="noreferrer"
                className="btn-primary inline-block"
                style={{ background: 'var(--red)', borderColor: 'var(--red)' }}
              >
                {project.demo.label} ↗
              </a>
            )}
            {project.links.map((l) => (
              <a
                key={l.label}
                href={l.href}
                target="_blank"
                rel="noreferrer"
                className="btn-primary inline-block"
              >
                {l.label}
              </a>
            ))}
            <button onClick={onClose} className="btn-ghost cursor-pointer">
              ◂ BACK TO INDEX
            </button>
          </div>
        </div>
      </div>
    </div>
  )
}
