import type { Type1913 } from '../batch-09/file-1913.js';
export interface Type1914 {
  id: 1914;
  name: 'File1914';
  next: Type1913;
}

export function make1914(): Type1914 {
  return { id: 1914, name: 'File1914', next: null as unknown as Type1913 };
}
