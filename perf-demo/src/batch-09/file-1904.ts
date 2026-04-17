import type { Type1903 } from '../batch-09/file-1903.js';
export interface Type1904 {
  id: 1904;
  name: 'File1904';
  next: Type1903;
}

export function make1904(): Type1904 {
  return { id: 1904, name: 'File1904', next: null as unknown as Type1903 };
}
