import type { Type1941 } from '../batch-09/file-1941.js';
export interface Type1942 {
  id: 1942;
  name: 'File1942';
  next: Type1941;
}

export function make1942(): Type1942 {
  return { id: 1942, name: 'File1942', next: null as unknown as Type1941 };
}
