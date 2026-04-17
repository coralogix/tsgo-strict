import type { Type1608 } from '../batch-08/file-1608.js';
export interface Type1609 {
  id: 1609;
  name: 'File1609';
  next: Type1608;
}

export function make1609(): Type1609 {
  return { id: 1609, name: 'File1609', next: null as unknown as Type1608 };
}
