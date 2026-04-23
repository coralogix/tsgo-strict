import type { Type1942 } from '../batch-09/file-1942.js';
export interface Type1943 {
  id: 1943;
  name: 'File1943';
  next: Type1942;
}

export function make1943(): Type1943 {
  return { id: 1943, name: 'File1943', next: null as unknown as Type1942 };
}
