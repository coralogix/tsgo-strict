import type { Type1960 } from '../batch-09/file-1960.js';
export interface Type1961 {
  id: 1961;
  name: 'File1961';
  next: Type1960;
}

export function make1961(): Type1961 {
  return { id: 1961, name: 'File1961', next: null as unknown as Type1960 };
}
