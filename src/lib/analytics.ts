
/**
 * Analytics is disabled for the independent repository baseline.
 * Keep this wrapper so call sites stay stable if analytics is reintroduced later.
 */

export function track(
  _event: string,
  _props?: Record<string, string | number>,
) {
  return
}
