import type { Type3311 } from '../batch-16/file-3311.js';
export interface Type3312 {
  id: 3312;
  name: 'File3312';
  next: Type3311;
}

export function make3312(): Type3312 {
  return { id: 3312, name: 'File3312', next: null as unknown as Type3311 };
}
