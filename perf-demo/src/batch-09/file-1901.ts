import type { Type1900 } from '../batch-09/file-1900.js';
export interface Type1901 {
  id: 1901;
  name: 'File1901';
  next: Type1900;
}

export function make1901(): Type1901 {
  return { id: 1901, name: 'File1901', next: null as unknown as Type1900 };
}
