import type { Type12 } from '../batch-00/file-0012.js';
export interface Type13 {
  id: 13;
  name: 'File13';
  next: Type12;
}

export function make13(): Type13 {
  return { id: 13, name: 'File13', next: null as unknown as Type12 };
}
