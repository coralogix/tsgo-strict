import type { Type1300 } from '../batch-06/file-1300.js';
export interface Type1301 {
  id: 1301;
  name: 'File1301';
  next: Type1300;
}

export function make1301(): Type1301 {
  return { id: 1301, name: 'File1301', next: null as unknown as Type1300 };
}
