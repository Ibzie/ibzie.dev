import { ChapterMarker } from '../components/FX'
import { useReveal } from '../hooks/useReveal'

const skills = [
  'DEEP LEARNING ARCHITECTURE',
  'MECHANISTIC INTERPRETABILITY',
  'MIXTURE-OF-EXPERTS SYSTEMS',
  'MULTI-MODAL / AGENTIC PIPELINES',
  'PRODUCTION ML (GCP / AWS / BARE METAL)',
  'PYTORCH / LIBTORCH / UNSLOTH',
  'PYTHON / C++ / LUA',
]

const timeline = [
  {
    period: 'DEC 2025 — PRESENT',
    role: 'ML ENGINEER',
    org: 'WEEL.IO',
    detail:
      'Architected autonomous multi-modal agentic pipelines serving 100k+ requests/week across 7 enterprise organizations, and engineered end-to-end PII anonymization for zero-trust compliance.',
    highlight: false,
  },
  {
    period: 'JUN 2025 — DEC 2025',
    role: 'ML ENGINEER',
    org: 'TALENTBRIDGE FI & NESTE',
    detail:
      'Built cloud-native automation pipelines on GCP (+15% target KPIs) and high-concurrency on-premise Voice AI, cutting operational costs 65% with multilingual conversational agents.',
    highlight: false,
  },
  {
    period: 'APR 2024 — JUN 2025',
    role: 'SOFTWARE DEVELOPER — VOCALINK',
    org: 'KDD LAB',
    detail:
      'Co-founded a voice-driven document editor for accessibility; architected a low-latency MoE routing system and hit 40% user preference in a 65-participant blind evaluation.',
    highlight: false,
  },
  {
    period: 'JUN 2025',
    role: 'AWARD — TOP 10 FINALIST',
    org: 'GOOGLE APAC SOLUTION CHALLENGE',
    detail:
      'Top 10 APAC finalist (out of 3,300+ entries) for Vocalink, an AI voice-driven editor for neurodivergent users; presented at the APAC Digital Transformation Forum 2025.',
    highlight: true,
  },
  {
    period: 'JUN 2024 — AUG 2024',
    role: 'DATA SCIENCE INTERN',
    org: 'RAYN',
    detail:
      'Engineered context-retrieval pipelines indexing 50GB of documents and a multi-vector retriever that cut RAG hallucinations ~70%, plus local AI infrastructure for full data privacy.',
    highlight: false,
  },
  {
    period: 'AUG 2021 — JUL 2025',
    role: 'BSC — DATA SCIENCE',
    org: 'FAST NUCES',
    detail: 'Bachelor of Data Science. 2 Dean’s List awards.',
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
              learning (training loops, kernels, evaluation harnesses) and on the research side,
              where my current obsessions are generative 3D models, mixture-of-experts
              interpretability, and game-theoretic evaluation of sequence models.
            </p>
            <p>
              My research exists as pre-prints on GitHub rather than in venues: the hypotheses are
              validated at small scale, and the limiting factor is compute access, not ideas. That
              constraint shapes how I work: small experiments designed to be maximally informative,
              code written to scale the moment hardware appears.
            </p>
            <p>
              Living with climate-stressed electricity infrastructure (heatwaves, floods, load
              shedding) is part of why I care about efficient, resilient ML systems that do more
              with less.
            </p>
          </div>
        </div>

        {/* skills menu */}
        <div className="reveal border-2 border-[var(--ink)] h-fit" style={{ background: 'var(--paper-2)' }}>
          <div className="border-b-2 border-[var(--ink)] px-4 py-1.5 font-display text-base tracking-[0.25em] text-[var(--ink-dim)] flex justify-between">
            <span>SKILL.MENU</span>
            <span style={{ color: 'var(--blue)' }}>{skills.length} LOADED</span>
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
