import type { Type1820 } from '../batch-09/file-1820.js';
export interface Type1821 {
  id: 1821;
  name: 'File1821';
  next: Type1820;
}

export function make1821(): Type1821 {
  return { id: 1821, name: 'File1821', next: null as unknown as Type1820 };
}
