export function headerTimestamp(header: any): number {
  return Math.floor(header.content.timestamp / 1000);
}

export const sleep = (ms: number) =>
  new Promise(r => setTimeout(() => r(null), ms));
