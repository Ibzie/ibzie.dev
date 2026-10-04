import { useEffect, useRef } from 'react'

/** Adds .visible to elements with .reveal when they enter the viewport. */
export function useReveal<T extends HTMLElement>() {
  const ref = useRef<T>(null)

  useEffect(() => {
    const root = ref.current
    if (!root) return
    const els = root.querySelectorAll('.reveal')

    // Immediately reveal anything already within (or above) the viewport so
    // content is never permanently hidden, even if the observer never fires.
    const revealNow = () => {
      els.forEach((el) => {
        const top = el.getBoundingClientRect().top
        if (top < window.innerHeight) el.classList.add('visible')
      })
    }

    const io = new IntersectionObserver(
      (entries) => {
        entries.forEach((e) => {
          if (e.isIntersecting) {
            e.target.classList.add('visible')
            io.unobserve(e.target)
          }
        })
      },
      { threshold: 0, rootMargin: '0px 0px -8% 0px' }
    )
    els.forEach((el) => io.observe(el))
    revealNow()
    // Re-check shortly after mount in case layout shifts.
    const t = window.setTimeout(revealNow, 350)
    return () => {
      io.disconnect()
      window.clearTimeout(t)
    }
  }, [])

  return ref
}
