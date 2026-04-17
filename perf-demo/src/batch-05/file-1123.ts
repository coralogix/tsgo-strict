import type { Type1122 } from '../batch-05/file-1122.js';
export interface Type1123 {
  id: 1123;
  name: 'File1123';
  next: Type1122;
}

export function make1123(): Type1123 {
  return { id: 1123, name: 'File1123', next: null as unknown as Type1122 };
}
