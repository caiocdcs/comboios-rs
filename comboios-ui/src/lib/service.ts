/**
 * Split CP's service field into its short code and name:
 * "IC|Intercidades" → { code: "IC", name: "Intercidades" }
 */
export function parseService(serviceType: string | null | undefined): { code: string; name: string } {
  const [rawCode, rawName = ''] = (serviceType ?? '').split('|', 2);
  const code = rawCode.trim() || '?';
  return { code, name: rawName.trim() || code };
}
