import type { Type999 } from '../batch-04/file-0999.js';
export interface Type1000 {
  id: 1000;
  name: 'File1000';
  next: Type999;
}

export function make1000(): Type1000 {
  return { id: 1000, name: 'File1000', next: null as unknown as Type999 };
}
