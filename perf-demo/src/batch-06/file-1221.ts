import type { Type1220 } from '../batch-06/file-1220.js';
export interface Type1221 {
  id: 1221;
  name: 'File1221';
  next: Type1220;
}

export function make1221(): Type1221 {
  return { id: 1221, name: 'File1221', next: null as unknown as Type1220 };
}
