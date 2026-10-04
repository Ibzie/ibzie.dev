import { useEffect, useState } from 'react'

/** Full-screen CRT/VHS overlay stack: scanlines, noise, vignette, glitch bands. */
export function FXOverlay() {
  return (
    <>
      <div className="fx-layer fx-vignette" />
      <div className="fx-layer fx-scanlines" />
      <div className="fx-layer fx-noise" />
      <div className="glitch-band glitch-band--1" style={{ top: '18%' }} />
      <div className="glitch-band glitch-band--2" style={{ top: '52%' }} />
      <div className="glitch-band glitch-band--3" style={{ top: '76%' }} />
    </>
  )
}

function formatTime(s: number) {
  const h = String(Math.floor(s / 3600)).padStart(2, '0')
  const m = String(Math.floor((s % 3600) / 60)).padStart(2, '0')
  const sec = String(s % 60).padStart(2, '0')
  return `${h}:${m}:${sec}`
}

/** Corner HUD: live playback timer (bottom-right) + tracking indicator (bottom-left). */
export function HUD() {
  const [seconds, setSeconds] = useState(0)
  const [playing, setPlaying] = useState(true)

  useEffect(() => {
    if (!playing) return
    const t = setInterval(() => setSeconds((s) => s + 1), 1000)
    return () => clearInterval(t)
  }, [playing])

  return (
    <>
      <button
        onClick={() => setPlaying((p) => !p)}
        className="hidden sm:block fixed bottom-4 right-5 z-[95] font-display text-lg tracking-widest cursor-pointer select-none"
        style={{ color: playing ? 'var(--green)' : 'var(--red)' }}
        aria-label="Toggle playback timer"
      >
        {playing ? '▸ PLAY' : '❚❚ PAUSE'} {formatTime(seconds)}
      </button>
      <div className="hidden sm:flex fixed bottom-4 left-5 z-[95] font-display text-lg tracking-widest text-[var(--ink-dim)] items-center gap-2">
        <span
          className="inline-block w-2 h-2"
          style={{ background: 'var(--green)', boxShadow: '0 0 6px rgba(44,122,63,0.6)' }}
        />
        TRACKING
      </div>
    </>
  )
}

/** Chapter divider with decorative rules, e.g. "CH.01 ▸ PROJECT INDEX". */
export function ChapterMarker({ chapter, title }: { chapter: string; title: string }) {
  return (
    <div className="flex items-center gap-2 sm:gap-4 reveal">
      <span className="h-px flex-1 bg-[var(--ink)] opacity-20" />
      <span className="font-display text-base sm:text-xl md:text-2xl tracking-[0.12em] sm:tracking-[0.2em] text-[var(--ink)] whitespace-nowrap">
        {chapter} <span style={{ color: 'var(--red)' }}>▸</span> {title}
      </span>
      <span className="h-px flex-1 bg-[var(--ink)] opacity-20" />
    </div>
  )
}
