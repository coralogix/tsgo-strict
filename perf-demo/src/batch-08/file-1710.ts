import type { Type1709 } from '../batch-08/file-1709.js';
export interface Type1710 {
  id: 1710;
  name: 'File1710';
  next: Type1709;
}

export function make1710(): Type1710 {
  return { id: 1710, name: 'File1710', next: null as unknown as Type1709 };
}
