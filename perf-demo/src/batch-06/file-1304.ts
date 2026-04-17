import type { Type1303 } from '../batch-06/file-1303.js';
export interface Type1304 {
  id: 1304;
  name: 'File1304';
  next: Type1303;
}

export function make1304(): Type1304 {
  return { id: 1304, name: 'File1304', next: null as unknown as Type1303 };
}
