export type PageId = 'home' | 'projects' | 'about' | 'contact'

const NAV: { id: PageId; label: string; code: string }[] = [
  { id: 'home', label: 'HOME', code: '00' },
  { id: 'projects', label: 'PROJECTS', code: '01' },
  { id: 'about', label: 'ABOUT', code: '02' },
  { id: 'contact', label: 'CONTACT', code: '03' },
]

export default function Header({
  page,
  onNavigate,
}: {
  page: PageId
  onNavigate: (p: PageId) => void
}) {
  return (
    <header
      className="fixed top-0 left-0 right-0 z-[94] border-b border-[var(--ink)]/15"
      style={{ background: 'rgba(247,243,238,0.82)', backdropFilter: 'blur(6px)' }}
    >
      <div className="max-w-6xl mx-auto px-5 h-14 flex items-center justify-between">
        <button
          onClick={() => onNavigate('home')}
          className="font-display text-2xl tracking-wider chroma-sm cursor-pointer"
        >
          Ibzie<span style={{ color: 'var(--red)' }}>.dev</span>
        </button>
        <nav className="flex items-center gap-1 md:gap-2">
          {NAV.map((item) => {
            const active = page === item.id
            return (
              <button
                key={item.id}
                onClick={() => onNavigate(item.id)}
                className="font-display text-base md:text-lg tracking-[0.15em] px-2 md:px-3 py-1 cursor-pointer transition-transform duration-150 hover:-translate-y-0.5"
                style={
                  active
                    ? { background: 'var(--yellow)', color: 'var(--ink)' }
                    : { color: 'var(--ink-dim)' }
                }
              >
                <span className="hidden md:inline opacity-50 mr-1">{item.code}/</span>
                {item.label}
              </button>
            )
          })}
        </nav>
      </div>
    </header>
  )
}
