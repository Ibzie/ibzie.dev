import { useState } from 'react'

/**
 * Click-to-load YouTube facade. Renders only a lightweight thumbnail until the
 * user clicks, at which point the real iframe (with autoplay) is swapped in.
 * This avoids loading YouTube's player on every project open.
 */
export default function YouTubeEmbed({
  videoId,
  title,
}: {
  videoId: string
  title?: string
}) {
  const [playing, setPlaying] = useState(false)

  if (playing) {
    return (
      <iframe
        className="absolute inset-0 w-full h-full"
        src={`https://www.youtube-nocookie.com/embed/${videoId}?autoplay=1&rel=0`}
        title={title ?? 'Demo reel'}
        allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
        allowFullScreen
      />
    )
  }

  return (
    <button
      type="button"
      onClick={() => setPlaying(true)}
      aria-label={title ? `Play video: ${title}` : 'Play video'}
      className="absolute inset-0 w-full h-full cursor-pointer group"
      style={{ background: 'var(--paper-3)' }}
    >
      <img
        src={`https://i.ytimg.com/vi/${videoId}/hqdefault.jpg`}
        alt={title ?? 'Demo reel thumbnail'}
        loading="lazy"
        className="absolute inset-0 w-full h-full object-cover"
      />
      {/* scanlines */}
      <div
        className="absolute inset-0 pointer-events-none"
        style={{
          background:
            'repeating-linear-gradient(0deg, transparent, transparent 3px, rgba(23,18,23,0.05) 3px, rgba(23,18,23,0.05) 6px)',
        }}
      />
      <span
        className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 font-display text-5xl md:text-7xl chroma transition-transform duration-150 group-hover:scale-110"
        style={{ color: 'var(--paper)' , textShadow: '2px 2px 0 var(--ink)' }}
      >
        ▸
      </span>
    </button>
  )
}
