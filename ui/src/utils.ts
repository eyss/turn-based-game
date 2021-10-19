export function headerTimestamp(header: any): number {
  return Math.floor(header.content.timestamp / 1000);
}
