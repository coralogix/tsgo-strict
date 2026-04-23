// Uses global from default node_modules/@types — should not produce TS2304/TS2688.
// Has implicit-any to trigger a strict error.
export function check(x) {
  return ENV_NAME + x;
}
