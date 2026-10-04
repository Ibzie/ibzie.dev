import { profile } from '../data/projects'
import type { PageId } from '../components/Header'

const MARQUEE = [
  'GENERATIVE MODELS', 'MECHANISTIC INTERPRETABILITY', 'GAME THEORY',
  'PYTORCH', 'CUDA', 'DISTRIBUTED TRAINING', 'VAE', 'MIXTURE-OF-EXPERTS',
]

export default function Home({ onNavigate }: { onNavigate: (p: PageId) => void }) {
  return (
    <div className="min-h-screen flex flex-col pt-14">
      <div className="flex-1 max-w-6xl mx-auto px-5 w-full flex flex-col justify-center py-16">
        <p className="vhs-in font-display text-xl md:text-2xl tracking-[0.3em] text-[var(--ink-dim)]" style={{ animationDelay: '0.3s' }}>
          SIGNAL ACQUIRED — {profile.role}
        </p>

        <h1
          className="vhs-in font-display uppercase leading-[0.8] chroma mt-4"
          style={{ fontSize: 'clamp(4.5rem, 16vw, 13rem)', animationDelay: '0.45s' }}
        >
          {profile.name}
        </h1>

        <p className="fade-in mt-6 max-w-2xl text-lg md:text-xl leading-relaxed text-[var(--ink)]/85" style={{ animationDelay: '0.7s' }}>
          I build and dissect machine learning systems — generative 3D models, mixture-of-experts
          interpretability, and game-theoretic evaluation harnesses. Currently an ML engineer at{' '}
          <span className="font-semibold" style={{ color: 'var(--blue)' }}>Weel.io</span>; research
          lives as pre-prints on GitHub.
        </p>

        <div className="fade-in mt-10 flex flex-wrap gap-4" style={{ animationDelay: '0.9s' }}>
          <button onClick={() => onNavigate('projects')} className="btn-primary cursor-pointer text-xl">
            ▸ VIEW PROJECT INDEX
          </button>
          <button onClick={() => onNavigate('contact')} className="btn-ghost cursor-pointer text-xl">
            OPEN COMMS
          </button>
        </div>

        {/* status terminal */}
        <div
          className="fade-in mt-12 max-w-xl border-2 border-[var(--ink)] hard-shadow-red"
          style={{ background: 'var(--paper-2)', animationDelay: '1.1s' }}
        >
          <div className="border-b-2 border-[var(--ink)] px-4 py-1.5 font-display text-base tracking-[0.25em] text-[var(--ink-dim)]">
            SYS.STATUS
          </div>
          <div className="px-4 py-3 font-display text-lg md:text-xl tracking-[0.1em] space-y-1">
            <div className="flex justify-between gap-6"><span className="text-[var(--ink-dim)]">ROLE</span><span>{profile.role} @ WEEL.IO</span></div>
            <div className="flex justify-between gap-6"><span className="text-[var(--ink-dim)]">LOCATION</span><span>{profile.location}</span></div>
            <div className="flex justify-between gap-6"><span className="text-[var(--ink-dim)]">STATUS</span><span style={{ color: 'var(--green)' }}>● {profile.availability}</span></div>
            <div className="flex justify-between gap-6"><span className="text-[var(--ink-dim)]">COMPUTE</span><span style={{ color: 'var(--red)' }}>● BLOCKED — SEEKING ACCESS</span></div>
          </div>
        </div>
      </div>

      {/* marquee */}
      <div className="border-t-2 border-b-2 border-[var(--ink)] overflow-hidden py-2" style={{ background: 'var(--yellow)' }}>
        <div className="marquee-track flex whitespace-nowrap font-display text-xl md:text-2xl tracking-[0.2em] w-max">
          {[0, 1].map((copy) => (
            <span key={copy} className="flex">
              {MARQUEE.map((m) => (
                <span key={m} className="mx-6">{m} <span style={{ color: 'var(--red)' }}>✦</span></span>
              ))}
            </span>
          ))}
        </div>
      </div>
    </div>
  )
}
