import type { Type1701 } from '../batch-08/file-1701.js';
export interface Type1702 {
  id: 1702;
  name: 'File1702';
  next: Type1701;
}

export function make1702(): Type1702 {
  return { id: 1702, name: 'File1702', next: null as unknown as Type1701 };
}
