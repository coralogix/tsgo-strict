import type { Type1400 } from '../batch-07/file-1400.js';
export interface Type1401 {
  id: 1401;
  name: 'File1401';
  next: Type1400;
}

export function make1401(): Type1401 {
  return { id: 1401, name: 'File1401', next: null as unknown as Type1400 };
}
