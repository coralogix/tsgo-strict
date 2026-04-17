import type { Type1612 } from '../batch-08/file-1612.js';
export interface Type1613 {
  id: 1613;
  name: 'File1613';
  next: Type1612;
}

export function make1613(): Type1613 {
  return { id: 1613, name: 'File1613', next: null as unknown as Type1612 };
}
