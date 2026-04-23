import type { Type1899 } from '../batch-09/file-1899.js';
export interface Type1900 {
  id: 1900;
  name: 'File1900';
  next: Type1899;
}

export function make1900(): Type1900 {
  return { id: 1900, name: 'File1900', next: null as unknown as Type1899 };
}
