import type { Type1990 } from '../batch-09/file-1990.js';
export interface Type1991 {
  id: 1991;
  name: 'File1991';
  next: Type1990;
}

export function make1991(): Type1991 {
  return { id: 1991, name: 'File1991', next: null as unknown as Type1990 };
}
