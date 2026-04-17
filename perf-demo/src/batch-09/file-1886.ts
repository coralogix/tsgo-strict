import type { Type1885 } from '../batch-09/file-1885.js';
export interface Type1886 {
  id: 1886;
  name: 'File1886';
  next: Type1885;
}

export function make1886(): Type1886 {
  return { id: 1886, name: 'File1886', next: null as unknown as Type1885 };
}
