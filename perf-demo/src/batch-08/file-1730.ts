import type { Type1729 } from '../batch-08/file-1729.js';
export interface Type1730 {
  id: 1730;
  name: 'File1730';
  next: Type1729;
}

export function make1730(): Type1730 {
  return { id: 1730, name: 'File1730', next: null as unknown as Type1729 };
}
