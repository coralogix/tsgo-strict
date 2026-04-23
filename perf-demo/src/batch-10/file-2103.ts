import type { Type2102 } from '../batch-10/file-2102.js';
export interface Type2103 {
  id: 2103;
  name: 'File2103';
  next: Type2102;
}

export function make2103(): Type2103 {
  return { id: 2103, name: 'File2103', next: null as unknown as Type2102 };
}
