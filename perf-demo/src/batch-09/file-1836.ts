import type { Type1835 } from '../batch-09/file-1835.js';
export interface Type1836 {
  id: 1836;
  name: 'File1836';
  next: Type1835;
}

export function make1836(): Type1836 {
  return { id: 1836, name: 'File1836', next: null as unknown as Type1835 };
}
