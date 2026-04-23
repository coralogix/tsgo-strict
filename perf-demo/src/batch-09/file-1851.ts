import type { Type1850 } from '../batch-09/file-1850.js';
export interface Type1851 {
  id: 1851;
  name: 'File1851';
  next: Type1850;
}

export function make1851(): Type1851 {
  return { id: 1851, name: 'File1851', next: null as unknown as Type1850 };
}
