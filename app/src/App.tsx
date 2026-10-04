import { useCallback, useState } from 'react'
import Header, { type PageId } from './components/Header'
import { FXOverlay, HUD } from './components/FX'
import Home from './pages/Home'
import Projects from './pages/Projects'
import About from './pages/About'
import Contact from './pages/Contact'

export default function App() {
  // Landing page is Home; Projects remains the project index.
  const [page, setPage] = useState<PageId>('home')
  const [wiping, setWiping] = useState(false)

  const navigate = useCallback(
    (next: PageId) => {
      if (next === page || wiping) return
      setWiping(true)
      window.setTimeout(() => {
        setPage(next)
        window.scrollTo(0, 0)
      }, 260)
      window.setTimeout(() => setWiping(false), 580)
    },
    [page, wiping]
  )

  return (
    <div className="min-h-screen flex flex-col">
      <FXOverlay />
      <HUD />
      <Header page={page} onNavigate={navigate} />

      <main className="flex-1">
        {page === 'home' && <Home onNavigate={navigate} />}
        {page === 'projects' && <Projects />}
        {page === 'about' && <About />}
        {page === 'contact' && <Contact />}
      </main>

      {/* transition wipe */}
      {wiping && (
        <div
          className="wipe fixed inset-0 z-[97] flex items-center justify-center"
          style={{ background: 'var(--ink)' }}
        >
          <span className="font-display text-2xl md:text-4xl tracking-[0.3em] chroma" style={{ color: 'var(--paper)' }}>
            LOADING...
          </span>
        </div>
      )}

      <footer className="border-t border-[var(--ink)]/15">
        <div className="max-w-6xl mx-auto px-5 pt-6 pb-16 sm:pb-14 flex flex-wrap items-center justify-between gap-3">
          <span className="font-display text-lg tracking-[0.2em] text-[var(--ink-dim)]">
            © 2026 IBZIE.DEV · SIGNAL ENDS
          </span>
          <button
            onClick={() => window.scrollTo({ top: 0, behavior: 'smooth' })}
            className="font-display text-lg tracking-[0.2em] cursor-pointer transition-all duration-150 hover:-translate-y-0.5"
            style={{ color: 'var(--red)' }}
          >
            ◂◂ REWIND
          </button>
        </div>
      </footer>
    </div>
  )
}
