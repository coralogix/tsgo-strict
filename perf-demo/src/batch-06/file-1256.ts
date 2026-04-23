import type { Type1255 } from '../batch-06/file-1255.js';
export interface Type1256 {
  id: 1256;
  name: 'File1256';
  next: Type1255;
}

export function make1256(): Type1256 {
  return { id: 1256, name: 'File1256', next: null as unknown as Type1255 };
}
