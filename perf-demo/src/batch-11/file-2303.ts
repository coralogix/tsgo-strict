import type { Type2302 } from '../batch-11/file-2302.js';
export interface Type2303 {
  id: 2303;
  name: 'File2303';
  next: Type2302;
}

export function make2303(): Type2303 {
  return { id: 2303, name: 'File2303', next: null as unknown as Type2302 };
}
