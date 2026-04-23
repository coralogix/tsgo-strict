import type { Type1975 } from '../batch-09/file-1975.js';
export interface Type1976 {
  id: 1976;
  name: 'File1976';
  next: Type1975;
}

export function make1976(): Type1976 {
  return { id: 1976, name: 'File1976', next: null as unknown as Type1975 };
}
