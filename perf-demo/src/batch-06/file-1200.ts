import type { Type1199 } from '../batch-05/file-1199.js';
export interface Type1200 {
  id: 1200;
  name: 'File1200';
  next: Type1199;
}

export function make1200(): Type1200 {
  return { id: 1200, name: 'File1200', next: null as unknown as Type1199 };
}
