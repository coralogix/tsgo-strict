import type { Type1961 } from '../batch-09/file-1961.js';
export interface Type1962 {
  id: 1962;
  name: 'File1962';
  next: Type1961;
}

export function make1962(): Type1962 {
  return { id: 1962, name: 'File1962', next: null as unknown as Type1961 };
}
