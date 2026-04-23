// Uses globals from both typeRoots — should not produce TS2304/TS2688.
// Has implicit-any to trigger a strict error.
export function check(x) {
  return BUILD_FLAG ? ENV_NAME : x;
}
