import type { Type2060 } from '../batch-10/file-2060.js';
export interface Type2061 {
  id: 2061;
  name: 'File2061';
  next: Type2060;
}

export function make2061(): Type2061 {
  return { id: 2061, name: 'File2061', next: null as unknown as Type2060 };
}
