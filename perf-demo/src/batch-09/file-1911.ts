import type { Type1910 } from '../batch-09/file-1910.js';
export interface Type1911 {
  id: 1911;
  name: 'File1911';
  next: Type1910;
}

export function make1911(): Type1911 {
  return { id: 1911, name: 'File1911', next: null as unknown as Type1910 };
}
