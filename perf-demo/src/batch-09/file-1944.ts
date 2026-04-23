import type { Type1943 } from '../batch-09/file-1943.js';
export interface Type1944 {
  id: 1944;
  name: 'File1944';
  next: Type1943;
}

export function make1944(): Type1944 {
  return { id: 1944, name: 'File1944', next: null as unknown as Type1943 };
}
