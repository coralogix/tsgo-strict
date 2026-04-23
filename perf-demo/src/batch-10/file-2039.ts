import type { Type2038 } from '../batch-10/file-2038.js';
export interface Type2039 {
  id: 2039;
  name: 'File2039';
  next: Type2038;
}

export function make2039(): Type2039 {
  return { id: 2039, name: 'File2039', next: null as unknown as Type2038 };
}
