import type { Type1241 } from '../batch-06/file-1241.js';
export interface Type1242 {
  id: 1242;
  name: 'File1242';
  next: Type1241;
}

export function make1242(): Type1242 {
  return { id: 1242, name: 'File1242', next: null as unknown as Type1241 };
}
