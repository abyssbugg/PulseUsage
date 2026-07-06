export const APP_DISPLAY_NAME = "PulseUsage"
export const INTERNAL_PROJECT_NAME = "pulseusage"
export const MARKETING_NAME = "PulseUsage"

export const GITHUB_OWNER = "abyssbugg"
export const GITHUB_REPO = "PulseUsage"
export const GITHUB_REPOSITORY = `${GITHUB_OWNER}/${GITHUB_REPO}`
export const GITHUB_REPOSITORY_URL = `https://github.com/${GITHUB_REPOSITORY}`

export const ISSUES_URL = `${GITHUB_REPOSITORY_URL}/issues`
export const RELEASES_URL = `${GITHUB_REPOSITORY_URL}/releases`
export const DOCUMENTATION_URL = `${GITHUB_REPOSITORY_URL}#readme`
export const SUPPORT_URL = ISSUES_URL

export function buildReleaseApiUrl(tag: string) {
  return `https://api.github.com/repos/${GITHUB_REPOSITORY}/releases/tags/${tag}`
}

export function buildPullRequestUrl(number: number) {
  return `${GITHUB_REPOSITORY_URL}/pull/${number}`
}

export function buildCommitUrl(hash: string) {
  return `${GITHUB_REPOSITORY_URL}/commit/${hash}`
}
