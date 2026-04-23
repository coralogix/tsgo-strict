import type { Type1208 } from '../batch-06/file-1208.js';
export interface Type1209 {
  id: 1209;
  name: 'File1209';
  next: Type1208;
}

export function make1209(): Type1209 {
  return { id: 1209, name: 'File1209', next: null as unknown as Type1208 };
}
