export function headerTimestamp(header) {
    return Math.floor(header.content.timestamp / 1000);
}
export const sleep = (ms) => new Promise(r => setTimeout(() => r(null), ms));
//# sourceMappingURL=utils.js.map