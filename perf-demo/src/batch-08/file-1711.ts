import type { Type1710 } from '../batch-08/file-1710.js';
export interface Type1711 {
  id: 1711;
  name: 'File1711';
  next: Type1710;
}

export function make1711(): Type1711 {
  return { id: 1711, name: 'File1711', next: null as unknown as Type1710 };
}
