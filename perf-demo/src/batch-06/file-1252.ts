import type { Type1251 } from '../batch-06/file-1251.js';
export interface Type1252 {
  id: 1252;
  name: 'File1252';
  next: Type1251;
}

export function make1252(): Type1252 {
  return { id: 1252, name: 'File1252', next: null as unknown as Type1251 };
}
