import { ChapterMarker } from '../components/FX'
import { useReveal } from '../hooks/useReveal'

const skills = [
  'PYTORCH / CUDA KERNELS',
  'GENERATIVE MODELS (VAE, DIFFUSION)',
  'MECHANISTIC INTERPRETABILITY',
  'MIXTURE-OF-EXPERTS SYSTEMS',
  'DISTRIBUTED TRAINING (RAY)',
  'PYTHON / TYPESCRIPT',
  'MLOPS / DEPLOYMENT',
]

const timeline = [
  {
    period: '2025 — NOW',
    role: 'ML ENGINEER',
    org: 'WEEL.IO',
    detail: 'Production ML systems — model development through deployment.',
    highlight: false,
  },
  {
    period: '2025',
    role: 'RESEARCH — KDD LAB',
    org: 'VOCALINK PROJECT',
    detail: 'Google APAC Top 10, 2025.',
    highlight: true,
  },
  {
    period: '— 2025',
    role: 'ML ENGINEER',
    org: 'TALENTBRIDGE',
    detail: 'Machine learning engineering across client workloads.',
    highlight: false,
  },
  {
    period: 'EDUCATION',
    role: 'BDS — DATA SCIENCE',
    org: 'FAST NUCES',
    detail: 'Bachelor of Data Science.',
    highlight: false,
  },
]

export default function About() {
  const ref = useReveal<HTMLDivElement>()

  return (
    <div ref={ref} className="max-w-6xl mx-auto px-5 pt-28 pb-32">
      <ChapterMarker chapter="CH.02" title="OPERATOR FILE" />

      <div className="mt-10 grid md:grid-cols-[1.2fr_0.8fr] gap-10">
        <div className="reveal">
          <h1 className="font-display text-5xl md:text-6xl leading-[0.85] chroma">ABOUT</h1>
          <div className="mt-6 space-y-4 text-base md:text-lg leading-relaxed text-[var(--ink)]/90">
            <p>
              I'm Ibrahim, an ML engineer based in Pakistan. I work on the systems side of machine
              learning — training loops, kernels, evaluation harnesses — and on the research side,
              where my current obsessions are generative 3D models, mixture-of-experts
              interpretability, and game-theoretic evaluation of sequence models.
            </p>
            <p>
              My research exists as pre-prints on GitHub rather than in venues: the hypotheses are
              validated at small scale, and the limiting factor is compute access, not ideas. That
              constraint shapes how I work — small experiments designed to be maximally informative,
              code written to scale the moment hardware appears.
            </p>
            <p>
              Living with climate-stressed electricity infrastructure — heatwaves, floods, load
              shedding — is part of why I care about efficient, resilient ML systems that do more
              with less.
            </p>
          </div>
        </div>

        {/* skills menu */}
        <div className="reveal border-2 border-[var(--ink)] h-fit" style={{ background: 'var(--paper-2)' }}>
          <div className="border-b-2 border-[var(--ink)] px-4 py-1.5 font-display text-base tracking-[0.25em] text-[var(--ink-dim)] flex justify-between">
            <span>SKILL.MENU</span>
            <span style={{ color: 'var(--blue)' }}>7 LOADED</span>
          </div>
          <ul>
            {skills.map((s, i) => (
              <li
                key={s}
                className="flex items-baseline justify-between gap-4 px-4 py-2.5 border-b border-[var(--ink)]/10 last:border-b-0
                           font-display text-lg tracking-[0.1em] cursor-default
                           transition-all duration-150 hover:translate-x-1 hover:text-[var(--blue)]"
              >
                <span>{s}</span>
                <span className="text-[var(--ink-dim)]">00:{String(i + 1).padStart(2, '0')}</span>
              </li>
            ))}
          </ul>
        </div>
      </div>

      {/* timeline */}
      <div className="mt-16">
        <ChapterMarker chapter="CH.02.1" title="EXPERIENCE LOG" />
        <div className="mt-8 ml-2 border-l-2 border-[var(--ink)]/25">
          {timeline.map((t) => (
            <div
              key={t.org}
              className="reveal relative pl-8 pb-10 last:pb-0 transition-transform duration-150 hover:translate-x-1"
            >
              <span
                className="absolute -left-[7px] top-1.5 w-3 h-3"
                style={{
                  background: t.highlight ? 'var(--yellow)' : 'var(--red)',
                  boxShadow: t.highlight
                    ? '0 0 8px rgba(249,227,2,0.7)'
                    : '0 0 6px rgba(250,37,0,0.4)',
                  border: '2px solid var(--ink)',
                }}
              />
              <p className="font-display text-base tracking-[0.25em] text-[var(--ink-dim)]">{t.period}</p>
              <p className="font-display text-2xl md:text-3xl mt-1">
                {t.role} <span style={{ color: 'var(--blue)' }}>@ {t.org}</span>
              </p>
              <p className="mt-1 text-[var(--ink)]/80">{t.detail}</p>
            </div>
          ))}
        </div>
      </div>
    </div>
  )
}
