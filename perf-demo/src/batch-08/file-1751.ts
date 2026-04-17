import type { Type1750 } from '../batch-08/file-1750.js';
export interface Type1751 {
  id: 1751;
  name: 'File1751';
  next: Type1750;
}

export function make1751(): Type1751 {
  return { id: 1751, name: 'File1751', next: null as unknown as Type1750 };
}
