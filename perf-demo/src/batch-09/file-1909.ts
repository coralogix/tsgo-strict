import type { Type1908 } from '../batch-09/file-1908.js';
export interface Type1909 {
  id: 1909;
  name: 'File1909';
  next: Type1908;
}

export function make1909(): Type1909 {
  return { id: 1909, name: 'File1909', next: null as unknown as Type1908 };
}
