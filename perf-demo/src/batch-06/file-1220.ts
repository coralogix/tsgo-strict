import type { Type1219 } from '../batch-06/file-1219.js';
export interface Type1220 {
  id: 1220;
  name: 'File1220';
  next: Type1219;
}

export function make1220(): Type1220 {
  return { id: 1220, name: 'File1220', next: null as unknown as Type1219 };
}
