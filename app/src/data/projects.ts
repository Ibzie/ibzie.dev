export type ProjectStatus = 'PRE-PRINT' | 'COMPUTE-BLOCKED' | 'VALIDATED'

/** Optional demo video, hosted on YouTube. */
export type ProjectVideo = {
  youtubeId: string
  title?: string
}

/** Optional paper / pre-print write-up. */
export type ProjectPaper = {
  title: string
  authors: string
  abstract: string
  href: string
}

export type ProjectLink = { label: string; href: string }

export type Project = {
  id: string
  num: string
  year: string
  title: string
  type: string
  status: ProjectStatus
  stack: string[]
  tagline: string
  description: string[]
  /** Optional — rendered only when attached. */
  video?: ProjectVideo
  /** Optional — rendered only when attached. */
  paper?: ProjectPaper
  /** Optional outbound link — live demo, hosted artifact, external write-up. */
  demo?: ProjectLink
  links: ProjectLink[]
}

export type Profile = {
  name: string
  handle: string
  role: string
  location: string
  github: string
  availability: string
}

/**
 * Project content lives as one JSON file per project in `src/content/projects/`.
 * Adding or removing a project is a matter of adding or deleting that file —
 * no code changes required. Files are collected eagerly and sorted by `num`.
 */
const projectModules = import.meta.glob<{ default: Project }>('../content/projects/*.json', {
  eager: true,
})

export const projects: Project[] = Object.values(projectModules)
  .map((mod) => mod.default)
  .sort((a, b) => a.num.localeCompare(b.num))

const profileModule = import.meta.glob<{ default: Profile }>('../content/profile.json', {
  eager: true,
})

export const profile: Profile = Object.values(profileModule)[0].default
