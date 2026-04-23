import type { Type1111 } from '../batch-05/file-1111.js';
export interface Type1112 {
  id: 1112;
  name: 'File1112';
  next: Type1111;
}

export function make1112(): Type1112 {
  return { id: 1112, name: 'File1112', next: null as unknown as Type1111 };
}
