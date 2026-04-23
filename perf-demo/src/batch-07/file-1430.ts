import type { Type1429 } from '../batch-07/file-1429.js';
export interface Type1430 {
  id: 1430;
  name: 'File1430';
  next: Type1429;
}

export function make1430(): Type1430 {
  return { id: 1430, name: 'File1430', next: null as unknown as Type1429 };
}
