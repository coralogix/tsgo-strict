import type { Type1311 } from '../batch-06/file-1311.js';
export interface Type1312 {
  id: 1312;
  name: 'File1312';
  next: Type1311;
}

export function make1312(): Type1312 {
  return { id: 1312, name: 'File1312', next: null as unknown as Type1311 };
}
