import type { Type1989 } from '../batch-09/file-1989.js';
export interface Type1990 {
  id: 1990;
  name: 'File1990';
  next: Type1989;
}

export function make1990(): Type1990 {
  return { id: 1990, name: 'File1990', next: null as unknown as Type1989 };
}
