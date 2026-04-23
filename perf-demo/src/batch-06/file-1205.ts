import type { Type1204 } from '../batch-06/file-1204.js';
export interface Type1205 {
  id: 1205;
  name: 'File1205';
  next: Type1204;
}

export function make1205(): Type1205 {
  return { id: 1205, name: 'File1205', next: null as unknown as Type1204 };
}
