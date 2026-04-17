import type { Type1604 } from '../batch-08/file-1604.js';
export interface Type1605 {
  id: 1605;
  name: 'File1605';
  next: Type1604;
}

export function make1605(): Type1605 {
  return { id: 1605, name: 'File1605', next: null as unknown as Type1604 };
}
