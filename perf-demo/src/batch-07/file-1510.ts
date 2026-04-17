import type { Type1509 } from '../batch-07/file-1509.js';
export interface Type1510 {
  id: 1510;
  name: 'File1510';
  next: Type1509;
}

export function make1510(): Type1510 {
  return { id: 1510, name: 'File1510', next: null as unknown as Type1509 };
}
