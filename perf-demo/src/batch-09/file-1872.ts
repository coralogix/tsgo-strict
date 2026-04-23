import type { Type1871 } from '../batch-09/file-1871.js';
export interface Type1872 {
  id: 1872;
  name: 'File1872';
  next: Type1871;
}

export function make1872(): Type1872 {
  return { id: 1872, name: 'File1872', next: null as unknown as Type1871 };
}
