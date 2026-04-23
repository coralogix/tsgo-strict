import type { Type1904 } from '../batch-09/file-1904.js';
export interface Type1905 {
  id: 1905;
  name: 'File1905';
  next: Type1904;
}

export function make1905(): Type1905 {
  return { id: 1905, name: 'File1905', next: null as unknown as Type1904 };
}
