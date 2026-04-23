import type { Type1250 } from '../batch-06/file-1250.js';
export interface Type1251 {
  id: 1251;
  name: 'File1251';
  next: Type1250;
}

export function make1251(): Type1251 {
  return { id: 1251, name: 'File1251', next: null as unknown as Type1250 };
}
