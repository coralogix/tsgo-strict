import type { Type1200 } from '../batch-06/file-1200.js';
export interface Type1201 {
  id: 1201;
  name: 'File1201';
  next: Type1200;
}

export function make1201(): Type1201 {
  return { id: 1201, name: 'File1201', next: null as unknown as Type1200 };
}
