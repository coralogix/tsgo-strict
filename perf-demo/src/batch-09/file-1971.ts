import type { Type1970 } from '../batch-09/file-1970.js';
export interface Type1971 {
  id: 1971;
  name: 'File1971';
  next: Type1970;
}

export function make1971(): Type1971 {
  return { id: 1971, name: 'File1971', next: null as unknown as Type1970 };
}
