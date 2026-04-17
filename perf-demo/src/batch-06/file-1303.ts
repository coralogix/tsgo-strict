import type { Type1302 } from '../batch-06/file-1302.js';
export interface Type1303 {
  id: 1303;
  name: 'File1303';
  next: Type1302;
}

export function make1303(): Type1303 {
  return { id: 1303, name: 'File1303', next: null as unknown as Type1302 };
}
