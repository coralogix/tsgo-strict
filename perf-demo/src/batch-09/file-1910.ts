import type { Type1909 } from '../batch-09/file-1909.js';
export interface Type1910 {
  id: 1910;
  name: 'File1910';
  next: Type1909;
}

export function make1910(): Type1910 {
  return { id: 1910, name: 'File1910', next: null as unknown as Type1909 };
}
