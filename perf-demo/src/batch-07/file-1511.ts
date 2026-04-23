import type { Type1510 } from '../batch-07/file-1510.js';
export interface Type1511 {
  id: 1511;
  name: 'File1511';
  next: Type1510;
}

export function make1511(): Type1511 {
  return { id: 1511, name: 'File1511', next: null as unknown as Type1510 };
}
