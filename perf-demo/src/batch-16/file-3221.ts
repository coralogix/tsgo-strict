import type { Type3220 } from '../batch-16/file-3220.js';
export interface Type3221 {
  id: 3221;
  name: 'File3221';
  next: Type3220;
}

export function make3221(): Type3221 {
  return { id: 3221, name: 'File3221', next: null as unknown as Type3220 };
}
