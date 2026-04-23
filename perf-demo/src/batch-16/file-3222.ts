import type { Type3221 } from '../batch-16/file-3221.js';
export interface Type3222 {
  id: 3222;
  name: 'File3222';
  next: Type3221;
}

export function make3222(): Type3222 {
  return { id: 3222, name: 'File3222', next: null as unknown as Type3221 };
}
