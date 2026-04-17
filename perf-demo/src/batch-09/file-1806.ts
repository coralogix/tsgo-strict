import type { Type1805 } from '../batch-09/file-1805.js';
export interface Type1806 {
  id: 1806;
  name: 'File1806';
  next: Type1805;
}

export function make1806(): Type1806 {
  return { id: 1806, name: 'File1806', next: null as unknown as Type1805 };
}
